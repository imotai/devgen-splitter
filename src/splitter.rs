//
// spliter.rs
// Copyright (C) 2024 imotai <codego.me@gmail.com>
// Distributed under terms of the MIT license.
//
mod context_splitter;
pub mod entity_splitter;
mod line_spliter;
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

const METHOD_NAME: &str = "method.name";
const METHOD_DEFINITION: &str = "method.definition";
const METHOD_COMMENT: &str = "method.comment";
const CLASS_DEFINITION: &str = "class.definition";

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
    let mut entity_captures_map: BTreeMap<usize, (HashMap<String, EntityNode>, Vec<Node>)> =
        BTreeMap::new();

    for m in matches {
        let mut captures: HashMap<String, EntityNode> = HashMap::new();
        let mut parent_capture: Option<(String, EntityNode)> = None;
        let mut nodes = vec![];
        let mut definition_start = 0;
        for c in m.captures {
            let capture_name = query.capture_names()[c.index as usize];
            if capture_name == CLASS_DEFINITION {
                // enter a new class or interface
                parent_capture = Some((
                    capture_name.to_string(),
                    EntityNode {
                        byte_range: c.node.byte_range(),
                        line_range: c.node.start_position().row..c.node.end_position().row,
                    },
                ));
                continue;
            }
            // handle the multi times for the same capture name
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

            if capture_name.ends_with(".definition") {
                definition_start = c.node.byte_range().start;
            }

            // when meet method.name, we need to push code chunk for every method.name
            if capture_name == METHOD_NAME {
                if let Some(ref parent_capture) = parent_capture {
                    captures.insert(parent_capture.0.clone(), parent_capture.1.clone());
                }
                entity_captures_map.insert(definition_start, (captures.clone(), nodes));
                let new_captures = captures.clone();
                captures = HashMap::new();
                captures.extend(new_captures);
                captures.remove(METHOD_NAME);
                captures.remove(METHOD_DEFINITION);
                captures.remove(METHOD_COMMENT);
                nodes = vec![];
                continue;
            }
            if !capture_name.ends_with(".name") {
                nodes.push(c.node);
            }
        }
        if nodes.len() > 0 {
            entity_captures_map.insert(definition_start, (captures, nodes));
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
                Err(_e) => None,
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
        println!("captures: {:?}", captures);
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
    #[rstest]
    #[case(
        r#"
fn main() { 
    println!("Hello, world!");
}
"#,
        vec![(0, "function.definition")],
        vec![1..3],
    )]
    #[case(
        r#"
pub struct Test {
    pub a: i32,
    pub b: i32,
}
"#,
        vec![(0, "struct.definition")],
        vec![1..4],
    )]
    #[case(
        r#"
pub enum Test {
    A,
    B,
}
"#,
        vec![(0, "enum.definition")],
        vec![1..4],
    )]
    #[case(
        r#"
impl Test {
    pub fn a(&self) {
        println!("Hello, world!");
    }
}
"#,
        vec![(0, "class.definition"), (0, "method.definition")],
        vec![1..5, 2..4],
    )]
    #[case(
        r#"
/// this is a test
impl Test {
    /// this is a test
    pub fn a(&self) {
        println!("Hello, world!");
    }
    
    pub fn b() {
        println!("Hello, world!");
    }
}
"#,
        vec![(0, "class.definition"), (0, "method.comment"), (0, "method.definition"), (1, "method.definition")],
        vec![2..11, 3..4, 4..6, 8..10],
    )]
    #[case(
        r#"
trait Test {
    fn a(&self);
}

trait Test2 {
    fn b(&self);
}
"#,
        vec![(0, "class.definition"),(0, "method.definition"), (1, "class.definition"), (1, "method.definition")],
        vec![1..3, 2..2, 5..7, 6..6],
    )]
    fn test_rust_query_captures(
        #[case] code: &str,
        #[case] capture_names: Vec<(usize, &str)>,
        #[case] line_ranges: Vec<Range<usize>>,
    ) {
        run_test_case("test.rs", code, capture_names, line_ranges);
    }

    #[rstest]
    #[case(
        r#"
function test() {
    console.log("Hello, world!");
}
"#,
        vec![(0, "function.definition")],
        vec![1..3],
    )]
    #[case(
        r#"
interface Test {
    a: string;
    b: number;
}
"#,
        vec![(0, "struct.definition")],
        vec![1..4],
    )]
    #[case(
        r#"
// add  array function
const test = () => {
    console.log("Hello, world!");   
}

"#,
        vec![(0, "function.comment"), (0, "function.definition")],
        vec![1..1, 2..4],
    )]
    #[case(
        r#"
class Test {
    constructor() {
    }
    test() {
        console.log("Hello, world!");
    }
}
"#,
        vec![(0, "class.definition"), (0, "method.definition"), (1, "method.definition")],
        vec![1..7, 2..3, 4..6],
    )]
    fn test_typescript_query_captures(
        #[case] code: &str,
        #[case] capture_names: Vec<(usize, &str)>,
        #[case] line_ranges: Vec<Range<usize>>,
    ) {
        run_test_case("test.ts", code, capture_names, line_ranges);
    }
}
