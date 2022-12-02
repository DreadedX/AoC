use std::fs;

pub enum Part {
    ONE,
    TWO
}

pub trait Solver {
    fn day() -> u8;
    fn part1(input: &str) -> u32;
    fn part2(input: &str) -> u32;

    fn test(part: Part) {
        // Select the right function
        let fun = match part {
            Part::ONE => Self::part1,
            Part::TWO => Self::part2,
        };

        // Read the test input
        let test = fs::read_to_string(format!("input/{}/test", Self::day())).expect("Test file does not exist!");

        // Get the correct test result
        let result: u32 = match test.split("\n")
            .skip(part as usize)
            .next()
            .expect("Expected second line to contain result for part1")
            .parse() {
                Ok(result) => result,
                _ => 0, // Use zero if no value is specified yet
            };

        // Get the input for the test
        // @TODO This creates a new string, would be nice if we could actually get a slice here
        let input = test.split("\n")
            .skip(2)
            .collect::<Vec<&str>>()
            .join("\n");

        // Assert that the result matches the expected value
        assert_eq!(fun(&input), result);
    }

    fn solve() {
        let input = fs::read_to_string(format!("input/{}/test", Self::day())).expect("Input file does not exist!");
        println!("Part 1: {}", Self::part1(&input));
        println!("Part 2: {}", Self::part2(&input));
    }
}
