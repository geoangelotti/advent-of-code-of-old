fn floor_resolve(c: char) -> i32 {
    match c {
        '(' => 1,
        ')' => -1,
        char => panic!("invalid input: {{{}}}", char),
    }
}

pub fn process_part_1(input: &str) -> i32 {
    input.chars().fold(0, |acc, c| acc + floor_resolve(c))
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
    fn test_part_1(#[case] line: &str, #[case] expected: i32) {
        let result = process_part_1(line);
        assert_eq!(result, expected);
    }
}
