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
        Self::split_internal(&lang_config, code, options)
    }

    fn split_internal(
        lang_config: &LangConfig,
        code: &str,
        options: &SplitOptions,
    ) -> Result<Vec<CodeChunk>> {
        let mut parser = Parser::new();
        parser.set_language(&(lang_config.grammar)())?;
        let tree = parser
            .parse(code, None)
            .ok_or(anyhow::anyhow!("Failed to parse code"))?;
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
                println!("capture_name: {:?}", capture_name);
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
                if nodes.len() > 0 {
                    let last_node = nodes.last().expect("Failed to get last node");
                    if last_node.start_position().row == c.node.start_position().row {
                        continue;
                    }
                }
                nodes.push(c.node);
            }
            if nodes.len() > 0 {
                captures_map.insert(definition_start, (captures, nodes));
            }
        }
        let entities = captures_map
            .iter()
            .filter_map(|(_definition_range, (captures, nodes))| {
                match Self::convert_node_to_code_entity(captures, code) {
                    Ok(entity) => Some((entity, nodes.to_vec())),
                    Err(_e) => None,
                }
            })
            .collect::<Vec<(CodeEntity, Vec<Node>)>>();
        Self::merge_code_entities(code, entities, options)
    }
}
