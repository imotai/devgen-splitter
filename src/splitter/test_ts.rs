#[cfg(test)]
mod tests {
    use crate::splitter::run_test_case;
    use rstest::*;
    use std::ops::Range;
    #[rstest]
    #[case(
        r#"
function test() {
    console.log("Hello, world!");
}
"#,
        vec![(0, "function.definition")],
        vec![1..3],
    )]
    #[case(
        r#"
interface Test {
    a: string;
    b: number;
}
"#,
        vec![(0, "struct.definition")],
        vec![1..4],
    )]
    #[case(
        r#"
// add  array function
const test = () => {
    console.log("Hello, world!");   
}

"#,
        vec![(0, "function.comment"), (0, "function.definition")],
        vec![1..1, 2..4],
    )]
    #[case(
        r#"
class Test {
    constructor() {
    }
    test() {
        console.log("Hello, world!");
    }
}
"#,
        vec![(0, "class.definition"), (0, "method.definition"), (1, "method.definition")],
        vec![1..7, 2..3, 4..6],
    )]
    fn test_typescript_query_captures(
        #[case] code: &str,
        #[case] capture_names: Vec<(usize, &str)>,
        #[case] line_ranges: Vec<Range<usize>>,
    ) {
        run_test_case("test.ts", code, capture_names, line_ranges);
    }
}
