use devgen_splitter::{
    split,
    SplitOptions,
};

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
    };
    println!("Splitting Rust code:");
    let rust_chunks = split("example.rs", rust_code, &options).unwrap();
    let rust_code_lines = rust_code.lines().collect::<Vec<&str>>();
    for chunk in rust_chunks {
        println!("chunk lines: {:?}", chunk.line_range);
        println!(
            "chunk content: {}",
            rust_code_lines[chunk.line_range.clone()].join("\n")
        );
        println!("----------context----------");
        for entity in chunk.entities {
            println!("entity: {:?}", entity);
        }
    }
    println!("Splitting Ts code:");
    let ts_chunks = split("example.ts", ts_code, &options).unwrap();
    let ts_code_lines = ts_code.lines().collect::<Vec<&str>>();
    for chunk in ts_chunks {
        println!("chunk lines: {:?}", chunk.line_range);
        println!(
            "chunk content: {}",
            ts_code_lines[chunk.line_range.clone()].join("\n")
        );
        println!("----------context----------");
        for entity in chunk.entities {
            println!("entity: {:?}", entity);
        }
    }
}
