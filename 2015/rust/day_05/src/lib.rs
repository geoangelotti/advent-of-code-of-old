use std::collections::BTreeSet;

pub fn process_part_1(input: &str) -> u32 {
    let vowels = vec!['a', 'e', 'i', 'o', 'u']
        .into_iter()
        .collect::<BTreeSet<char>>();
    let bad_strings = vec!["ab", "cd", "pq", "xy"];
    input
        .lines()
        .into_iter()
        .filter(|line| {
            let has_enough_vowels = line
                .chars()
                .filter(|c| vowels.contains(c))
                .collect::<Vec<char>>()
                .len()
                > 2;
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

pub fn process_part_2(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

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

    #[rstest]
    #[case("qjhvhtzxzqqjkmpb", 1)]
    #[case("xxyxx", 1)]
    #[case("uurcxstgmygtbstg", 0)]
    #[case("ieodomkazucvgmuy", 0)]
    fn test_part_2(#[case] input: &str, #[case] expected: u32) {
        let result = process_part_2(input);
        assert_eq!(result, expected);
    }
}
