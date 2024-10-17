use devgen_splitter::{
    split,
    SplitOptions,
};
use rstest::rstest;

#[rstest]
#[case(
    "rust_function_test.rs",
    include_str!("./cases/rust/rust_function_test.rs"),
    SplitOptions { chunk_line_limit: 40},
    1
)]
#[case(
    "rust_function_in_mod.rs",
    include_str!("./cases/rust/rust_function_in_mod.rs"),
    SplitOptions { chunk_line_limit: 40},
    3
)]
#[case(
    "rust_long_function.rs",
    include_str!("./cases/rust/rust_long_function.rs"),
    SplitOptions { chunk_line_limit: 40 },
    4
)]
#[case(
    "rust_tonic_case.rs",
    include_str!("./cases/rust/rust_tonic_case.rs"),
    SplitOptions { chunk_line_limit: 40 },
    10
)]
fn test_rust_split(
    #[case] filename: &str,
    #[case] code: &str,
    #[case] options: SplitOptions,
    #[case] expected: usize,
) {
    let result = split(filename, code, &options);
    assert_eq!(result.is_ok(), true);
    let result = result.unwrap();
    assert_eq!(result.len(), expected);
    let lines = code.lines().collect::<Vec<&str>>();
    for chunk in result {
        println!("----------------{:?} --------------", chunk.line_range,);
        println!(
            "{}",
            lines[chunk.line_range.start..chunk.line_range.end].join("\n")
        );
        println!("-------------------------------");
    }
}
