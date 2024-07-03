pub fn process_part_1(input: &str) -> u32 {
    let mut i = 0;
    loop {
        let hash = md5::compute(format!("{}{}", input, i));
        let prefix = &format!("{:?}", hash)[0..5];
        if prefix == "00000" {
            break;
        }
        i += 1;
    }
    return i;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("abcdef", 609043)]
    #[case("pqrstuv", 1048970)]
    fn test_part_1(#[case] input: &str, #[case] expected: u32) {
        let result = process_part_1(input);
        assert_eq!(result, expected);
    }
}
