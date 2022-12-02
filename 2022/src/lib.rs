use std::fs;

pub enum Part {
    ONE,
    TWO
}

pub trait Solver {
    fn day() -> u8;
    fn part1(input: &str) -> u32;
    fn part2(input: &str) -> u32;

    fn test(part: Part, name: &str, result: u32) {
        // Select the right function
        let fun = match part {
            Part::ONE => Self::part1,
            Part::TWO => Self::part2,
        };

        // Read the test input
        let input = fs::read_to_string(format!("input/{}/{name}", Self::day())).expect("Test file does not exist!");

        // Assert that the result matches the expected value
        assert_eq!(fun(&input), result);
    }

    fn solve() {
        let input = fs::read_to_string(format!("input/{}/input", Self::day())).expect("Input file does not exist!");
        println!("Part 1: {}", Self::part1(&input));
        println!("Part 2: {}", Self::part2(&input));
    }
}
