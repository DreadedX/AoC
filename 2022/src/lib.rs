#![feature(test)]
extern crate test;

use core::fmt;
use std::{fs, fmt::Debug};

use anyhow::{Context, Result};

pub enum Part {
    ONE,
    TWO
}

pub trait Solver {
    type Output: fmt::Display + Debug + PartialEq;

    fn day() -> u8;
    fn part1(input: &str) -> Self::Output;
    fn part2(input: &str) -> Self::Output;

    fn select(part: Part) -> for<'a> fn(&'a str) -> <Self as Solver>::Output {
        match part {
            Part::ONE => Self::part1,
            Part::TWO => Self::part2,
        }
    }

    fn test(part: Part, name: &str, result: Self::Output) -> Result<()> {
        // Select the right function

        // Read the test input
        let input = fs::read_to_string(format!("input/{}/{name}", Self::day())).with_context(|| format!("Failed to read '{}' for day {}", name, Self::day()))?;

        // Assert that the result matches the expected value
        assert_eq!(Self::select(part)(&input), result);

        Ok(())
    }

    fn solve() -> Result<()> {
        let input = fs::read_to_string(format!("input/{}/input", Self::day())).with_context(|| format!("Failed to read 'input' for day {}", Self::day()))?;
        println!("Part 1: {}", Self::part1(&input));
        println!("Part 2: {}", Self::part2(&input));

        Ok(())
    }

    fn benchmark(part: Part, b: &mut test::Bencher) {
        let f = Self::select(part);

        b.iter(|| {
            let input = fs::read_to_string(format!("input/{}/input", Self::day())).unwrap();
            f(&input)
        });
    }
}
