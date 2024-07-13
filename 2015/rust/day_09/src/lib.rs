pub fn process_part_1(input: &str) -> u32 {
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
