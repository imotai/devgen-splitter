use super::{
    CodeChunk,
    CodeEntity,
    EntityType,
    SplitOptions,
    Splitter,
};
use anyhow::Result;
use std::ops::Range;
use tree_sitter::Node;

impl Splitter {
    pub fn build_header_for_entities(entities: &Vec<CodeEntity>) -> Option<String> {
        if entities.is_empty() {
            return None;
        }

        // Check if all entities are methods and belong to the same class
        let all_methods = entities.iter().all(|e| e.entity_type == EntityType::Method);
        let same_class = entities
            .iter()
            .all(|e| e.parent_name == entities[0].parent_name);

        if all_methods && same_class {
            // Get the class name from the first entity's parent_name
            if let Some(class_name) = &entities[0].parent_name {
                return Some(format!(
                    "The following methods belong to class '{}'",
                    class_name
                ));
            }
        }

        None
    }

    pub fn build_header(entity: &CodeEntity) -> Option<String> {
        match entity.entity_type {
            EntityType::Class => Some(format!(
                "The incomplete part comes from the '{}' class definition",
                entity.name
            )),
            EntityType::Interface => Some(format!(
                "The incomplete part comes from the '{}' interface definition",
                entity.name
            )),
            EntityType::Function => Some(format!(
                "The incomplete part comes from the '{}' function implementation",
                entity.name
            )),
            EntityType::Method => Some(format!(
                "The incomplete part comes from the '{}' method implementation in the '{}' class",
                entity.name,
                entity.parent_name.as_deref().unwrap_or("")
            )),
            EntityType::Enum => Some(format!(
                "The incomplete part comes from the '{}' enum definition",
                entity.name
            )),
        }
    }

    pub fn split_entity<'a>(
        last_chunk_end_line: usize,
        entity: &CodeEntity,
        nodes: &Vec<Node<'a>>,
        options: &SplitOptions,
    ) -> Result<(Vec<CodeChunk>, usize)> {
        let mut chunks = Vec::new();
        let mut current_chunk_end_line = last_chunk_end_line;
        let header = if options.enable_header {
            Self::build_header(entity)
        } else {
            None
        };
        for node in nodes {
            current_chunk_end_line = Self::chunk_entity(
                node,
                current_chunk_end_line,
                &mut chunks,
                options,
                header.clone(),
            )?;
        }
        Ok((chunks, current_chunk_end_line))
    }

    fn chunk_entity(
        node: &Node,
        last_chunk_end_line: usize,
        chunks: &mut Vec<CodeChunk>,
        options: &SplitOptions,
        header: Option<String>,
    ) -> Result<usize> {
        let mut local_last_chunk_end_line = last_chunk_end_line;
        for i in 0..node.child_count() {
            let child = node.child(i).expect("Failed to get child node");
            let child_end_line = child.end_position().row;
            let child_start_line = child.start_position().row;
            let left_chunk_line_count = child_start_line - local_last_chunk_end_line;
            if left_chunk_line_count > options.chunk_line_limit {
                let chunk = CodeChunk {
                    line_range: Range {
                        start: local_last_chunk_end_line,
                        end: child_start_line,
                    },
                    entities: Vec::new(),
                    header: header.clone(),
                };
                chunks.push(chunk);
                local_last_chunk_end_line = child_start_line;
            }
            let node_line_count = child_end_line - child_start_line;
            if node_line_count > options.chunk_line_limit {
                if child.child_count() > 0 {
                    let update_last_chunk_end_line = Self::chunk_entity(
                        &child,
                        local_last_chunk_end_line,
                        chunks,
                        options,
                        header.clone(),
                    )?;
                    local_last_chunk_end_line = update_last_chunk_end_line;
                } else {
                    let chunk = CodeChunk {
                        line_range: Range {
                            start: local_last_chunk_end_line,
                            end: child_end_line,
                        },
                        entities: Vec::new(),
                        header: header.clone(),
                    };
                    chunks.push(chunk);
                    local_last_chunk_end_line = child_end_line;
                }
            } else if node_line_count + left_chunk_line_count >= options.chunk_line_limit {
                let chunk = CodeChunk {
                    line_range: Range {
                        start: local_last_chunk_end_line,
                        end: child_end_line,
                    },
                    entities: Vec::new(),
                    header: header.clone(),
                };
                chunks.push(chunk);
                local_last_chunk_end_line = child_end_line;
            }
        }
        Ok(local_last_chunk_end_line)
    }
}
