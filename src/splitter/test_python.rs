#[cfg(test)]
mod tests {
    use crate::splitter::run_test_case;
    use rstest::*;
    use std::ops::Range;

    #[rstest]
    #[case(
        r#"
def test():
    print("Hello, world!")
"#,
        vec![(0, "function.definition")],
        vec![1..2],
    )]
    #[rstest]
    #[case(
        r#"
class Test:
    def test(self):
        print("Hello, world!")
"#,
        vec![(0, "class.definition"), (0, "method.definition")],
        vec![1..3, 2..3],
    )]
    fn test_python_split(
        #[case] code: &str,
        #[case] capture_names: Vec<(usize, &str)>,
        #[case] line_ranges: Vec<Range<usize>>,
    ) {
        run_test_case("test.py", code, capture_names, line_ranges);
    }
}
