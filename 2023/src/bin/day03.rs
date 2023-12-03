#![feature(test)]
use std::collections::{HashMap, HashSet};

use anyhow::Result;
use aoc::Solver;
use regex::Regex;

// -- Runners --
fn main() -> Result<()> {
    Day::solve()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() -> Result<()> {
        Day::test(Day::part1, "test-1", 4361)
    }

    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 550934)
    }

    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(Day::part2, "test-1", 467835)
    }

    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", 81997870)
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

// Regex for matching the numbers
lazy_static::lazy_static! {
    static ref RE: Regex = Regex::new(r"\d+").expect("Regex should be valid");
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = usize;
    type Output2 = usize;

    fn day() -> u8 {
        3
    }

    fn part1(input: &str) -> Self::Output1 {
        // Constuct a set of all symbol positions
        let grid: HashSet<(isize, isize)> = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(x, c)| {
                        if c != '.' && !c.is_ascii_digit() {
                            Some((x as isize, y as isize))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        // Calculate the sum of all the numbers adjacent to a symbol
        input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                let y = y as isize;

                // Find every number in the line
                RE.find_iter(line)
                    .map(|m| {
                        // Parse the number
                        let number = m.as_str().parse().expect("Match should always be a number");

                        // Start one before and end one after the number
                        let start = m.start() as isize - 1;
                        // NOTE: End is the offset of the byte immediately following the last matched byte
                        let end = m.end() as isize;

                        // Check before and after the number
                        if grid.contains(&(start, y)) || grid.contains(&(end, y)) {
                            return number;
                        }

                        // Check above, below and diagonal of the number
                        for x in start..=end {
                            if grid.contains(&(x, y - 1)) || grid.contains(&(x, y + 1)) {
                                return number;
                            }
                        }

                        0
                    })
                    .sum::<usize>()
            })
            .sum()
    }

    fn part2(input: &str) -> Self::Output2 {
        // Constuct a set of all symbol positions, this time with a vector for storing adjacent
        // numbers
        let mut grid: HashMap<(isize, isize), Vec<usize>> = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(x, c)| {
                        if c != '.' && !c.is_ascii_digit() {
                            Some(((x as isize, y as isize), vec![]))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        // For each number find the adjacent symbols and add the number to its list
        input.lines().enumerate().for_each(|(y, line)| {
            let y = y as isize;
            RE.find_iter(line).for_each(|m| {
                // Parse the number
                let number = m.as_str().parse().expect("Match should always be a number");

                // Start one before and end one after the number
                let start = m.start() as isize - 1;
                // NOTE: End is the offset of the byte immediately following the last matched byte
                let end = m.end() as isize;

                // Check before the number
                if let Some(nums) = grid.get_mut(&(start, y)) {
                    nums.push(number);
                }

                // Check after the number
                if let Some(nums) = grid.get_mut(&(end, y)) {
                    nums.push(number);
                }

                // Check above, below, and diagonally
                for x in start..=end {
                    // Above
                    if let Some(nums) = grid.get_mut(&(x, y - 1)) {
                        nums.push(number);
                    }
                    // Below
                    if let Some(nums) = grid.get_mut(&(x, y + 1)) {
                        nums.push(number);
                    }
                }
            });
        });

        // For every symbol with two or more adjacent numbers, multiply them together
        grid.values()
            .filter(|nums| nums.len() > 1)
            .map(|nums| nums.iter().product::<usize>())
            .sum()
    }
}
