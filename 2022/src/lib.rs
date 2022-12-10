#![feature(test)]
extern crate test;

use core::fmt;
use std::{fs, fmt::Debug};

use anyhow::{Context, Result};

pub trait Solver {
    type Output1: fmt::Display + Debug + PartialEq;
    type Output2: fmt::Display + Debug + PartialEq;

    fn day() -> u8;
    fn part1(input: &str) -> Self::Output1;
    fn part2(input: &str) -> Self::Output2;

    fn test<T: Fn(&str) -> U, U: Debug + PartialEq>(f: T, name: &str, result: U) -> Result<()> {
        // Select the right function

        // Read the test input
        let input = fs::read_to_string(format!("input/{}/{name}", Self::day())).with_context(|| format!("Failed to read '{}' for day {}", name, Self::day()))?;

        // Assert that the result matches the expected value
        assert_eq!(f(&input), result);

        Ok(())
    }

    fn solve() -> Result<()> {
        let input = fs::read_to_string(format!("input/{}/input", Self::day())).with_context(|| format!("Failed to read 'input' for day {}", Self::day()))?;
        println!("Part 1:\n{}", Self::part1(&input));
        println!("Part 2:\n{}", Self::part2(&input));

        Ok(())
    }

    fn benchmark<T: Fn(&str) -> U, U: Debug + PartialEq>(f: T, b: &mut test::Bencher) {
        let input = fs::read_to_string(format!("input/{}/input", Self::day())).with_context(|| format!("Failed to read 'input' for day {}", Self::day())).unwrap();

        b.iter(|| {
            f(&input)
        });
    }
}
