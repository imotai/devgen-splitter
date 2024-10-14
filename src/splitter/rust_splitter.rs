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
    Splitter,
};
use anyhow::Result;
use std::collections::HashMap;
use tiktoken_rs::p50k_base;
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
const TRAIT_DEFINITION: &str = "trait.definition";
const TRAIT_COMMENT: &str = "trait.comment";
const TRAIT_NAME: &str = "trait.name";
const TRAIT_DERIVE: &str = "trait.derive";

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

impl Splitter {
    /// Merge the code entities into code chunks by the given options
    pub(crate) fn merge_rust_code_entities<'a>(
        code: &str,
        entities: Vec<(CodeEntity, Vec<Node>)>,
        options: &SplitOptions,
    ) -> Result<Vec<CodeChunk>> {
        let lines: Vec<&str> = code.lines().collect();
        let mut chunks = vec![];
        let tokenizer = p50k_base()?;
        let mut last_chunk_end_line = 0;
        let mut current_chunk = CodeChunk {
            line_range: 0..0,
            entities: vec![],
            token_count: 0,
        };
        for (entity, nodes) in entities {
            println!(
                "entity: {:?} last_chunk_end_line: {:?}",
                entity, last_chunk_end_line
            );
            let start = entity
                .comment_line_range
                .as_ref()
                .map_or(entity.body_line_range.start, |range| range.start);
            let end = entity.body_line_range.end;
            let entity_content = lines[start..end].join("\n");
            let entity_token_count = tokenizer.encode_with_special_tokens(&entity_content).len();
            let left_content = lines[last_chunk_end_line..start].join("\n");
            let left_token_count = tokenizer.encode_with_special_tokens(&left_content).len();
            if left_token_count > options.chunk_token_size {
                current_chunk.line_range.start = last_chunk_end_line;
                current_chunk.line_range.end = start;
                current_chunk.token_count = left_token_count;
                chunks.push(current_chunk);
                current_chunk = CodeChunk {
                    line_range: 0..0,
                    entities: vec![],
                    token_count: 0,
                };
                last_chunk_end_line = start;
            }
            if entity_token_count > options.chunk_token_size {
                if !current_chunk.entities.is_empty() {
                    current_chunk.line_range.start = last_chunk_end_line;
                    current_chunk.line_range.end = start;
                    chunks.push(current_chunk);
                    last_chunk_end_line = start;
                    current_chunk = CodeChunk {
                        line_range: 0..0,
                        entities: vec![],
                        token_count: 0,
                    };
                }
                let (new_chunks, new_last_chunk_end_line) =
                    Self::split_entity(&lines, last_chunk_end_line, &entity, &nodes, options)?;
                chunks.extend(new_chunks);
                last_chunk_end_line = new_last_chunk_end_line;
            } else if left_token_count + entity_token_count > options.chunk_token_size {
                current_chunk.line_range.start = last_chunk_end_line;
                current_chunk.line_range.end = end;
                current_chunk.token_count = left_token_count + entity_token_count;
                current_chunk.entities.push(entity);
                chunks.push(current_chunk);
                last_chunk_end_line = end;
                current_chunk = CodeChunk {
                    line_range: 0..0,
                    entities: vec![],
                    token_count: 0,
                };
            } else {
                current_chunk.entities.push(entity);
            }
        }
        if last_chunk_end_line < lines.len() {
            current_chunk.line_range.start = last_chunk_end_line;
            current_chunk.line_range.end = lines.len();
            let content = lines[last_chunk_end_line..].join("\n");
            let token_count = tokenizer.encode_with_special_tokens(&content).len();
            current_chunk.token_count = token_count;
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
    pub(crate) fn convert_rust_node_to_code_entity(
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
                EntityType::Class,
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
            (_, _, _, Some(node), _) => {
                (EntityType::Method, node, METHOD_COMMENT, METHOD_NAME, None)
            }
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
}
