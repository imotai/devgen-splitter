use devgen_splitter::{
    split,
    SplitOptions,
};
use rstest::rstest;

#[rstest]
#[case(
    "python_function_test.py",
    include_str!("./cases/python/test_octogen.py"),
    SplitOptions { chunk_line_limit: 40},
    12
)]
fn test_python_splitter(
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
