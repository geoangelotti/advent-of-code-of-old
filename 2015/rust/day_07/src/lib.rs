use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

#[derive(Clone, Copy, Debug)]
enum InputPort<'a> {
    Port(&'a str),
    Value(u16),
}

impl<'a> InputPort<'a> {
    fn resolve(&self, map: &BTreeMap<&str, u16>) -> u16 {
        match &self {
            InputPort::Port(key) => *map.get(key).unwrap_or(&0),
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

fn parse_port(input: &str) -> IResult<&str, InputPort> {
    alt((
        map(complete::u16, InputPort::Value),
        map(alpha1, InputPort::Port),
    ))(input)
}

fn parse_gate_2(input: &str) -> IResult<&str, Gate> {
    alt((
        tag(" AND ").map(|_| Gate::AND),
        tag(" OR ").map(|_| Gate::OR),
        tag(" LSHIFT ").map(|_| Gate::LSHIFT),
        tag(" RSHIFT ").map(|_| Gate::RSHIFT),
    ))(input)
}

fn parse_not_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, input_1) = preceded(tag("NOT "), parse_port)(input)?;
    let (input, output) = preceded(tag(" -> "), alpha1)(input)?;
    Ok((
        input,
        Instruction {
            gate: Gate::NOT,
            input: [input_1, InputPort::Value(0)],
            output,
        },
    ))
}

fn parse_arrow_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, (input_1, output)) = separated_pair(parse_port, tag(" -> "), alpha1)(input)?;
    Ok((
        input,
        Instruction {
            gate: Gate::ARROW,
            input: [input_1, InputPort::Value(0)],
            output,
        },
    ))
}

fn parse_instruction_2(input: &str) -> IResult<&str, Instruction> {
    let (input, input_1) = parse_port(input)?;
    let (input, gate) = parse_gate_2(input)?;
    let (input, input_2) = parse_port(input)?;
    let (input, output) = preceded(tag(" -> "), alpha1)(input)?;
    Ok((
        input,
        Instruction {
            gate,
            input: [input_1, input_2],
            output,
        },
    ))
}

fn parse_line(input: &str) -> IResult<&str, Instruction> {
    alt((
        parse_instruction_2,
        parse_arrow_instruction,
        parse_not_instruction,
    ))(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(newline, parse_line)(input)
}

pub fn process_part_1(input: &str) -> BTreeMap<&str, u16> {
    let mut map = BTreeMap::new();
    let (_, instuctions) = parse_input(input).unwrap();
    dbg!(instuctions);
    map
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
