use super::{
    CodeChunk,
    CodeEntity,
    SplitOptions,
};
use anyhow::Result;
use std::ops::Range;
use tree_sitter::Node;

pub fn split_entity<'a>(
    last_chunk_end_line: usize,
    entity: &CodeEntity,
    nodes: &Vec<Node<'a>>,
    options: &SplitOptions,
) -> Result<(Vec<CodeChunk>, usize)> {
    let mut chunks = Vec::new();
    let mut current_chunk_end_line = last_chunk_end_line;
    for node in nodes {
        current_chunk_end_line =
            chunk_entity(node, current_chunk_end_line, &mut chunks, options, entity)?;
    }
    Ok((chunks, current_chunk_end_line))
}

fn chunk_entity(
    node: &Node,
    last_chunk_end_line: usize,
    chunks: &mut Vec<CodeChunk>,
    options: &SplitOptions,
    entity: &CodeEntity,
) -> Result<usize> {
    let mut local_last_chunk_end_line = last_chunk_end_line;
    for i in 0..node.child_count() {
        let child = node.child(i).expect("Failed to get child node");
        let child_end_line = child.end_position().row;
        let child_start_line = child.start_position().row;
        if child_start_line < local_last_chunk_end_line {
            continue;
        }
        let left_chunk_line_count = child_start_line - local_last_chunk_end_line;
        if left_chunk_line_count > options.chunk_line_limit {
            let chunk = CodeChunk {
                line_range: Range {
                    start: local_last_chunk_end_line,
                    end: child_start_line,
                },
                entities: vec![entity.clone()],
            };
            chunks.push(chunk);
            local_last_chunk_end_line = child_start_line;
        }
        let node_line_count = child_end_line - child_start_line;
        if node_line_count > options.chunk_line_limit {
            if child.child_count() > 0 {
                let update_last_chunk_end_line =
                    chunk_entity(&child, local_last_chunk_end_line, chunks, options, entity)?;
                local_last_chunk_end_line = update_last_chunk_end_line;
            } else {
                let chunk = CodeChunk {
                    line_range: Range {
                        start: local_last_chunk_end_line,
                        end: child_end_line,
                    },
                    entities: vec![entity.clone()],
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
                entities: vec![entity.clone()],
            };
            chunks.push(chunk);
            local_last_chunk_end_line = child_end_line;
        }
    }
    Ok(local_last_chunk_end_line)
}
