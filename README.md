Devgen Splitter is a Rust tool that breaks down source code into meaningful parts. It uses special language rules to understand code structure in different programming languages and creates helpful descriptions to show how different pieces of code relate to each other.

## Features

- Language-aware code splitting
- Identification of code entities (classes, functions, methods, etc.)
- Flexible chunking options
- Support for multiple programming languages
- Contextual header generation based on the relationship between entities

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
devgen-splitter = "0.1.0"
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

- [x] Rust
- [x] TypeScript
- [x] Java
- [ ] Python
- [ ] Go
- [ ] C++
- [ ] C#
- [ ] PHP
- [ ] SQL
- [ ] Ruby
- [ ] Bash
- [ ] MD

More languages coming soon!

## Development Status

Devgen Splitter is in active development. We welcome community contributions and feedback.
