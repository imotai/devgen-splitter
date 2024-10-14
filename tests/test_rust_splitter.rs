use code_splitter::splitter::{
    SplitOptions,
    Splitter,
};
use rstest::rstest;

#[rstest]
#[case(
    "rust_function_test.rs",
    include_str!("./cases/rust/rust_function_test.rs"),
    SplitOptions { chunk_token_size: 128 },
    1
)]
#[case(
    "rust_function_in_mod.rs",
    include_str!("./cases/rust/rust_function_in_mod.rs"),
    SplitOptions { chunk_token_size: 128 },
    1
)]
#[case(
    "rust_long_function.rs",
    include_str!("./cases/rust/rust_long_function.rs"),
    SplitOptions { chunk_token_size: 128 },
    1
)]
fn test_rust_split(
    #[case] filename: &str,
    #[case] code: &str,
    #[case] options: SplitOptions,
    #[case] expected: usize,
) {
    let result = Splitter::split(filename, code, &options);
    assert_eq!(result.is_ok(), true);
    let result = result.unwrap();
    let lines = code.lines().collect::<Vec<&str>>();
    for chunk in result {
        println!("----------------{:?}--------------", chunk.line_range);
        println!("{}", lines[chunk.line_range.clone()].join("\n"));
        println!("-------------------------------");
    }
}
