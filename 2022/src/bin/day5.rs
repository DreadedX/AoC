use core::fmt;
use std::collections::HashMap;
use lazy_static::lazy_static;

use regex::Regex;
use anyhow::Result;
use aoc::{Solver, Output};

// -- Runners --
fn main() -> Result<()> {
    Day::solve()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() -> Result<()> {
        Day::test(aoc::Part::ONE, "test-1", Output::String("CMZ".to_string()))
    }
    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(aoc::Part::TWO, "test-1", Output::String("MCD".to_string()))
    }
    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(aoc::Part::ONE, "input", Output::String("RNZLFZSJH".to_string()))
    }
    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(aoc::Part::TWO, "input", Output::String("CNSFCGJSM".to_string()))
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

impl Boat {
    fn new(lines: &Vec<&str>) -> Boat {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\[[A-Z]\]").unwrap();
        }

        let mut boat = Boat{stacks: HashMap::new()};
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

    fn take(&mut self, index: usize, amount: usize) -> Vec<Crate> {
        let stack = self.stacks.entry(index).or_default();
        stack.split_off(stack.len()-amount)
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

// -- Helpers --
fn parse_instruction(s: &str) -> (usize, usize, usize) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"move (?P<amount>[0-9]+) from (?P<from>[0-9]) to (?P<to>[0-9])").unwrap();
    }

    let capture = RE.captures(s).unwrap();

    let parse_number = |name| capture.name(name).unwrap().as_str().parse::<usize>().unwrap();
    let amount: usize = parse_number("amount");
    let from: usize = parse_number("from");
    let to: usize = parse_number("to");

    (amount, from, to)
}

fn solution(input: &str, reverse: bool) -> Output {
    // The current layout description ends with an empty line
    let mut boat = Boat::new(&input
                             .lines()
                             .take_while(|line| !line.is_empty())
                             .collect());

    // Each instruction starts with an 'm'
    input
        .lines()
        .skip_while(|line| !line.starts_with('m'))
        .map(parse_instruction)
        .for_each(|(amount, from, to)| {
            let mut taken = boat.take(from, amount);
            // The order needs to be reversed in part due to the crates being moves one at the time
            // instead of the whole stack at once as happens in part 2
            if reverse {
                taken.reverse();
            }
            boat.put(to, taken);
        });

    Output::String(boat.to_string())
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    fn day() -> u8 {
        5
    }

    fn part1(input: &str) -> Output {
        solution(input, true)
    }

    fn part2(input: &str) -> Output {
        solution(input, false)
    }
}
