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
    //dbg!((input, &unescaped), (input.len(), &unescaped.len()));
    (input.len(), unescaped.len())
}

pub fn process_part_1(input: &str) -> u32 {
    input
        .lines()
        .map(difference)
        .map(|(a, b)| a - b)
        .sum::<usize>() as u32
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
