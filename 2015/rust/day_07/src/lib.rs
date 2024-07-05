use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug)]
enum InputPort<'a> {
    Port(&'a str),
    Value(u16),
}

impl<'a> InputPort<'a> {
    fn resolve(&self, map: &BTreeMap<&str, u16>) -> u16 {
        match &self {
            InputPort::Port(key) => *map.get(key).unwrap(),
            InputPort::Value(v) => *v,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Gate {
    ARROW,
    AND,
    OR,
    NOT,
    LSHIFT,
    RSHIFT,
}

#[derive(Clone, Copy, Debug)]
struct Instruction<'a> {
    gate: Gate,
    input: [InputPort<'a>; 2],
    output: &'a str,
}

impl<'a> Instruction<'a> {
    fn execute(&self, map: &mut BTreeMap<&'a str, u16>) {
        let a = self.input[0].resolve(map);
        let b = self.input[1].resolve(map);
        let c = match &self.gate {
            Gate::ARROW => a,
            Gate::AND => a & b,
            Gate::OR => a | b,
            Gate::NOT => !a,
            Gate::LSHIFT => a << b,
            Gate::RSHIFT => a >> b,
        };
        map.insert(self.output, c);
    }
}

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
