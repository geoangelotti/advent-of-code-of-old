fn unescape(input: &str) -> String {
    let mut unescaped = String::new();
    let mut chars = input[1..input.len() - 1].chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\\' {
            if let Some(next_char) = chars.peek() {
                match next_char {
                    '\\' => {
                        chars.next();
                        unescaped.push('\\');
                    }
                    '"' => {
                        chars.next();
                        unescaped.push('"');
                    }
                    'x' => {
                        chars.next();
                        let mut hex = String::new();
                        if let Some(first) = chars.next() {
                            hex.push(first);
                        }
                        if let Some(second) = chars.next() {
                            hex.push(second);
                        }
                        if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                            unescaped.push(byte as char);
                        }
                    }
                    _ => {
                        unescaped.push(c);
                    }
                }
            }
        } else {
            unescaped.push(c);
        }
    }
    unescaped
}

fn difference(input: &str) -> (usize, usize) {
    let unescaped = unescape(input);
    (input.len(), unescaped.encode_utf16().count())
}

pub fn process_part_1(input: &str) -> u32 {
    input
        .lines()
        .map(difference)
        .map(|(a, b)| a - b)
        .sum::<usize>() as u32
}

pub fn process_part_2(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const INPUTS: [&str; 5] = [
        r#""""#,
        r#""abc""#,
        r#""aaa\"aaa""#,
        r#""\x27""#,
        r#""e\xcef\\hkiu""#,
    ];

    #[rstest]
    #[case(INPUTS[0], r#""#)]
    #[case(INPUTS[1], r#"abc"#)]
    #[case(INPUTS[2], r#"aaa"aaa"#)]
    #[case(INPUTS[3], r"'")]
    #[case(INPUTS[4], r#"eÎf\hkiu"#)]
    fn test_unespace(#[case] input: &str, #[case] expected: &str) {
        let result = unescape(input);
        assert_eq!(result, expected);
        assert_eq!(result.len(), expected.len())
    }

    #[rstest]
    #[case(INPUTS[0], 2)]
    #[case(INPUTS[1], 2)]
    #[case(INPUTS[2], 3)]
    #[case(INPUTS[3], 5)]
    #[case(INPUTS[4], 6)]
    fn test_procces_part_1(#[case] input: &str, #[case] expected: u32) {
        let result = process_part_1(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(INPUTS[0], 4)]
    #[case(INPUTS[1], 4)]
    #[case(INPUTS[2], 6)]
    #[case(INPUTS[3], 5)]
    fn test_procces_part_2(#[case] input: &str, #[case] expected: u32) {
        let result = process_part_2(input);
        assert_eq!(result, expected);
    }
}
