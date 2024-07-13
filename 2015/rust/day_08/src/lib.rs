pub fn process_part_1(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(r#""""#, 2)]
    #[case(r#""abc""#, 2)]
    #[case(r#""aaa\"aaa""#, 3)]
    #[case(r#""\x27""#, 5)]
    fn test_procces_part_1(#[case] input: &str, #[case] expected: u32) {
        let result = process_part_1(input);
        assert_eq!(result, expected);
    }
}
