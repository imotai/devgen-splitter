//
// spliter.rs
// Copyright (C) 2024 imotai <codego.me@gmail.com>
// Distributed under terms of the MIT license.
//
mod rust_spliter;
use crate::lang::{
    Lang,
    LangConfig,
};
use anyhow::Result;
use std::{
    collections::HashMap,
    ops::Range,
};
use tree_sitter::{
    Parser,
    Query,
    QueryCursor,
};
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
    pub entities: Vec<CodeEntity>,
}

/// Options for splitting code into chunks
pub struct SplitOptions {
    /// The maximum number of tokens for each code chunk.
    ///
    /// This value determines the size of the "window" used when splitting the code into chunks.
    /// If a chunk exceeds this size, it will be divided into smaller chunks.
    /// A larger value results in fewer, larger chunks, while a smaller value produces more,
    /// smaller chunks.
    pub chunk_token_size: usize,
}

#[derive(Debug, Clone)]
pub struct EntityNode {
    pub byte_range: Range<usize>,
    pub line_range: Range<usize>,
}

pub struct Splitter;

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
        let mut captures_map = vec![];
        for m in matches {
            let mut captures: HashMap<String, EntityNode> = HashMap::new();
            for c in m.captures {
                let capture_name = query.capture_names()[c.index as usize];
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
            }
            captures_map.push(captures)
        }
        match lang_config.lang[0] {
            "Rust" => {
                let entities = captures_map
                    .iter()
                    .filter_map(|captures| {
                        match Self::convert_rust_node_to_code_entity(captures, code) {
                            Ok(entity) => Some(entity),
                            Err(_e) => None,
                        }
                    })
                    .collect::<Vec<CodeEntity>>();
                Self::merge_rust_code_entities(code, entities, options)
            }
            "TypeScript" => {
                println!("TypeScript is {:?}", captures_map);
                anyhow::bail!("TypeScript is not supported yet")
            }
            _ => anyhow::bail!("Unsupported language"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "test.rs",
        r#"
fn main() {
    println!("Hello, world!");
}
    
/// Adds two numbers
fn add(a: i32, b: i32) -> i32 {
    a + b
}
"#,
        SplitOptions { chunk_token_size: 128 },
        vec![
            CodeChunk {
                line_range: 0..5,
                entities: vec![
                    CodeEntity {
                        parent_name: None,
                        name: "main".to_string(),
                        interface_names: vec![],
                        comment_line_range: None,
                        body_line_range: 2..4,
                        entity_type: EntityType::Function,
                    }
                ],
            }
        ]
    )]
    #[case(
        "test.rs",
        r#"
/// A person struct
///
/// # Fields
///
/// * `name` - The name of the person
/// * `age` - The age of the person
struct Person {
    name: String,
    age: u32,
} 

/// Greets a person
/// 
/// # Arguments
///
/// * `person` - The person to greet
fn greet(person: &Person) {
    println!("Hello, {}! You are {} years old.", person.name, person.age);
}
"#,
        SplitOptions { chunk_token_size: 256 },
        vec![
            CodeChunk {
                line_range: 0..20,
                entities: vec![
                    CodeEntity {
                        parent_name: None,
                        name: "Person".to_string(),
                        interface_names: vec![],
                        comment_line_range: Some(1..7),
                        body_line_range: 7..10,
                        entity_type: EntityType::Class,
                    },
                    CodeEntity {
                        parent_name: None,
                        name: "greet".to_string(),
                        interface_names: vec![],
                        comment_line_range: Some(12..17),
                        body_line_range: 17..19,
                        entity_type: EntityType::Function,
                    }
                ],
            }
        ]
    )]
    #[case(
        "test.rs",
        r#"
/// Represents different types of vehicles
enum Vehicle {
    /// A car with four wheels
    Car,
    /// A motorcycle with two wheels
    Motorcycle,
    /// A truck with a specified cargo capacity
    Truck(u32),
}

/// Returns the number of wheels for a given vehicle
fn wheel_count(vehicle: &Vehicle) -> u32 {
    match vehicle {
        Vehicle::Car => 4,
        Vehicle::Motorcycle => 2,
        Vehicle::Truck(_) => 6,
    }
}
"#,
        SplitOptions { chunk_token_size: 256 },
        vec![
            CodeChunk {
                line_range: 0..19,
                entities: vec![
                    CodeEntity {
                        parent_name: None,
                        name: "Vehicle".to_string(),
                        interface_names: vec![],
                        comment_line_range: Some(1..2),
                        body_line_range: 2..9,
                        entity_type: EntityType::Enum,
                    },
                    CodeEntity {
                        parent_name: None,
                        name: "wheel_count".to_string(),
                        interface_names: vec![],
                        comment_line_range: Some(11..12),
                        body_line_range: 12..18,
                        entity_type: EntityType::Function,
                    }
                ],
            }
        ]
    )]
    fn test_rust_split(
        #[case] filename: &str,
        #[case] code: &str,
        #[case] options: SplitOptions,
        #[case] expected: Vec<CodeChunk>,
    ) {
        let result = Splitter::split(filename, code, &options);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected);
    }

    #[rstest]
    #[case(
        "test.ts",
        r#"
/**
 * Represents a person
 */
interface Person {
    name: string;
    age: number;
}

function greet(person: Person): void {
    console.log(`Hello, ${person.name}! You are ${person.age} years old.`);
}

class Employee implements Person {
    constructor(public name: string, public age: number, private company: string) {}

    /**
     * Introduces the employee
     */
    introduce(): void {
        console.log(`I'm ${this.name}, ${this.age} years old, working at ${this.company}.`);
    }
}
"#,
        SplitOptions { chunk_token_size: 256 },
        vec![
            CodeChunk {
                line_range: 0..25,
                entities: vec![
                    CodeEntity {
                        parent_name: None,
                        name: "Person".to_string(),
                        interface_names: vec![],
                        comment_line_range: None,
                        body_line_range: 1..4,
                        entity_type: EntityType::Interface,
                    },
                    CodeEntity {
                        parent_name: None,
                        name: "greet".to_string(),
                        interface_names: vec![],
                        comment_line_range: Some(6..9),
                        body_line_range: 10..12,
                        entity_type: EntityType::Function,
                    },
                    CodeEntity {
                        parent_name: None,
                        name: "Employee".to_string(),
                        interface_names: vec!["Person".to_string()],
                        comment_line_range: None,
                        body_line_range: 14..24,
                        entity_type: EntityType::Class,
                    },
                    CodeEntity {
                        parent_name: Some("Employee".to_string()),
                        name: "introduce".to_string(),
                        interface_names: vec![],
                        comment_line_range: Some(18..20),
                        body_line_range: 21..23,
                        entity_type: EntityType::Method,
                    }
                ],
            }
        ]
    )]
    fn test_typescript_split(
        #[case] filename: &str,
        #[case] code: &str,
        #[case] options: SplitOptions,
        #[case] expected: Vec<CodeChunk>,
    ) {
        let result = Splitter::split(filename, code, &options);
        println!("result: {:?}", result);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected);
    }
}
