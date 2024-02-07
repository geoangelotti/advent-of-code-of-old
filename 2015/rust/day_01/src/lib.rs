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

pub fn process_part_2(input: &str) -> usize {
    let mut acc = 0;
    let mut index = 0;
    for (i, c) in input.chars().enumerate().into_iter() {
        acc += floor_resolve(c);
        if acc == -1 {
            index = i;
            break;
        }
    }
    index + 1
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

    #[rstest]
    #[case(")", 1)]
    #[case("()())", 5)]
    fn test_part_2(#[case] line: &str, #[case] expected: usize) {
        let result = process_part_2(line);
        assert_eq!(result, expected);
    }
}
