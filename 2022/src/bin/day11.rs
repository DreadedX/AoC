#![feature(test)]
use std::str::FromStr;

use anyhow::Result;
use aoc::Solver;

// -- Runners --
fn main() -> Result<()> {
    Day::solve()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() -> Result<()> {
        Day::test(Day::part1, "test-1", 10605)
    }
    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 95472)
    }
    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(Day::part2, "test-1", 2713310158)
    }
    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", 17926061332)
    }

    // Benchmarks
    extern crate test;
    #[bench]
    #[ignore]
    fn part1_bench(b: &mut test::Bencher) {
        Day::benchmark(Day::part1, b)
    }
    #[bench]
    #[ignore]
    fn part2_bench(b: &mut test::Bencher) {
        Day::benchmark(Day::part2, b)
    }
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Add(i64),
    AddOld,
    Multiply(i64),
    MultiplyOld,
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let operation = s.split(" ").collect::<Vec<_>>();

        match operation.as_slice() {
            ["old", "*", "old"] => Ok(Operation::MultiplyOld),
            ["old", "*", value] => Ok(Operation::Multiply(value.parse()?)),
            ["old", "+", "old"] => Ok(Operation::AddOld),
            ["old", "+", value] => Ok(Operation::Add(value.parse()?)),
            _ => Err(anyhow::anyhow!("Invalid input")),
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    divisor: i64,
    next: (usize, usize),
    inspects: i64,
}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();
        let items = lines[1].split_once(": ").unwrap().1.split(", ").map(|num| num.parse().unwrap()).collect();

        let operation: Operation = lines[2].split_once("= ").unwrap().1.parse()?;

        let divisor = lines[3].rsplit_once(" ").unwrap().1.parse()?;

        let next_true = lines[4].rsplit_once(" ").unwrap().1.parse()?;
        let next_false = lines[5].rsplit_once(" ").unwrap().1.parse()?;

        Ok(Self {items, operation, divisor, next: (next_true, next_false), inspects: 0})
    }
}

fn step(monkeys: &mut Vec<Monkey>, div: i64, reducer: i64) {
    for idx in 0..monkeys.len() {
        let items = std::mem::take(&mut monkeys[idx].items);
        for item in items {
            // Calculate the new item value
            let mut i = match monkeys[idx].operation {
                Operation::Add(v) => item + v,
                Operation::AddOld => item + item,
                Operation::Multiply(v) => item * v,
                Operation::MultiplyOld => item * item,
            } / div;

            i %= reducer;

            // Increase the inspect counter for this monkey
            monkeys[idx].inspects += 1;

            let next;
            if i % monkeys[idx].divisor == 0 {
                next = monkeys[idx].next.0;
            } else {
                next = monkeys[idx].next.1;
            }
            monkeys[next].items.push(i);
        }
    }
}

fn solution(input: &str, rounds: i32, div: i64) -> i64 {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(|line| line.parse().unwrap()).collect();

    let reducer = monkeys.iter().fold(1, |acc, monkey| acc * monkey.divisor);

    for _ in 0..rounds {
        step(&mut monkeys, div, reducer);
    }

    let mut inspects = monkeys.iter().map(|monkey| monkey.inspects).collect::<Vec<_>>();
    inspects.sort();

    inspects.into_iter().rev().take(2).reduce(|acc, inspects| acc * inspects).unwrap()
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = i64;
    type Output2 = i64;

    fn day() -> u8 {
        11
    }

    fn part1(input: &str) -> Self::Output1 {
        solution(input, 20, 3)
    }

    fn part2(input: &str) -> Self::Output2 {
        solution(input, 10000, 1)
    }
}
