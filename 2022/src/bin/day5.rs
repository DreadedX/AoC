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
        Day::test(aoc::Part::ONE, "test-1", "CMZ".to_string())
    }
    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(aoc::Part::ONE, "input", "RNZLFZSJH".to_string())
    }
    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(aoc::Part::TWO, "test-1", "MCD".to_string())
    }
    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(aoc::Part::TWO, "input", "CNSFCGJSM".to_string())
    }

    // Run it on a 6MB file to see how it performs
    // https://www.reddit.com/r/adventofcode/comments/zd1hqy/2022_day_5_i_know_i_am_overthinking_it/iyzvsnp/?context=3
    #[test]
    #[ignore]
    fn part1_large() -> Result<()> {
        Day::test(aoc::Part::ONE, "large", "GATHERING".to_string())
    }
    #[test]
    #[ignore]
    fn part2_large() -> Result<()> {
        Day::test(aoc::Part::TWO, "large", "DEVSCHUUR".to_string())
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
        Day::benchmark(aoc::Part::ONE, b)
    }
    #[bench]
    #[ignore]
    fn part2_solution(b: &mut test::Bencher) {
        Day::benchmark(aoc::Part::TWO, b)
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

impl From<Vec<&str>> for Boat {
    fn from(lines: Vec<&str>) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\[[A-Z]\]").unwrap();
        }

        let mut boat = Self{stacks: HashMap::new()};
        for line in lines {
            for mat in RE.find_iter(line) {
                let index = mat.start() / 4 + 1;
                // based on the pattern [.] we get the second char
                let letter = mat.as_str().chars().nth(1).unwrap();

                boat.push(index, Crate{letter});
            }
        }

        // Because of how we load the data, each stack is upside down
        for stack in boat.stacks.iter_mut() {
            stack.1.reverse()
        }

        return boat;
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
    // The current layout description ends with an empty line
    let mut boat: Boat = input.lines()
                             .take_while(|line| !line.is_empty())
                             .collect::<Vec<_>>()
                             .into();

    // Each instruction starts with an 'm'
    input
        .lines()
        .skip_while(|line| !line.starts_with('m'))
        .map(|line| line.parse().unwrap())
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
    type Output = String;
    fn day() -> u8 {
        5
    }

    fn part1(input: &str) -> Self::Output {
        solution(input, true)
    }

    fn part2(input: &str) -> Self::Output {
        solution(input, false)
    }
}
