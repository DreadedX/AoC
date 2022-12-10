#![feature(test)]
use std::{path::PathBuf, collections::HashMap, ops::AddAssign};

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
        Day::test(Day::part1, "test-1", 95437)
    }
    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 2031851)
    }
    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(Day::part2, "test-1", 24933642)
    }
    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", 2568781)
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

// -- Helper functions --
fn process(input: &str) -> Vec<u32> {
    let mut path = PathBuf::new();
    let mut map = HashMap::new();

    input
        .lines()
        .map(|line| line.rsplit_once(" ").unwrap())
        .for_each(|split| {
            match split {
                ("$ cd", "/") => { path.clear(); }, // Clear the path
                ("$ cd", "..") => { path.pop(); }, // Go up one level in the path
                ("$ cd", name) => { path.push(name); }, // Enter a directory
                ("$", "ls") => {},
                ("dir", _name) => {},
                (size, _name) => {
                    let mut temp = path.clone();
                    while {
                        // Update the size of the current and all parent directories
                        map.entry(temp.clone()).or_insert(0).add_assign(size.parse::<u32>().unwrap());
                        temp.pop()
                    } {}
                },
            }
        });

    map.iter().map(|(_, &size)| size).collect()
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = u32;
    type Output2 = u32;
    fn day() -> u8 {
        7
    }

    fn part1(input: &str) -> Self::Output1 {
        process(input)
            .iter()
            .filter(|&&size| size < 100000)
            .sum()
    }

    fn part2(input: &str) -> Self::Output2 {
        let mut sizes = process(input);
        sizes.sort();

        // The root is always the larges directory, so it will end up in the last element
        let need_to_free = sizes.last().unwrap() - 40000000;

        sizes.iter()
            .find_map(|&size| {
            if size > need_to_free {
                Some(size)
            } else {
                None
            }
        }).unwrap()
    }
}
