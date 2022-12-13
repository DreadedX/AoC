#![feature(test)]
use core::fmt;
use std::{str::FromStr, cmp::{min, Ordering}};

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
        Day::test(Day::part1, "test-1", 13)
    }
    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 5529)
    }
    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(Day::part2, "test-1", 140)
    }
    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", 27690)
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

#[derive(Debug, PartialEq, Eq, Clone)]
enum List {
    Value(u32),
    List(Vec<List>),
}


impl List {
    fn new() -> Self {
        Self::List ( Vec::new() )
    }

    fn push(&mut self, item: List) {
        match self {
            Self::List(list) => list.push(item),
            _ => panic!("Trying to push item into value")
        }
    }
}

impl FromStr for List {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut list = Self::new();

        // Make sure 
        if input.is_empty() {
            return Err(anyhow::anyhow!("Expected list, got empty string instead"));
        }

        // unwrap the list [ something ] into just something
        let unparsed_list = input.get(1..input.len()-1).unwrap();

        let mut indent = 0;
        let mut idx = 0;
        loop {
            let item = unparsed_list.chars().skip(idx).take_while(|&c| {
                idx += 1;
                if c == '[' {
                    indent += 1;
                } else if c == ']' {
                    indent -= 1;
                } else if indent == 0 && c == ',' {
                    return false;
                }

                return true;
            }).collect::<String>();

            if item.len() == 0 {
                return Ok(list);
            }

            if item.starts_with('[') {
                list.push(List::from_str(&item).unwrap());
            } else {
                list.push(List::Value(item.parse().unwrap()))
            }
        }
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            List::List(list) => {
                write!(f, "[")?;
                for entry in list {
                    write!(f, "{entry},")?;
                }
                write!(f, "]")?;
            },
            List::Value(value) => {
                write!(f, "{value}")?;
            }
        }

        Ok(())
    }
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (List::Value(a), List::Value(b)) => {
                return a.cmp(b);
            },
            (List::List(a), List::List(b)) => {
                let len = min(a.len(), b.len());
                for idx in 0..len {
                    let ord = a[idx].cmp(&b[idx]);
                    if ord != Ordering::Equal {
                        return ord;
                    }
                }

                return a.len().cmp(&b.len());
            },
            (List::List(_), List::Value(b)) => {
                let mut list = List::new();
                list.push(List::Value(*b));
                return self.cmp(&list);
            }
            (List::Value(a), List::List(_)) => {
                let mut list = List::new();
                list.push(List::Value(*a));
                return list.cmp(other);
            }
        }
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = usize;
    type Output2 = usize;

    fn day() -> u8 {
        13
    }

    fn part1(input: &str) -> Self::Output1 {
        input
            .trim()
            .split("\n\n")
            .map(|pair| pair.split_once("\n").unwrap())
            .map(|(left, right)| (List::from_str(left).unwrap(), List::from_str(right).unwrap()))
            .enumerate()
            .fold(0, |mut acc, (idx, (left, right))| {
                if left <= right {
                    acc += idx + 1;
                }
                acc
            })
    }

    fn part2(input: &str) -> Self::Output2 {
        let mut lists = input
            .trim()
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| List::from_str(line).unwrap())
            .collect::<Vec<_>>();

        // Add the two divider packets
        let a = List::List(vec![List::List(vec![List::Value(2)])]);
        let b = List::List(vec![List::List(vec![List::Value(6)])]);
        lists.push(a.clone());
        lists.push(b.clone());

        lists.sort();

        lists
            .into_iter()
            .enumerate()
            .filter(|(_, list)| *list == a || *list == b)
            .fold(1, |acc, (idx, _)| acc * (idx+1))
    }
}
