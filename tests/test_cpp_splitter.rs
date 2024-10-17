use devgen_splitter::{
    split,
    SplitOptions,
};
use rstest::rstest;

#[rstest]
#[case(
    "test.cc",
    include_str!("./cases/cpp/test.cc"),
    SplitOptions { chunk_line_limit: 20 },
    2
)]
fn test_cpp_split(
    #[case] filename: &str,
    #[case] code: &str,
    #[case] options: SplitOptions,
    #[case] expected: usize,
) {
    let result = split(filename, code, &options);
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
