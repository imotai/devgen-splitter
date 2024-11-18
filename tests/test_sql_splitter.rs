use devgen_splitter::{
    split,
    SplitOptions,
};
use rstest::rstest;

#[rstest]
#[case(
    "test.sql",
    include_str!("./cases/sql/test_sql.sql"),
    SplitOptions { chunk_line_limit: 10},
    1
)]
#[case(
    "test.sql",
    include_str!("./cases/sql/test_sql_large_query.sql"),
    SplitOptions { chunk_line_limit: 10},
    2
)]
fn test_sql_split(
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
