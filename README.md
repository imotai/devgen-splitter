Devgen Splitter is a Rust library that breaks down source code into contextual chunks. 
It utilizes tree-sitter to identify code entities (such as classes, functions, and methods) and generate chunks with contextual metadata.

[![Crates.io Version](https://img.shields.io/crates/v/devgen-splitter)](https://crates.io/crates/devgen-splitter)

## Features

- Language-aware code splitting
- Generate chunks with contextual metadata
- Support for multiple programming languages


## Usage


Add `devgen-splitter` to your project:

```bash
cargo add devgen-splitter
```

Basic example:

```rust
use devgen_splitter::splitter::{SplitOptions, Splitter};

let code = "fn main() { println!(\"Hello, world!\"); }";
let options = SplitOptions { chunk_line_limit: 10, enable_header: true };
let chunks = Splitter::split("example.rs", code, &options).unwrap();

for chunk in chunks {
    println!("Chunk: {:?}", chunkt);
}
```

## Supported Languages

| Language   | Query Rules | Splitter | Test |
|------------|-------------|----------|------|
| Rust       | ✅          | ✅       | ✅   |
| TypeScript | ✅          | ✅       | ✅   |
| Java       | ✅          | ✅       | ✅   |
| Python     | 🚧          | 🚧       | 🚧   |
| Go         | 🚧          | 🚧       | 🚧   |
| C++        | 🚧          | 🚧       | 🚧   |
| C          | 🚧          | 🚧       | 🚧   |
| MD         | 🚧          | 🚧       | 🚧   |

More languages coming soon!

## Language Mapping

The following table shows how different code structures are represented across various programming languages and their corresponding tree-sitter query rule names:

| Type       | Tree-sitter Query | Rust     | Java     | TypeScript | Python   | Go       | C++      |
|------------|-------------------|----------|----------|------------|----------|----------|----------|
| Function   | function.definition | function     | N/A   | function/array function   | function     | function  | function |
| Method   | method.definition | method    | method   | method   | method      | method     | method |
| Struct     | struct.declaration | struct  | class    | interface  | class    | struct   | struct   |
| Class      | class.declaration | impl     | class    | class      | class    | N/A      | class    |
| Interface  | interface.declaration | trait | interface| N/A  | N/A      | N/A| N/A      | N/A      |
| Enum       | enum.declaration  | enum     | enum     | enum       | N/A      | N/A      | enum     |


## Development Status

Devgen Splitter is in active development. We welcome community contributions and feedback.
