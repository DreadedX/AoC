use core::fmt;
use std::fs;

use anyhow::{Context, Result};

pub enum Part {
    ONE,
    TWO
}

#[derive(Debug,PartialEq, PartialOrd)]
pub enum Output {
    Number(u32),
    String(String),
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Output::Number(i) => write!(f, "{}", i)?,
            Output::String(s) => write!(f, "{}", s)?,
            _ => panic!("fmt::Display not implemented")
        }

        Ok(())
    }
}

pub trait Solver {
    fn day() -> u8;
    fn part1(input: &str) -> Output;
    fn part2(input: &str) -> Output;

    fn test(part: Part, name: &str, result: Output) -> Result<()> {
        // Select the right function
        let fun = match part {
            Part::ONE => Self::part1,
            Part::TWO => Self::part2,
        };

        // Read the test input
        let input = fs::read_to_string(format!("input/{}/{name}", Self::day())).with_context(|| format!("Failed to read '{}' for day {}", name, Self::day()))?;

        // Assert that the result matches the expected value
        assert_eq!(fun(&input), result);

        Ok(())
    }

    fn solve() -> Result<()> {
        let input = fs::read_to_string(format!("input/{}/input", Self::day())).with_context(|| format!("Failed to read 'input' for day {}", Self::day()))?;
        println!("Part 1: {}", Self::part1(&input));
        println!("Part 2: {}", Self::part2(&input));

        Ok(())
    }
}
