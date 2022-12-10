#![feature(test)]
use core::fmt;
use std::{collections::HashMap, str::FromStr};
use lazy_static::lazy_static;

use regex::Regex;
use anyhow::Result;
use aoc::Solver;

// -- Runners --
fn main() -> Result<()> {
    Day::solve()
}

// -- Tests --
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() -> Result<()> {
        Day::test(Day::part1, "test-1", "CMZ".to_string())
    }
    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", "RNZLFZSJH".to_string())
    }
    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(Day::part2, "test-1", "MCD".to_string())
    }
    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", "CNSFCGJSM".to_string())
    }

    // Run it on a 6MB file to see how it performs
    // https://www.reddit.com/r/adventofcode/comments/zd1hqy/2022_day_5_i_know_i_am_overthinking_it/iyzvsnp/?context=3
    #[test]
    #[ignore]
    fn part1_large() -> Result<()> {
        Day::test(Day::part1, "large", "GATHERING".to_string())
    }
    #[test]
    #[ignore]
    fn part2_large() -> Result<()> {
        Day::test(Day::part2, "large", "DEVSCHUUR".to_string())
    }
}

// -- Benchmarks --
#[cfg(test)]
mod bench {
    use super::*;

    // Benchmarks
    extern crate test;
    #[bench]
    #[ignore]
    fn part1_solution(b: &mut test::Bencher) {
        Day::benchmark(Day::part1, b)
    }
    #[bench]
    #[ignore]
    fn part2_solution(b: &mut test::Bencher) {
        Day::benchmark(Day::part2, b)
    }
}

// -- Implementation --
#[derive(Debug)]
struct Crate{
    letter: char,
}

#[derive(Debug)]
struct Boat{
    stacks: HashMap<usize, Vec<Crate>>,
}

impl fmt::Display for Boat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.stacks.len() {
            match self.top(i+1) {
                Some(cr) => write!(f, "{}", cr.letter)?,
                None => write!(f, " ")?,
            }
        }

        Ok(())
    }
}

impl FromStr for Boat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\[[A-Z]\]").unwrap();
        }

        let mut boat = Self{stacks: HashMap::new()};

        s.lines().rev().for_each(|line| {
            for mat in RE.find_iter(line) {
                let index = mat.start() / 4 + 1;
                // based on the pattern [.] we get the second char
                let letter = mat.as_str().chars().nth(1).unwrap();

                boat.push(index, Crate{letter});
            }
        });

        Ok(boat)
    }
}

impl Boat {
    fn take(&mut self, index: usize, amount: usize) -> Vec<Crate> {
        let stack = self.stacks.entry(index).or_default();
        stack.split_off(stack.len()-amount as usize)
    }

    fn put(&mut self, index: usize, mut vec: Vec<Crate>) {
        let stack = self.stacks.entry(index).or_default();
        stack.append(&mut vec);
    }

    fn push(&mut self, index: usize, cr: Crate) {
        let stack = self.stacks.entry(index).or_default();
        stack.push(cr);
    }

    fn top(&self, index: usize) -> Option<&Crate> {
        let stack = match self.stacks.get(&index) {
            Some(s) => s,
            None => return None,
        };

        stack.last()
    }
}

struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"move (?P<amount>[0-9]+) from (?P<from>[0-9]) to (?P<to>[0-9])").unwrap();
        }

        let capture = RE.captures(s).unwrap();

        let parse = |name| capture.name(name).unwrap().as_str().parse();

        let amount = parse("amount")?;
        let from = parse("from")?;
        let to = parse("to")?;

        Ok(Instruction{amount, from, to})

    }
}

// -- Helpers --
fn solution(input: &str, part1: bool) -> String {
    let (boat, moves) = input.split_once("\n\n").expect("Input is invalid");

    let mut boat: Boat = boat.parse().expect("Invalid input");

    moves.lines()
        .map(|line| line.parse().expect("Invalid input"))
        .for_each(|i: Instruction| {
            let mut taken = boat.take(i.from, i.amount);
            if part1 {
                // In part one we move the crates on by one, so the vector needs to be reverse to
                // get the correct order
                taken.reverse();
            }
            boat.put(i.to, taken);
        });

    boat.to_string()
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = String;
    type Output2 = String;
    fn day() -> u8 {
        5
    }

    fn part1(input: &str) -> Self::Output1 {
        solution(input, true)
    }

    fn part2(input: &str) -> Self::Output2 {
        solution(input, false)
    }
}
