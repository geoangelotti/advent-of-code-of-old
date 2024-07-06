use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Clone, Copy, Debug)]
enum InputPort<'a> {
    Port(&'a str),
    Value(u16),
}

impl<'a> InputPort<'a> {
    fn resolve(&self, map: &BTreeMap<&str, Instruction>) -> u16 {
        match &self {
            InputPort::Port(key) => map.get(key).unwrap().execute(map),
            InputPort::Value(v) => *v,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Gate<'a> {
    Wire(InputPort<'a>),
    And(InputPort<'a>, InputPort<'a>),
    Or(InputPort<'a>, InputPort<'a>),
    Not(InputPort<'a>),
    Lshift(InputPort<'a>, u16),
    Rshift(InputPort<'a>, u16),
}

#[derive(Clone, Copy, Debug)]
struct Instruction<'a> {
    gate: Gate<'a>,
    output: &'a str,
}

impl<'a> Instruction<'a> {
    fn execute(&self, map: &BTreeMap<&'a str, Instruction>) -> u16 {
        let result = match &self.gate {
            Gate::Wire(a) => a.resolve(map),
            Gate::And(a, b) => a.resolve(map) & b.resolve(map),
            Gate::Or(a, b) => a.resolve(map) | b.resolve(map),
            Gate::Not(a) => !a.resolve(map),
            Gate::Lshift(a, b) => a.resolve(map) << b,
            Gate::Rshift(a, b) => a.resolve(map) >> b,
        };
        dbg!((self.output, result));
        result
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
        map(
            separated_pair(parse_port, tag(" AND "), parse_port),
            |(a, b)| Gate::And(a, b),
        ),
        map(
            separated_pair(parse_port, tag(" OR "), parse_port),
            |(a, b)| Gate::Or(a, b),
        ),
        map(
            separated_pair(parse_port, tag(" LSHIFT "), complete::u16),
            |(a, b)| Gate::Lshift(a, b),
        ),
        map(
            separated_pair(parse_port, tag(" RSHIFT "), complete::u16),
            |(a, b)| Gate::Rshift(a, b),
        ),
    ))(input)
}

fn parse_not_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, input_1) = preceded(tag("NOT "), parse_port)(input)?;
    let (input, output) = preceded(tag(" -> "), alpha1)(input)?;
    Ok((
        input,
        Instruction {
            gate: Gate::Not(input_1),
            output,
        },
    ))
}

fn parse_wire_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, (input_1, output)) = separated_pair(parse_port, tag(" -> "), alpha1)(input)?;
    Ok((
        input,
        Instruction {
            gate: Gate::Wire(input_1),
            output,
        },
    ))
}

fn parse_instruction_2(input: &str) -> IResult<&str, Instruction> {
    let (input, gate) = parse_gate_2(input)?;
    let (input, output) = preceded(tag(" -> "), alpha1)(input)?;
    Ok((input, Instruction { gate, output }))
}

fn parse_line(input: &str) -> IResult<&str, Instruction> {
    alt((
        parse_instruction_2,
        parse_wire_instruction,
        parse_not_instruction,
    ))(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(newline, parse_line)(input)
}

pub fn process_part_1(input: &str, key: &str) -> u16 {
    let mut map = BTreeMap::new();
    let (_, instuctions) = parse_input(input).unwrap();
    for instruction in instuctions {
        map.insert(instruction.output, instruction);
    }
    map.get(key).unwrap().execute(&map)
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
        let result = process_part_1(input, key);
        assert_eq!(result, expected);
    }
}
