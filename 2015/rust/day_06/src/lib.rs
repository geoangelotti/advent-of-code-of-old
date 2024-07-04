use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

#[derive(Clone, Copy, Debug)]
enum Action {
    Toggle,
    TurnOn,
    TurnOff,
}

#[derive(Clone, Copy, Debug)]
struct Command {
    action: Action,
    start: (u32, u32),
    end: (u32, u32),
}

fn parse_action(input: &str) -> IResult<&str, Action> {
    let (input, action) = alt((
        tag("toggle").map(|_| Action::Toggle),
        tag("turn on").map(|_| Action::TurnOn),
        tag("turn off").map(|_| Action::TurnOff),
    ))(input)?;
    Ok((input, action))
}

fn parse_point(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, point) = separated_pair(complete::u32, tag(","), complete::u32)(input)?;
    Ok((input, point))
}

fn parse_points(input: &str) -> IResult<&str, ((u32, u32), (u32, u32))> {
    let (input, points) = separated_pair(parse_point, tag(" through "), parse_point)(input)?;
    Ok((input, points))
}

fn parse_line(input: &str) -> IResult<&str, Command> {
    let (input, (action, (start, end))) =
        separated_pair(parse_action, tag(" "), parse_points)(input)?;
    Ok((input, Command { action, start, end }))
}

fn parse_lines(input: &str) -> IResult<&str, Vec<Command>> {
    let (input, commands) = separated_list1(newline, parse_line)(input)?;
    Ok((input, commands))
}

pub fn process_part_1(input: &str) -> u32 {
    let (_, commands) = parse_lines(input).unwrap();
    let mut grid: Vec<Vec<bool>> = vec![vec![false; 1000]; 1000];
    for command in commands {
        //dbg!(command.start.0..command.end.0);
        for i in command.start.0..command.end.0 + 1 {
            //dbg!(command.start.1..command.end.1);
            for j in command.start.1..command.end.1 + 1 {
                grid[i as usize][j as usize] = match command.action {
                    Action::Toggle => !grid[i as usize][j as usize],
                    Action::TurnOn => true,
                    Action::TurnOff => false,
                }
            }
        }
    }
    grid.into_iter()
        .map(|vec| vec.into_iter().filter(|&a| a).count())
        .sum::<usize>() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const COMMANDS: &str = "turn on 0,0 through 999,999
toggle 0,0 through 999,0
turn off 499,499 through 500,500";

    #[rstest]
    #[case(COMMANDS, 998996)]
    fn test_procces_part_1(#[case] input: &str, #[case] expected: u32) {
        let result = process_part_1(input);
        assert_eq!(result, expected);
    }
}
