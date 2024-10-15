//
// lib.rs
// Copyright (C) 2024 imotai <codego.me@gmail.com>
// Distributed under terms of the MIT license.
//
use std::ops::Range;
/// Represents the different types of entities that can be identified in the code.
#[derive(Debug, Clone, PartialEq)]
pub enum EntityType {
    /// Represents a class definition
    Class,
    /// Represents an interface definition
    Interface,
    /// Represents a standalone function
    Function,
    /// Represents a method within a class or interface
    Method,
    /// Represents an enumeration definition
    Enum,
}

/// Represents a code entity with its associated metadata.
///
/// This struct contains information about a specific code entity, including its name,
/// type, and line ranges both in the original source code and within the current chunk.
#[derive(Debug, Clone, PartialEq)]
pub struct Entity {
    /// name of the entity
    pub name: String,
    /// type of the entity
    pub entity_type: EntityType,
    /// the line range of the entity in the source code
    /// including the doc string and the body
    pub completed_line_range: Range<usize>,
    /// the line range of the chunk in the current chunk
    pub chunk_line_range: Range<usize>,
}

/// Represents a chunk of code containing one or more entities.
///
/// A chunk is a section of the source code that may contain multiple entities
/// and is defined by a range of line numbers.
#[derive(Debug, Clone, PartialEq)]
pub struct Chunk {
    /// The line range of the chunk in the source code
    pub line_range: Range<usize>,
    /// The entities contained within this chunk
    pub entities: Vec<Entity>,
}

/// Configuration options for the devgen splitter.
///
/// This struct defines the parameters used to control how the source code
/// is split into chunks, specifying the minimum and maximum number of lines
/// for each chunk.
#[derive(Debug, Clone, PartialEq)]
pub struct SplitterOptions {
    /// The maximum number of lines allowed for each chunk
    pub max_line_limit: usize,
    /// The minimum number of lines required for each chunk
    pub min_line_limit: usize,
}

pub mod lang;
pub mod splitter;
