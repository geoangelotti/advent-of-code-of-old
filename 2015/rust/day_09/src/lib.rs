use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, newline},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

fn parse_locations(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(alpha1, tag(" to "), alpha1)(input)
}

fn parse_distance(input: &str) -> IResult<&str, (&str, &str, u32)> {
    let (input, locations) = parse_locations(input)?;
    let (input, distance) = preceded(tag(" = "), complete::u32)(input)?;
    Ok((input, (locations.0, locations.1, distance)))
}

fn parse_distances(input: &str) -> IResult<&str, Vec<(&str, &str, u32)>> {
    separated_list1(newline, parse_distance)(input)
}

pub fn process_part_1(input: &str) -> u32 {
    let (_, distances) = parse_distances(input).unwrap();
    dbg!(distances);
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";

    #[test]
    fn it_works() {
        let result = process_part_1(INPUT);
        assert_eq!(result, 605);
    }
}
