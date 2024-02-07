pub fn process_part_1(input: &str) -> i32 {
    input.chars().fold(0, |acc, c| {
        acc + match c {
            '(' => 1,
            ')' => -1,
            char => panic!("invalid input: {{{}}}", char),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("(())", 0)]
    #[case("()()", 0)]
    #[case("(((", 3)]
    #[case("(()(()(", 3)]
    #[case("))(((((", 3)]
    #[case("())", -1)]
    #[case("))(", -1)]
    #[case(")))", -3)]
    #[case(")())())", -3)]
    fn line_test(#[case] line: &str, #[case] expected: i32) {
        let result = process_part_1(line);
        assert_eq!(result, expected);
    }
}
