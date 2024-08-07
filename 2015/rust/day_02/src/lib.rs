use itertools::Itertools;

fn parse_dimensions(line: &str) -> (u32, u32, u32) {
    line.split('x')
        .map(|num| num.parse::<u32>().unwrap())
        .sorted()
        .collect_tuple()
        .unwrap()
}

pub fn process_part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let dimensions: (u32, u32, u32) = parse_dimensions(line);
            dimensions.0 * dimensions.1
                + 2 * dimensions.0 * dimensions.1
                + 2 * dimensions.0 * dimensions.2
                + 2 * dimensions.1 * dimensions.2
        })
        .sum::<u32>()
}

pub fn process_part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let dimensions: (u32, u32, u32) = parse_dimensions(line);
            2 * dimensions.0 + 2 * dimensions.1 + dimensions.0 * dimensions.1 * dimensions.2
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("2x3x4", 58)]
    #[case("1x1x10", 43)]
    fn test_part_1(#[case] input: &str, #[case] expected: u32) {
        let result = process_part_1(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("2x3x4", 34)]
    #[case("1x1x10", 14)]
    fn test_part_2(#[case] input: &str, #[case] expected: u32) {
        let result = process_part_2(input);
        assert_eq!(result, expected);
    }
}
