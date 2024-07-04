fn contains_vowels(input: &str) -> bool {
    input
        .chars()
        .filter(|c| vec!['a', 'e', 'i', 'o', 'u'].contains(c))
        .count()
        > 2
}

fn contains_bad_strings(input: &str) -> bool {
    vec!["ab", "cd", "pq", "xy"]
        .into_iter()
        .any(|s| input.contains(s))
}

fn contains_duplicate(input: &str) -> bool {
    input
        .chars()
        .zip(input.chars().skip(1))
        .any(|(a, b)| a == b)
}

pub fn process_part_1(input: &str) -> u32 {
    input
        .lines()
        .into_iter()
        .filter(|&line| {
            let has_enough_vowels = contains_vowels(line);
            let has_bad_strings = contains_bad_strings(line);
            let has_duplicate = contains_duplicate(line);
            has_enough_vowels && has_duplicate && !has_bad_strings
        })
        .collect::<Vec<&str>>()
        .len() as u32
}

fn contains_pair(chars: &str) -> bool {
    for i in 0..chars.len() - 2 {
        let pair = &chars[i..i + 2];
        if chars[i + 2..].contains(pair) {
            return true;
        }
    }
    false
}

fn contains_repeating_with_seperator(input: &str) -> bool {
    let chars = input.chars().collect::<Vec<char>>();
    for i in 0..chars.len() - 2 {
        if chars[i] == chars[i + 2] {
            return true;
        }
    }
    false
}

pub fn process_part_2(input: &str) -> u32 {
    input
        .lines()
        .into_iter()
        .filter(|&line| {
            let has_repeating = contains_repeating_with_seperator(line);
            let has_pair = contains_pair(line);
            has_repeating && has_pair
        })
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("haegwjzuvuyypxyu", true)]
    #[case("jchzalrnumimnmhp", false)]
    #[case("dvszwmarrgswjxmb", false)]
    fn test_contains_bad_strings(#[case] input: &str, #[case] expected: bool) {
        let result = contains_bad_strings(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("aei", true)]
    #[case("xazegov", true)]
    #[case("aeiouaeiouaeiou", true)]
    fn test_contains_vowels(#[case] input: &str, #[case] expected: bool) {
        let result = contains_vowels(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("xx", true)]
    #[case("abcdde", true)]
    #[case("aabbccdd", true)]
    #[case("jchzalrnumimnmhp", false)]
    fn test_contains_duplicates(#[case] input: &str, #[case] expected: bool) {
        let result = contains_duplicate(input);
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

    #[rstest]
    #[case("xyxy", true)]
    #[case("aabcdefgaa", true)]
    #[case("aaa", false)]
    fn test_contains_pair(#[case] input: &str, #[case] expected: bool) {
        let result = contains_pair(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("xyx", true)]
    #[case("abcdefeghi", true)]
    #[case("aaa", true)]
    fn test_contains_repeating_with_seperator(#[case] input: &str, #[case] expected: bool) {
        let result = contains_repeating_with_seperator(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("qjhvhtzxzqqjkmpb", 1)]
    #[case("xxyxx", 1)]
    #[case("uurcxstgmygtbstg", 0)]
    #[case("ieodomkazucvgmuy", 0)]
    #[case("cvsrbdcvhtxxdmro", 0)]
    fn test_part_2(#[case] input: &str, #[case] expected: u32) {
        let result = process_part_2(input);
        assert_eq!(result, expected);
    }
}
