#![feature(test)]
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

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
        Day::test(Day::part1, "test-1", 21)
    }

    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 7674)
    }

    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(Day::part2, "test-1", 525152)
    }

    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", 4443895258186)
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

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl Spring {
    fn convert(c: char) -> Self {
        match c {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => unreachable!("Invalid input"),
        }
    }
}

impl std::fmt::Display for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Spring::Operational => write!(f, "."),
            Spring::Damaged => write!(f, "#"),
            Spring::Unknown => write!(f, "?"),
        }
    }
}

// The cache is optional as is slows down part1 by a lot
fn count_valid(
    springs: &mut [Spring],
    list: &[usize],
    damaged_chain: usize,
    cache: &mut Option<&mut Cache>,
) -> usize {
    let hash = if let Some(cache) = cache {
        // Calculate the hash manually, otherwise we end up with weird borrows or we have to clone
        // (which is slow)
        let mut hash = DefaultHasher::new();
        springs.hash(&mut hash);
        list.hash(&mut hash);
        damaged_chain.hash(&mut hash);
        let hash = hash.finish();

        if let Some(&value) = cache.get(&hash) {
            return value;
        }

        Some(hash)
    } else {
        None
    };

    // We reached the end of the list, no further processing is possible
    let count = if list.is_empty() {
        if springs
            .iter()
            .filter(|&spring| spring == &Spring::Damaged)
            .count()
            > 0
        {
            // There are still damaged springs remaining
            0
        } else {
            // There are no damaged springs remaining
            1
        }
    } else if springs.is_empty() {
        // If there is one last entry in the list, check if it maches
        if list.len() == 1 && damaged_chain == list[0] {
            1
        } else {
            0
        }
    } else {
        match springs[0] {
            Spring::Operational => {
                if damaged_chain == 0 {
                    // The previous entry was operation, skip over this entry
                    count_valid(&mut springs[1..], list, 0, cache)
                } else if damaged_chain == list[0] {
                    // We reached the end of a chain and it has the expected length, start working on the next chain
                    count_valid(&mut springs[1..], &list[1..], 0, cache)
                } else {
                    // We reached the end of a chain and it does not have the expected length
                    0
                }
            }
            Spring::Unknown => {
                // Replace unknown with operational
                springs[0] = Spring::Operational;
                let a = count_valid(springs, list, damaged_chain, cache);

                // Replace unknown with damaged
                springs[0] = Spring::Damaged;
                let b = count_valid(springs, list, damaged_chain, cache);

                // Reset spring back to unknown
                springs[0] = Spring::Unknown;

                // Return the sum of both branches
                a + b
            }
            Spring::Damaged => {
                // Add to the damaged chain
                count_valid(&mut springs[1..], list, damaged_chain+1, cache)
            }
        }
    };

    if let Some(cache) = cache {
        cache.insert(hash.unwrap(), count);
    }

    count
}

type Cache = HashMap<u64, usize>;

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = usize;
    type Output2 = usize;

    fn day() -> u8 {
        12
    }

    fn part1(input: &str) -> Self::Output1 {
        input
            .lines()
            .map(|line| line.split_once(' ').unwrap())
            .map(|(springs, list)| {
                (
                    springs.chars().map(Spring::convert).collect::<Vec<_>>(),
                    list.split(',')
                        .map(|num| num.parse::<usize>().unwrap())
                        .collect::<Vec<_>>(),
                )
            })
            .map(|(mut springs, list)| count_valid(&mut springs, &list, 0, &mut None))
            .sum()
    }

    fn part2(input: &str) -> Self::Output2 {
        input
            .lines()
            .map(|line| line.split_once(' ').unwrap())
            .map(|(springs, list)| {
                (
                    springs.chars().map(Spring::convert).collect::<Vec<_>>(),
                    list.split(',')
                        .map(|num| num.parse::<usize>().unwrap())
                        .collect::<Vec<_>>(),
                )
            })
            .map(|(springs, list)| {
                (
                    [
                        springs.clone(),
                        vec![Spring::Unknown],
                        springs.clone(),
                        vec![Spring::Unknown],
                        springs.clone(),
                        vec![Spring::Unknown],
                        springs.clone(),
                        vec![Spring::Unknown],
                        springs,
                    ]
                    .concat(),
                    [list.clone(), list.clone(), list.clone(), list.clone(), list].concat(),
                )
            })
            .map(|(mut springs, list)| {
                let mut cache = HashMap::new();
                count_valid(&mut springs, &list, 0, &mut Some(&mut cache))
            })
            .sum()
    }
}
