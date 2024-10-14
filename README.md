Devgen Splitter is a Rust library designed to intelligently parse and split source code into meaningful chunks. It uses tree-sitter grammars to understand the structure of various programming languages, allowing it to split code based on logical entities such as classes, functions, methods, and interfaces.

{{ Devgen Splitter is a submodule of Devgen, a Code Research Assistant for GitHub users. }}

Key features:
- Language-aware code splitting
- Identification of code entities (classes, functions, methods, etc.)
- Flexible chunking options
- Support for multiple programming languages


## How to Use

To use Devgen Splitter in your Rust project, add it to your `Cargo.toml`:

```toml
[dependencies]
devgen-splitter = "0.1.0"
```

### Basic Usage

Here's a simple example of how to use Devgen Splitter to split a code file:

```rust
use devgen_splitter::splitter::{
    SplitOptions,
    Splitter,
};

let code = "fn main() { println!("Hello, world!"); }";
let options = SplitOptions { chunk_line_limit: 10, enable_header: true };
let chunks = Splitter::split("example.rs", code, &options).unwrap();

for chunk in chunks {
    println!("Chunk: {}", chunk.content);
}
```

### Customizing Split Options

You can customize the splitting behavior by passing a `SplitOptions` struct to the `split` method. The `chunk_line_limit` parameter controls the maximum number of lines for each chunk, and `enable_header` toggles the inclusion of a header in each chunk.

## Language Support

Devgen Splitter currently supports the following programming languages:

- [x] Rust
- [x] Typescript
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

## Development Status

Devgen Splitter is in active development. Current status:

- Core functionality implemented for supported languages
- Ongoing improvements and expansion of capabilities
- Actively seeking community contributions and feedback

We welcome your input to help shape the future of Devgen Splitter!
