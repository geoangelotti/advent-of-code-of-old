fn has_enough_vowels(input: &str) -> bool {
    input
        .chars()
        .filter(|c| vec!['a', 'e', 'i', 'o', 'u'].contains(c))
        .count()
        > 2
}

pub fn process_part_1(input: &str) -> u32 {
    let bad_strings = vec!["ab", "cd", "pq", "xy"];
    input
        .lines()
        .into_iter()
        .filter(|line| {
            let has_enough_vowels = has_enough_vowels(input);
            let has_bad_strings = bad_strings
                .clone()
                .into_iter()
                .map(|s| line.contains(s))
                .any(|a| a);
            let has_duplicate = line.chars().zip(line.chars().skip(1)).any(|(a, b)| a == b);
            has_enough_vowels && has_duplicate && !has_bad_strings
        })
        .collect::<Vec<&str>>()
        .len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("aei", true)]
    #[case("xazegov", true)]
    #[case("aeiouaeiouaeiou", true)]
    fn test_has_enough_vowels(#[case] input: &str, #[case] expected: bool) {
        let result = has_enough_vowels(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("ugknbfddgicrmopn", 1)]
    #[case("aaa", 1)]
    #[case("jchzalrnumimnmhp", 0)]
    #[case("haegwjzuvuyypxyu", 0)]
    #[case("dvszwmarrgswjxmb", 0)]
    fn test_part_1(#[case] input: &str, #[case] expected: u32) {
        let result = process_part_1(input);
        assert_eq!(result, expected);
    }
}
