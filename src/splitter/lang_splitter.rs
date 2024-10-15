//
// rust_spliter.rs
// Copyright (C) 2024 imotai <codego.me@gmail.com>
// Distributed under terms of the MIT license.
//

use super::{
    CodeChunk,
    CodeEntity,
    EntityNode,
    EntityType,
    SplitOptions,
};
use anyhow::Result;
use std::collections::HashMap;
use tree_sitter::Node;

/// the capture names for rust function definition
const FUNCTION_DEFINITION: &str = "function.definition";
const FUNCTION_COMMENT: &str = "function.comment";
const FUNCTION_NAME: &str = "function.name";

/// the capture names for rust struct definition
const STRUCT_DEFINITION: &str = "struct.definition";
const STRUCT_COMMENT: &str = "struct.comment";
const STRUCT_NAME: &str = "struct.name";
const STRUCT_DERIVE: &str = "struct.derive";

/// the capture names for rust trait definition
const TRAIT_DEFINITION: &str = "interface.definition";
const TRAIT_COMMENT: &str = "interface.comment";
const TRAIT_NAME: &str = "interface.name";
const TRAIT_DERIVE: &str = "interface.derive";

/// the capture names for rust method definition
const METHOD_DEFINITION: &str = "method.definition";
const METHOD_COMMENT: &str = "method.comment";
const METHOD_NAME: &str = "method.name";
const IMPL_TRAIT_NAME: &str = "method.interface.name";
const IMPL_CLASS_NAME: &str = "method.class.name";

/// the capture names for rust enum definition
const ENUM_DEFINITION: &str = "enum.definition";
const ENUM_COMMENT: &str = "enum.comment";
const ENUM_NAME: &str = "enum.name";
const ENUM_DERIVE: &str = "enum.derive";

/// Merge the code entities into code chunks by the given options
pub(crate) fn merge_code_entities<'a>(
    code: &str,
    entities: &Vec<(CodeEntity, Vec<Node>)>,
    options: &SplitOptions,
) -> Result<Vec<CodeChunk>> {
    let lines: Vec<&str> = code.lines().collect();
    let mut chunks = vec![];
    let mut last_chunk_end_line = 0;
    let mut current_chunk = CodeChunk {
        line_range: 0..0,
        entities: vec![],
    };
    for (i, (entity, nodes)) in entities.iter().enumerate() {
        let start = if let Some(ref comment_line_range) = &entity.comment_line_range {
            if comment_line_range.start < entity.body_line_range.start {
                comment_line_range.start
            } else {
                entity.body_line_range.start
            }
        } else {
            entity.body_line_range.start
        };
        let end = entity.body_line_range.end;
        if i == 0 && start - last_chunk_end_line > 10 {
            current_chunk.line_range.start = last_chunk_end_line;
            current_chunk.line_range.end = start;
            chunks.push(current_chunk);
            last_chunk_end_line = start;
            current_chunk = CodeChunk {
                line_range: 0..0,
                entities: vec![],
            };
        }
        let left_lines = start - last_chunk_end_line;
        if left_lines > options.chunk_line_limit {
            current_chunk.line_range.start = last_chunk_end_line;
            current_chunk.line_range.end = start;
            chunks.push(current_chunk);
            current_chunk = CodeChunk {
                line_range: 0..0,
                entities: vec![],
            };
            last_chunk_end_line = start;
        }
        let entity_lines = end - start;
        if entity_lines > options.chunk_line_limit {
            let (new_chunks, new_last_chunk_end_line) = super::entity_splitter::split_entity(
                last_chunk_end_line,
                &entity,
                &nodes,
                options,
            )?;
            chunks.extend(new_chunks);
            last_chunk_end_line = new_last_chunk_end_line;
        } else if entity_lines + left_lines > options.chunk_line_limit {
            current_chunk.line_range.start = last_chunk_end_line;
            current_chunk.line_range.end = end + 1;
            current_chunk.entities.push(entity.clone());
            chunks.push(current_chunk);
            last_chunk_end_line = end + 1;
            current_chunk = CodeChunk {
                line_range: 0..0,
                entities: vec![],
            };
        } else {
            current_chunk.entities.push(entity.clone());
        }
    }
    if last_chunk_end_line < lines.len() {
        current_chunk.line_range.start = last_chunk_end_line;
        current_chunk.line_range.end = lines.len();
        chunks.push(current_chunk);
    }
    Ok(chunks)
}

