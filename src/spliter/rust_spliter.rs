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

/// the capture names for rust function definition
const FUNCTION_DEFINITION: &str = "function.definition";
const FUNCTION_COMMENT: &str = "function.comment";
const FUNCTION_NAME: &str = "function.name";

/// the capture names for rust struct definition
const STRUCT_DEFINITION: &str = "struct.definition";
const STRUCT_COMMENT: &str = "struct.comment";
const STRUCT_NAME: &str = "struct.name";

/// the capture names for rust trait definition
const TRAIT_DEFINITION: &str = "trait.definition";
const TRAIT_COMMENT: &str = "trait.comment";
const TRAIT_NAME: &str = "trait.name";

/// the capture names for rust method definition
const METHOD_DEFINITION: &str = "method.definition";
const METHOD_COMMENT: &str = "method.comment";
const METHOD_NAME: &str = "method.name";
const IMPL_TRAIT_NAME: &str = "impl.trait.name";
const IMPL_CLASS_NAME: &str = "impl.class.name";

/// the capture names for rust enum definition
const ENUM_DEFINITION: &str = "enum.definition";
const ENUM_COMMENT: &str = "enum.comment";
const ENUM_NAME: &str = "enum.name";

impl Splitter {
    /// Merge the code entities into code chunks by the given options
    pub(crate) fn merge_rust_code_entities(
        code: &str,
        entities: Vec<CodeEntity>,
        options: &SplitOptions,
    ) -> Result<Vec<CodeChunk>> {
        let lines: Vec<&str> = code.lines().collect();
        let mut entity_buffer = vec![];
        let mut chunked_line_number = 0;
        let mut chunks = vec![];
        let tokenizer = p50k_base()?;

        for entity in entities {
            let start = if let Some(ref comment_range) = entity.comment_line_range {
                comment_range.start
            } else {
                entity.body_line_range.start
            };
            let left_content = lines[chunked_line_number..start].join("\n");
            let left_token_count = tokenizer.encode_with_special_tokens(&left_content).len();
            if left_token_count > options.chunk_token_size {
                chunks.push(CodeChunk {
                    line_range: chunked_line_number..start,
                    entities: std::mem::take(&mut entity_buffer),
                });
                chunked_line_number = start;
            }
            // Add the entity to the buffer
            entity_buffer.push(entity);
        }

        // Handle the last chunk
        if chunked_line_number < lines.len() {
            chunks.push(CodeChunk {
                line_range: chunked_line_number..lines.len(),
                entities: entity_buffer,
            });
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
        let (entity_type, definition_node, comment_key, name_key) = match (
            captures.get(FUNCTION_DEFINITION),
            captures.get(STRUCT_DEFINITION),
            captures.get(TRAIT_DEFINITION),
            captures.get(METHOD_DEFINITION),
            captures.get(ENUM_DEFINITION),
        ) {
            (Some(node), _, _, _, _) => {
                (EntityType::Function, node, FUNCTION_COMMENT, FUNCTION_NAME)
            }
            (_, Some(node), _, _, _) => (EntityType::Class, node, STRUCT_COMMENT, STRUCT_NAME),
            (_, _, Some(node), _, _) => (EntityType::Interface, node, TRAIT_COMMENT, TRAIT_NAME),
            (_, _, _, Some(node), _) => (EntityType::Method, node, METHOD_COMMENT, METHOD_NAME),
            (_, _, _, _, Some(node)) => (EntityType::Enum, node, ENUM_COMMENT, ENUM_NAME),
            _ => return Err(anyhow::anyhow!("Unsupported entity type")),
        };

        let comment_line_range = captures
            .get(comment_key)
            .map(|node| node.line_range.clone());

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

        Ok(CodeEntity {
            name,
            comment_line_range,
            body_line_range,
            entity_type,
            parent_name,
            interface_names,
        })
    }
}
