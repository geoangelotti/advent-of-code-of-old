pub fn process_part_1(input: &str) -> u32 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const COMMANDS: &str = "turn on 0,0 through 999,999
toggle 0,0 through 999,0
turn off 499,499 through 500,500";

    #[rstest]
    #[case(COMMANDS, 998996)]
    fn test_procces_part_1(#[case] input: &str, #[case] expected: u32) {
        let result = process_part_1(input);
        assert_eq!(result, expected);
    }
}