/// Converts the captured nodes to a CodeEntity
///
/// This function processes the captured nodes from the tree-sitter query
/// and constructs a CodeEntity based on the type of definition found.
///
/// # Arguments
///
/// * `captures` - A HashMap containing the captured nodes, where keys are capture names
///
/// # Returns
///
/// A Result containing the constructed CodeEntity
///
/// # Supported Entity Types
///
/// * Function: identified by the "function.definition" key
/// * Struct: identified by the "struct.definition" key
/// * Interface (Trait): identified by the "trait.definition" key
/// * Method: identified by the "method.definition" key
/// * Enum: identified by the "enum.definition" key
///
/// # Errors
///
/// Returns an error if the captures don't contain a recognized entity type
/// or if there's an issue constructing the CodeEntity
pub(crate) fn convert_node_to_code_entity(
    captures: &HashMap<String, EntityNode>,
    code: &str,
) -> Result<CodeEntity> {
    let (entity_type, definition_node, comment_key, name_key, derive_key) = match (
        captures.get(FUNCTION_DEFINITION),
        captures.get(STRUCT_DEFINITION),
        captures.get(TRAIT_DEFINITION),
        captures.get(METHOD_DEFINITION),
        captures.get(ENUM_DEFINITION),
    ) {
        (Some(node), _, _, _, _) => (
            EntityType::Function,
            node,
            FUNCTION_COMMENT,
            FUNCTION_NAME,
            None,
        ),
        (_, Some(node), _, _, _) => (
            EntityType::Struct,
            node,
            STRUCT_COMMENT,
            STRUCT_NAME,
            Some(STRUCT_DERIVE),
        ),
        (_, _, Some(node), _, _) => (
            EntityType::Interface,
            node,
            TRAIT_COMMENT,
            TRAIT_NAME,
            Some(TRAIT_DERIVE),
        ),
        (_, _, _, Some(node), _) => (EntityType::Method, node, METHOD_COMMENT, METHOD_NAME, None),
        (_, _, _, _, Some(node)) => (
            EntityType::Enum,
            node,
            ENUM_COMMENT,
            ENUM_NAME,
            Some(ENUM_DERIVE),
        ),
        _ => return Err(anyhow::anyhow!("Unsupported entity type")),
    };

    let comment_line_range = captures
        .get(comment_key)
        .map(|node| node.line_range.clone());
    let comment_line_range = if let Some(derive_key) = derive_key {
        if let Some(derive_node) = captures.get(derive_key) {
            if let Some(comment_line_range) = comment_line_range.clone() {
                let derive_line_range = derive_node.line_range.clone();
                let start = if derive_line_range.start < comment_line_range.start {
                    derive_line_range.start
                } else {
                    comment_line_range.start
                };
                let end = if derive_line_range.end > comment_line_range.end {
                    derive_line_range.end
                } else if derive_line_range.start == comment_line_range.end {
                    comment_line_range.end + 1
                } else {
                    comment_line_range.end
                };
                Some(start..end)
            } else {
                comment_line_range
            }
        } else {
            comment_line_range
        }
    } else {
        comment_line_range
    };

    let body_line_range = definition_node.line_range.clone();

    let name = captures
        .get(name_key)
        .map(|node| code[node.byte_range.clone()].to_string())
        .ok_or_else(|| anyhow::anyhow!("Entity name not found"))?;

    let (parent_name, interface_names) = if entity_type == EntityType::Method {
        let parent_name = captures
            .get(IMPL_CLASS_NAME)
            .map(|node| code[node.byte_range.clone()].to_string());
        let interface_names = captures
            .get(IMPL_TRAIT_NAME)
            .map(|node| vec![code[node.byte_range.clone()].to_string()])
            .unwrap_or_default();
        (parent_name, interface_names)
    } else {
        (None, vec![])
    };
    let code_entity = CodeEntity {
        name,
        comment_line_range,
        body_line_range,
        entity_type,
        parent_name,
        interface_names,
    };
    Ok(code_entity)
}
