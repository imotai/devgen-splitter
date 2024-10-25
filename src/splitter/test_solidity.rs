#[cfg(test)]
mod tests {
    use crate::splitter::run_test_case;
    use rstest::*;
    use std::ops::Range;

    #[rstest]
    #[case(
        r#"
contract Test {
    function test() public {
    }
}
"#,
        vec![(0, "class.definition"), (0, "method.definition")],
        vec![1..4, 2..3],
    )]
    fn test_solidity_split(
        #[case] code: &str,
        #[case] capture_names: Vec<(usize, &str)>,
        #[case] line_ranges: Vec<Range<usize>>,
    ) {
        run_test_case("test.sol", code, capture_names, line_ranges);
    }
}
