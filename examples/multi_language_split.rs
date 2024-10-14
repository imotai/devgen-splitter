use devgen_splitter::splitter::{SplitOptions, Splitter};

fn main() {
    // Rust example
    let rust_code = r#"
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    greet("World");
}
"#;

    // TypeScript example
    let ts_code = r#"
class Greeter {
    greeting: string;

    constructor(message: string) {
        this.greeting = message;
    }

    greet() {
        return "Hello, " + this.greeting;
    }
}

let greeter = new Greeter("world");
console.log(greeter.greet());
"#;

    let options = SplitOptions {
        chunk_line_limit: 5,
        enable_header: true,
    };

    println!("Splitting Rust code:");
    let rust_chunks = Splitter::split("example.rs", rust_code, &options).unwrap();
    let rust_code_lines = rust_code.lines().collect::<Vec<&str>>();
    for chunk in rust_chunks {
        println!("chunk lines: {:?}", chunk.line_range);
        println!(
            "chunk content: {}",
            rust_code_lines[chunk.line_range.clone()].join("\n")
        );
    }

    
}
