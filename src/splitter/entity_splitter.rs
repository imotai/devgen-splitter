use super::{
    CodeChunk,
    CodeEntity,
    SplitOptions,
    Splitter,
};
use anyhow::Result;
use std::ops::Range;
use tiktoken_rs::p50k_base;
use tree_sitter::Node;

impl Splitter {
    pub fn split_entity<'a>(
        lines: &[&str],
        last_chunk_end_line: usize,
        _entity: &CodeEntity,
        nodes: &Vec<Node<'a>>,
        options: &SplitOptions,
    ) -> Result<(Vec<CodeChunk>, usize)> {
        let mut chunks = Vec::new();
        let mut current_chunk_end_line = last_chunk_end_line;
        for node in nodes {
            current_chunk_end_line = Self::chunk_entity(
                node,
                lines,
                current_chunk_end_line,
                &mut chunks,
                options.chunk_token_size,
            )?;
        }
        Ok((chunks, current_chunk_end_line))
    }

    fn chunk_entity(
        node: &Node,
        lines: &[&str],
        last_chunk_end_line: usize,
        chunks: &mut Vec<CodeChunk>,
        max_token_size: usize,
    ) -> Result<usize> {
        let tokenizer = p50k_base()?;
        let mut local_last_chunk_end_line = last_chunk_end_line;
        for i in 0..node.child_count() {
            let child = node.child(i).expect("Failed to get child node");
            let child_start_line = child.start_position().row;
            let child_end_line = child.end_position().row;
            let mut token_count = tokenizer
                .encode_with_special_tokens(
                    &lines[last_chunk_end_line..child_start_line].join("\n"),
                )
                .len();
            if token_count > max_token_size {
                let chunk = CodeChunk {
                    line_range: Range {
                        start: local_last_chunk_end_line,
                        end: child_start_line,
                    },
                    token_count: token_count,
                    entities: Vec::new(),
                };
                chunks.push(chunk);
                local_last_chunk_end_line = child_start_line;
                token_count = 0;
            }
            let node_token_count = tokenizer
                .encode_with_special_tokens(&lines[child_start_line..child_end_line].join("\n"))
                .len();
            if node_token_count > max_token_size {
                if child.child_count() > 0 {
                    let update_last_chunk_end_line = Self::chunk_entity(
                        &child,
                        lines,
                        local_last_chunk_end_line,
                        chunks,
                        max_token_size,
                    )?;
                    local_last_chunk_end_line = update_last_chunk_end_line;
                } else {
                    let chunk = CodeChunk {
                        line_range: Range {
                            start: local_last_chunk_end_line,
                            end: child_start_line,
                        },
                        token_count: node_token_count,
                        entities: Vec::new(),
                    };
                    chunks.push(chunk);
                    local_last_chunk_end_line = child_start_line;
                }
            } else if node_token_count + token_count >= max_token_size {
                let chunk = CodeChunk {
                    line_range: Range {
                        start: local_last_chunk_end_line,
                        end: child_end_line,
                    },
                    token_count: node_token_count + token_count,
                    entities: Vec::new(),
                };
                chunks.push(chunk);
                local_last_chunk_end_line = child_end_line;
            }
        }
        Ok(local_last_chunk_end_line)
    }
}
