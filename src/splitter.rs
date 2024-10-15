//
// spliter.rs
// Copyright (C) 2024 imotai <codego.me@gmail.com>
// Distributed under terms of the MIT license.
//
pub mod entity_splitter;
mod lang_splitter;
use crate::lang::{
    Lang,
    LangConfig,
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

/// Represents the type of code entity
#[derive(Debug, Clone, PartialEq)]
pub enum EntityType {
    /// Represents a class definition
    Class,
    /// Represents a standalone function
    Function,
    /// Represents an interface or trait definition
    Interface,
    /// Represents a method within a class or implementation block
    Method,
    /// Represents an enum definition
    Enum,
}

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
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CodeChunk {
    pub line_range: Range<usize>,
    /// description of the chunk
    /// it's a short string to describe the chunk content
    pub header: Option<String>,
    /// entities in the chunk
    pub entities: Vec<CodeEntity>,
}

/// Options for splitting code into chunks
pub struct SplitOptions {
    /// The maximum number of lines for each code chunk.
    ///
    /// This value determines the size of the "window" used when splitting the code into chunks.
    /// If a chunk exceeds this size, it will be divided into smaller chunks.
    /// A larger value results in fewer, larger chunks, while a smaller value produces more,
    /// smaller chunks.
    pub chunk_line_limit: usize,
    pub enable_header: bool,
}

#[derive(Debug, Clone)]
pub struct EntityNode {
    pub byte_range: Range<usize>,
    pub line_range: Range<usize>,
}

pub struct Splitter {}

impl Splitter {
    pub fn split(filename: &str, code: &str, options: &SplitOptions) -> Result<Vec<CodeChunk>> {
        let lang_config =
            Lang::from_filename(filename).ok_or(anyhow::anyhow!("Unsupported language"))?;
        let mut parser = Parser::new();
        parser.set_language(&(lang_config.grammar)())?;
        let tree = parser
            .parse(code, None)
            .ok_or(anyhow::anyhow!("Failed to parse code"))?;
        let captures = Self::query_captures(&lang_config, code, &tree)?;
        let entities = captures
            .iter()
            .filter_map(|(captures, nodes)| {
                match Self::convert_node_to_code_entity(captures, code) {
                    Ok(entity) => Some((entity, nodes.to_vec())),
                    Err(_e) => None,
                }
            })
            .collect::<Vec<(CodeEntity, Vec<Node>)>>();
        Self::merge_code_entities(code, &entities, options)
    }

    pub(crate) fn query_captures<'a>(
        lang_config: &LangConfig,
        code: &'a str,
        tree: &'a Tree,
    ) -> Result<Vec<(HashMap<String, EntityNode>, Vec<Node<'a>>)>> {
        let query = Query::new(&(lang_config.grammar)(), lang_config.query)?;
        let mut query_cursor = QueryCursor::new();
        let matches = query_cursor.matches(&query, tree.root_node(), code.as_bytes());
        let mut captures_map: BTreeMap<usize, (HashMap<String, EntityNode>, Vec<Node>)> =
            BTreeMap::new();
        for m in matches {
            let mut captures: HashMap<String, EntityNode> = HashMap::new();
            let mut nodes = vec![];
            let mut definition_start = 0;
            for c in m.captures {
                let capture_name = query.capture_names()[c.index as usize];
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
                    captures_map.insert(definition_start, (captures.clone(), nodes));
                    let new_captures = captures.clone();
                    captures = HashMap::new();
                    captures.extend(new_captures);
                    captures.remove(METHOD_NAME);
                    captures.remove(METHOD_DEFINITION);
                    captures.remove(METHOD_COMMENT);
                    nodes = vec![];
                    continue;
                }
                if capture_name.ends_with(".name") {
                    continue;
                }
                nodes.push(c.node);
            }
            if nodes.len() > 0 {
                captures_map.insert(definition_start, (captures, nodes));
            }
        }
        Ok(captures_map
            .iter()
            .map(|(_start, (captures, nodes))| (captures.clone(), nodes.clone()))
            .collect::<Vec<(HashMap<String, EntityNode>, Vec<Node>)>>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
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
        let captures = Splitter::query_captures(&lang_config, code, &tree).unwrap();
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
        vec![(0, "interface.definition"), (1, "interface.definition")],
        vec![1..3, 5..7],
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
