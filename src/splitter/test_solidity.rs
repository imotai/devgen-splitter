#[cfg(test)]
mod tests {
    use crate::splitter::run_test_case;
    use rstest::*;
    use std::ops::Range;

    #[rstest]
    #[case(
        r#"
contract Test {
    uint256 public a;
    function test(address db) public {
    }
}
"#,
        vec![(0, "class.definition"), (0, "method.definition")],
        vec![1..5, 3..4],
    )]
    // test struct in solidity
    #[case(
        r#"
contract Test {
    // struct definition
    struct Data1 {
        int a;
        string b;
    }
}
"#,
        vec![(0, "struct.comment"), (0, "struct.definition")],
        vec![2..2, 3..6],
    )]
    fn test_solidity_capture(
        #[case] code: &str,
        #[case] capture_names: Vec<(usize, &str)>,
        #[case] line_ranges: Vec<Range<usize>>,
    ) {
        run_test_case("test.sol", code, capture_names, line_ranges);
    }
}
