//
// spliter.rs
// Copyright (C) 2024 imotai <codego.me@gmail.com>
// Distributed under terms of the MIT license.
//

mod context_splitter;
pub mod entity_splitter;
mod line_spliter;

#[cfg(test)]
#[path = "./splitter/test_java.rs"]
mod test_java;
#[cfg(test)]
#[path = "./splitter/test_python.rs"]
mod test_python;
#[cfg(test)]
#[path = "./splitter/test_rust.rs"]
mod test_rust;
#[cfg(test)]
#[path = "./splitter/test_solidity.rs"]
mod test_solidity;
#[cfg(test)]
#[path = "./splitter/test_ts.rs"]
mod test_ts;

use crate::{
    lang::{
        Lang,
        LangConfig,
    },
    Chunk,
    Entity,
    EntityType,
    SplitOptions,
};
use anyhow::Result;
use std::{
    collections::{
        BTreeMap,
        HashMap,
    },
    ops::Range,
};
use tree_sitter::{
    Node,
    Parser,
    Query,
    QueryCursor,
    Tree,
};

/// Represents a code entity with associated metadata
#[derive(Debug, Clone, PartialEq)]
pub struct CodeEntity {
    /// Name of the parent entity (e.g., class name for a method)
    pub parent_name: Option<String>,
    /// Name of the entity (e.g., function name, class name)
    pub name: String,
    /// Names of interfaces or traits implemented by this entity
    pub interface_names: Vec<String>,
    /// Range of lines containing the entity's documentation comments
    pub comment_line_range: Option<Range<usize>>,
    /// Range of lines containing the entity's actual code body
    pub body_line_range: Range<usize>,
    /// Type of the entity (e.g., Class, Function, Interface, Method)
    pub entity_type: EntityType,
    /// byte range of the comment
    pub comment_byte_range: Option<Range<usize>>,
    /// byte range of the body
    pub body_byte_range: Range<usize>,
    /// line range of the parent
    pub parent_line_range: Option<Range<usize>>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CodeChunk {
    pub line_range: Range<usize>,
    /// description of the chunk
    /// entities in the chunk
    pub entities: Vec<CodeEntity>,
}

#[derive(Debug, Clone)]
pub struct EntityNode {
    /// the byte range of the node
    pub byte_range: Range<usize>,
    /// the line range of the node. the end is included
    pub line_range: Range<usize>,
}

fn parse_capture_for_entity<'a>(
    lang_config: &LangConfig,
    code: &'a str,
    tree: &'a Tree,
) -> Result<Vec<(HashMap<String, EntityNode>, Vec<Node<'a>>)>> {
    let query = Query::new(&(lang_config.grammar)(), lang_config.query)?;
    let mut query_cursor = QueryCursor::new();
    let matches = query_cursor.matches(&query, tree.root_node(), code.as_bytes());
    // only the method, function, struct, enum will be pushed to entity_captures_map
    // Note: if the method and function has the same location, only the method will be captured
    let mut entity_captures_map: BTreeMap<usize, (HashMap<String, EntityNode>, Vec<Node>)> =
        BTreeMap::new();
    for m in matches {
        let mut captures: HashMap<String, EntityNode> = HashMap::new();
        let mut parent_captures: HashMap<String, EntityNode> = HashMap::new();
        let mut nodes = vec![];
        let mut definition_start = 0;
        for c in m.captures {
            let capture_name = query.capture_names()[c.index as usize];
            // handle the parent capture. current the list of parent capture
            // 1. class.definition
            // 2. method.class.name
            // 3. method.interface.name
            if capture_name.contains("class") || capture_name.contains("interface") {
                parent_captures.insert(
                    capture_name.to_string(),
                    EntityNode {
                        byte_range: c.node.byte_range(),
                        line_range: c.node.start_position().row..c.node.end_position().row,
                    },
                );
                continue;
            }
            // handle the multi times for the same capture name
            // the line comment and block comment will be merged
            if let Some(existing_node) = captures.get_mut(capture_name) {
                existing_node.byte_range = Range {
                    start: existing_node
                        .byte_range
                        .start
                        .min(c.node.byte_range().start),
                    end: existing_node.byte_range.end.max(c.node.byte_range().end),
                };
                existing_node.line_range = Range {
                    start: existing_node
                        .line_range
                        .start
                        .min(c.node.start_position().row),
                    end: existing_node.line_range.end.max(c.node.end_position().row),
                };
            } else {
                captures.insert(
                    capture_name.to_string(),
                    EntityNode {
                        byte_range: c.node.byte_range(),
                        line_range: c.node.start_position().row..c.node.end_position().row,
                    },
                );
            }

            // handle the all the definition
            if capture_name.ends_with(".definition") {
                definition_start = c.node.byte_range().start;
            }

            // handle the name node match
            if capture_name.ends_with(".name") {
                // copy the parent capture to the captures
                parent_captures.iter().for_each(|(k, v)| {
                    captures.insert(k.clone(), v.clone());
                });
                entity_captures_map.insert(definition_start, (captures.clone(), nodes));
                // reset the captures and nodes
                captures = HashMap::new();
                nodes = vec![];
            } else {
                nodes.push(c.node);
            }
        }
    }
    Ok(entity_captures_map
        .iter()
        .map(|(_start, (captures, nodes))| (captures.clone(), nodes.clone()))
        .collect::<Vec<(HashMap<String, EntityNode>, Vec<Node>)>>())
}

