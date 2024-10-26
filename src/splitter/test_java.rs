#[cfg(test)]
mod tests {
    use crate::splitter::run_test_case;
    use rstest::*;
    use std::ops::Range;
    #[rstest]
    #[case(
        r#"
public class Test {
    public void test() {
        System.out.println("Hello, world!");
    }
}
"#,
        vec![(0, "class.definition"), (0, "method.definition")],
        vec![1..5, 2..4],
    )]
    #[case(
        r#"
public class Test {
    public void test() {
        System.out.println("Hello, world!");
    }
    public void test2() {
        System.out.println("Hello, world!");
    }
}
"#,
    vec![(0, "class.definition"), (0, "method.definition"), (1, "class.definition"), (1, "method.definition")],
    vec![1..8, 2..4, 1..8, 5..7],
    )]
    #[case(
        r#"
public interface Test {
    void test();
}
"#,
        vec![(0, "class.definition"), (0, "method.definition")],
        vec![1..3, 2..2],
    )]
    // test class with comment
    #[case(
        r#"
/**
 * this is a test
 */
public class Test {
    public void test() {
        System.out.println("Hello, world!");
    }
}
"#,
        vec![(0, "class.definition"), (0, "method.definition")],
        vec![4..8, 5..7],
    )]
    // test sub class
    #[case(
        r#"
public class Test2 extends Test {
    public void test2() {
        System.out.println("Hello, world!");
    }
}
"#,
        vec![(0, "class.definition"), (0, "method.definition")],
        vec![1..5, 2..4],
    )]
    // test inner class
    #[case(
        r#"
public class Test {
    public class Test2 {
        public void test2() {
            System.out.println("Hello, world!");
        }
    }
}
"#,
        vec![(0, "class.definition"), (0, "method.definition")],
        vec![2..6, 3..5],
    )]
    fn test_java_query_captures(
        #[case] code: &str,
        #[case] capture_names: Vec<(usize, &str)>,
        #[case] line_ranges: Vec<Range<usize>>,
    ) {
        run_test_case("test.java", code, capture_names, line_ranges);
    }
}
