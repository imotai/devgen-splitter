//! # DevGen Code Splitter Library
//!
//! This library provides functionality for splitting source code into manageable chunks
//! and identifying various code entities within those chunks. It's designed to assist
//! in code analysis, documentation generation, and other tasks that require structured
//! parsing of source code.
//!
//! ## Main Components:
//!
//! - `EntityType`: Enum representing different types of code entities (e.g., Struct, Function).
//! - `Entity`: Struct containing metadata about a specific code entity.
//! - `Chunk`: Struct representing a section of code containing one or more entities.
//! - `SplitOptions`: Configuration options for controlling how code is split into chunks.
//! - `Lang`: Enum representing supported programming languages (imported from `lang` module).
//! - `split`: Function for splitting code into chunks (imported from `splitter` module).
//!
//! ## Usage Example:
//!
//! ```rust
//! use devgen_splitter::{
//!     split,
//!     Lang,
//!     SplitOptions,
//! };
//!
//! let source_code = "// Your source code here...";
//! let options = SplitOptions {
//!     chunk_line_limit: 100,
//! };
//! let chunks = split("test.rs", source_code, &options).unwrap();
//!
//! for chunk in chunks {
//!     println!("Chunk line range: {:?}", chunk.line_range);
//!     for entity in chunk.entities {
//!         println!("Entity: {} ({:?})", entity.name, entity.entity_type);
//!     }
//! }
//! ```

// ... (rest of the existing code remains unchanged)
use serde::{
    Deserialize,
    Serialize,
};
use std::ops::Range;

/// Represents the different types of entities that can be identified in the code.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EntityType {
    /// Represents a struct definition
    Struct,
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    /// if the entity is a method, the name of the parent struct or interface
    pub parent: Option<String>,
    /// the line range of the parent in the source code
    pub parent_line_range: Option<Range<usize>>,
}

/// Represents a chunk of code containing one or more entities.
///
/// A chunk is a section of the source code that may contain multiple entities
/// and is defined by a range of line numbers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Chunk {
    /// The line range of the chunk in the source code
    pub line_range: Range<usize>,
    /// The entities contained within this chunk
    pub entities: Vec<Entity>,
}

/// Configuration options for the devgen splitter.
///
/// This struct defines the parameters used to control how the source code
/// is split into chunks, specifying the maximum number of characters for each chunk.
#[derive(Debug, Clone, Default)]
pub struct SplitOptions {
    /// the maximum number of lines for each chunk
    pub chunk_line_limit: usize,
}

mod lang;
mod splitter;
pub use lang::Lang;
pub use splitter::split;
