#[cfg(test)]
mod tests {
    use rstest::*;
    use crate::splitter::run_test_case;
    use std::ops::Range;
    #[rstest]
    #[case(
        r#"
fn main() { 
    println!("Hello, world!");
}
"#,
        vec![(0, "function.definition")],
        vec![1..3],
    )]
    #[case(
        r#"
pub struct Test {
    pub a: i32,
    pub b: i32,
}
"#,
        vec![(0, "struct.definition")],
        vec![1..4],
    )]
    #[case(
        r#"
pub enum Test {
    A,
    B,
}
"#,
        vec![(0, "enum.definition")],
        vec![1..4],
    )]
    #[case(
        r#"
impl Test {
    pub fn a(&self) {
        println!("Hello, world!");
    }
}
"#,
        vec![(0, "class.definition"), (0, "method.definition")],
        vec![1..5, 2..4],
    )]
    #[case(
        r#"
/// this is a test
impl Test {
    /// this is a test
    pub fn a(&self) {
        println!("Hello, world!");
    }
    
    pub fn b() {
        println!("Hello, world!");
    }
}
"#,
        vec![(0, "class.definition"), (0, "method.comment"), (0, "method.definition"), (1, "method.definition")],
        vec![2..11, 3..4, 4..6, 8..10],
    )]
    #[case(
        r#"
trait Test {
    fn a(&self);
}

trait Test2 {
    fn b(&self);
}
"#,
        vec![(0, "class.definition"),(0, "method.definition"), (1, "class.definition"), (1, "method.definition")],
        vec![1..3, 2..2, 5..7, 6..6],
    )]
    fn test_rust_query_captures(
        #[case] code: &str,
        #[case] capture_names: Vec<(usize, &str)>,
        #[case] line_ranges: Vec<Range<usize>>,
    ) {
        run_test_case("test.rs", code, capture_names, line_ranges);
    }
}