/// Splits the given code into chunks based on the provided options.
///
/// # Arguments
///
/// * `filename` - The name of the file containing the code.
/// * `code` - The source code to be split.
/// * `options` - The options for splitting the code.
///
/// # Returns
///
/// A `Result` containing a vector of `Chunk`s if successful, or an error if parsing fails.
///
/// # Example
///
/// ```
/// use devgen_splitter::{
///     split,
///     SplitOptions,
/// };
///
/// let code = "fn main() { println!(\"Hello, world!\"); }";
/// let options = SplitOptions {
///     chunk_line_limit: 5,
/// };
/// let chunks = split("example.rs", code, &options).unwrap();
/// ```
pub fn split(filename: &str, code: &str, options: &SplitOptions) -> Result<Vec<Chunk>> {
    let Some(lang_config) = Lang::from_filename(filename) else {
        return Err(anyhow::anyhow!("Unsupported language"));
    };
    let lines = code.lines().collect::<Vec<&str>>();
    let mut parser = Parser::new();
    parser.set_language(&(lang_config.grammar)())?;
    let tree = parser
        .parse(code, None)
        .ok_or(anyhow::anyhow!("Failed to parse code"))?;
    if lang_config.query.is_empty() {
        return line_spliter::split_tree_node(
            &lines,
            &tree.root_node(),
            options.chunk_line_limit,
            options.chunk_line_limit / 2,
        );
    }
    let captures = parse_capture_for_entity(&lang_config, code, &tree)?;
    let entities = captures
        .iter()
        .filter_map(|(captures, nodes)| {
            match context_splitter::convert_node_to_code_entity(captures, code) {
                Ok(entity) => Some((entity, nodes.to_vec())),
                Err(e) => {
                    println!("error: {:?}", e);
                    None
                }
            }
        })
        .collect::<Vec<(CodeEntity, Vec<Node>)>>();
    let chunks = context_splitter::merge_code_entities(code, &entities, options)?;
    Ok(chunks
        .iter()
        .map(|code_chunk| {
            let entities = code_chunk
                .entities
                .iter()
                .map(|entity| {
                    let chunk_line_range = Range {
                        start: code_chunk
                            .line_range
                            .start
                            .max(entity.body_line_range.start),
                        end: code_chunk.line_range.end.min(entity.body_line_range.end),
                    };
                    Entity {
                        name: entity.name.clone(),
                        entity_type: entity.entity_type.clone(),
                        parent: entity.parent_name.clone(),
                        completed_line_range: entity.body_line_range.clone(),
                        chunk_line_range,
                        parent_line_range: entity.parent_line_range.clone(),
                    }
                })
                .collect::<Vec<Entity>>();
            let chunk = Chunk {
                line_range: code_chunk.line_range.clone(),
                entities,
            };
            chunk
        })
        .collect::<Vec<Chunk>>())
}

#[cfg(test)]
fn run_test_case(
    filename: &str,
    code: &str,
    capture_names: Vec<(usize, &str)>,
    line_ranges: Vec<Range<usize>>,
) {
    let lang_config = Lang::from_filename(filename).unwrap();
    let mut parser = Parser::new();
    parser.set_language(&(lang_config.grammar)()).unwrap();
    let tree = parser
        .parse(code, None)
        .ok_or(anyhow::anyhow!("Failed to parse code"))
        .unwrap();
    let captures = parse_capture_for_entity(&lang_config, code, &tree).unwrap();
    for (i, (index, capture_name)) in capture_names.iter().enumerate() {
        let capture = captures[*index].0.get(*capture_name).unwrap();
        let line_range = line_ranges[i].clone();
        assert_eq!(
            capture.line_range, line_range,
            "capture_name: {}",
            capture_name
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    fn test_rust_split_demo() {
        let code = r#"
fn main() { 
    println!("Hello, world!");
}

struct Test {
    a: i32,
    b: i32,
}

impl Test {
    fn test() {
        for i in 0..10 {
            println!("i: {}", i);
        }
        for i in 0..10 {
            println!("i: {}", i);
        }
        for i in 0..10 {
            println!("i: {}", i);
        }
        for i in 0..10 {
            println!("i: {}", i);
        }
        for i in 0..10 {
            println!("i: {}", i);
        }
        for i in 0..10 {
            println!("i: {}", i);
        }
        for i in 0..10 {
            println!("i: {}", i);
        }
        for i in 0..10 {
            println!("i: {}", i);
        }
        println!("Hello, world!");
    }


    fn test_rust_split_2() {
        println!("test_rust_split_2");
    }
}
"#;
        let options = SplitOptions {
            chunk_line_limit: 5,
        };
        let result = split("test.rs", code, &options);
        assert_eq!(result.is_ok(), true);
        let chunks = result.unwrap();
        for chunk in &chunks {
            println!("chunk: {:?}", chunk);
        }
    }
}
