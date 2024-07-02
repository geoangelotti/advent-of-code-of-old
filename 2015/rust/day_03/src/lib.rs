use std::collections::BTreeSet;

pub fn process_part_1(input: &str) -> u32 {
    let mut houses: BTreeSet<(i32, i32)> = BTreeSet::new();
    let mut house = (0, 0);
    houses.insert(house);
    for c in input.chars() {
        house = match c {
            '^' => (house.0, house.1 + 1),
            'v' => (house.0, house.1 - 1),
            '>' => (house.0 + 1, house.1),
            '<' => (house.0 - 1, house.1),
            char => panic!("invalid input: {{{}}}", char),
        };
        houses.insert(house);
    }
    houses.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(">", 2)]
    #[case("^>v<", 4)]
    #[case("^v^v^v^v^v", 2)]
    fn test_part_1(#[case] input: &str, #[case] expected: u32) {
        let result = process_part_1(input);
        assert_eq!(result, expected);
    }
}
