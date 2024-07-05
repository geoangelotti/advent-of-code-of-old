use std::collections::BTreeMap;

fn process_part_1(input: &str) -> BTreeMap<&str, u16> {
    let mut map = BTreeMap::new();
    return map;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const INSTRUCTIONS: &str = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";

    #[rstest]
    #[case(INSTRUCTIONS, "d", 72)]
    #[case(INSTRUCTIONS, "e", 507)]
    #[case(INSTRUCTIONS, "f", 492)]
    #[case(INSTRUCTIONS, "g", 114)]
    #[case(INSTRUCTIONS, "h", 65412)]
    #[case(INSTRUCTIONS, "i", 65079)]
    #[case(INSTRUCTIONS, "x", 123)]
    #[case(INSTRUCTIONS, "y", 456)]
    fn test_process_part_1(#[case] input: &str, #[case] key: &str, #[case] expected: u16) {
        let result = process_part_1(input);
        assert_eq!(*result.get(key).unwrap(), expected);
    }
}
