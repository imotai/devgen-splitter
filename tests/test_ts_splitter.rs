use devgen_splitter::{
    split,
    SplitOptions,
};
use rstest::rstest;

#[rstest]
#[case(
    "ts_function_test.ts",
    include_str!("./cases/ts/typescript_function_test.ts"),
    SplitOptions { chunk_line_limit: 40},
    3

)]
#[case(
    "ts_react_test.tsx",
    include_str!("./cases/ts/typescript_react_test.ts"),
    SplitOptions { chunk_line_limit: 40},
    6
)]
#[case(
    "ts_function_class.ts",
    include_str!("./cases/ts/typescript_function_class.ts"),
    SplitOptions { chunk_line_limit: 30},
    3
)]
fn test_ts_split(
    #[case] filename: &str,
    #[case] code: &str,
    #[case] options: SplitOptions,
    #[case] expected: usize,
) {
    let result = split(filename, code, &options);
    println!("result: {:?}", result);
    assert_eq!(result.is_ok(), true);
    let result = result.unwrap();
    let lines = code.lines().collect::<Vec<&str>>();
    for chunk in &result {
        println!("----------------{:?} --------------", chunk.line_range,);
        println!("{}", lines[chunk.line_range.clone()].join("\n"));
        println!("-------------------------------");
    }
    assert_eq!(result.len(), expected);
}
