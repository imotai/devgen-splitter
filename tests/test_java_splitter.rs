use devgen_splitter::splitter::{
    SplitOptions,
    Splitter,
};
use rstest::rstest;

#[rstest]
#[case(
    "java_function_test.java",
    include_str!("./cases/java/test_java.java"),
    SplitOptions { chunk_line_limit: 40, enable_header: true },
    4
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
