use crate::Chunk;
use anyhow::Result;
use std::ops::Range;
use tree_sitter::Node;

pub fn split_tree_node(
    lines: &Vec<&str>,
    node: &Node,
    max_lines_per_chunk: usize,
    min_lines_per_chunk: usize,
) -> Result<Vec<Chunk>> {
    let mut chunks: Vec<Range<usize>> = Vec::new();
    let last_chunk_end_line_number = chunk_by_lines(
        node,
        0,
        &mut chunks,
        max_lines_per_chunk,
        min_lines_per_chunk,
    )?;

    if last_chunk_end_line_number < lines.len()
        && lines.len() - last_chunk_end_line_number > min_lines_per_chunk
    {
        chunks.push(Range {
            start: last_chunk_end_line_number,
            end: lines.len(),
        });
    }

    Ok(chunks
        .iter()
        .map(|chunk| {
            let start = chunk.start;
            let end = chunk.end;
            Chunk {
                line_range: start..end,
                entities: vec![],
            }
        })
        .collect())
}

fn chunk_by_lines(
    node: &Node,
    last_chunk_end_line: usize,
    chunks: &mut Vec<Range<usize>>,
    max_lines_per_chunk: usize,
    min_lines_per_chunk: usize,
) -> Result<usize> {
    let mut local_last_chunk_end_line = last_chunk_end_line;
    for i in 0..node.child_count() {
        let child = node.child(i).expect("Failed to get child node");
        let child_start_line = child.start_position().row;
        let child_end_line = child.end_position().row;
        if child_start_line - local_last_chunk_end_line >= max_lines_per_chunk {
            chunks.push(Range {
                start: local_last_chunk_end_line,
                end: child_start_line,
            });
            local_last_chunk_end_line = child_start_line;
        }
        // split child node
        if child_end_line - child_start_line > max_lines_per_chunk {
            if child.child_count() > 0 {
                let update_last_chunk_end_line = chunk_by_lines(
                    &child,
                    local_last_chunk_end_line,
                    chunks,
                    max_lines_per_chunk,
                    min_lines_per_chunk,
                )?;
                local_last_chunk_end_line = update_last_chunk_end_line;
            } else {
                // check the min chunk line size
                if child_start_line - local_last_chunk_end_line >= min_lines_per_chunk {
                    chunks.push(Range {
                        start: local_last_chunk_end_line,
                        end: child_start_line,
                    });
                    local_last_chunk_end_line = child_start_line;
                }
                chunks.push(Range {
                    start: local_last_chunk_end_line,
                    end: child_end_line,
                });
                local_last_chunk_end_line = child_end_line;
            }
        } else if child_end_line - local_last_chunk_end_line >= max_lines_per_chunk {
            chunks.push(Range {
                start: local_last_chunk_end_line,
                end: child_end_line,
            });
            local_last_chunk_end_line = child_end_line;
        }
    }
    Ok(local_last_chunk_end_line)
}
