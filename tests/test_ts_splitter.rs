use devgen_splitter::splitter::{
    SplitOptions,
    Splitter,
};
use rstest::rstest;

#[rstest]
#[case(
    "ts_function_test.ts",
    include_str!("./cases/ts/typescript_function_test.ts"),
    SplitOptions { chunk_line_limit: 40, enable_header: true },
    4
)]
#[case(
    "ts_react_test.tsx",
    include_str!("./cases/ts/typescript_react_test.ts"),
    SplitOptions { chunk_line_limit: 40, enable_header: true },
    6
)]
fn test_ts_split(
    #[case] filename: &str,
    #[case] code: &str,
    #[case] options: SplitOptions,
    #[case] expected: usize,
) {
    let result = Splitter::split(filename, code, &options);
    assert_eq!(result.is_ok(), true);
    let result = result.unwrap();
    let lines = code.lines().collect::<Vec<&str>>();
    for chunk in &result {
        println!(
            "----------------{:?} {}--------------",
            chunk.line_range,
            chunk.header.clone().unwrap_or("".to_string())
        );
        println!("{}", lines[chunk.line_range.clone()].join("\n"));
        println!("-------------------------------");
    }
    assert_eq!(result.len(), expected);
}
