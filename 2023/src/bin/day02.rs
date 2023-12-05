#![feature(test)]
use std::cmp::max;

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
        Day::test(Day::part1, "test-1", 8)
    }

    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 1931)
    }

    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(Day::part2, "test-1", 2286)
    }

    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", 83105)
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

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = usize;
    type Output2 = usize;

    fn day() -> u8 {
        2
    }

    fn part1(input: &str) -> Self::Output1 {
        let red = 12;
        let green = 13;
        let blue = 14;

        input
            .lines()
            .filter_map(|line| {
                // Split the game id from the actual game played
                let (game, line) = line.split_once(": ").unwrap();

                // Check if the game played by checking if all sets played are valid
                let valid = line.split("; ").all(|set| {
                    // Check if each entry in the set is valid
                    set.split(", ").all(|entry| {
                        // Get the amount and the color of the cubes
                        let (amount, color) = entry.split_once(' ').unwrap();
                        let amount: usize = amount.parse().unwrap();

                        // Check if that amount of cubes is valid for the given color
                        match color {
                            "red" => amount <= red,
                            "green" => amount <= green,
                            "blue" => amount <= blue,
                            _ => unreachable!("Input is invalid"),
                        }
                    })
                });

                // If valid, get the id and add it to the sum
                if valid {
                    let id = game.split_once(' ').unwrap().1;
                    let id: usize = id.parse().unwrap();
                    Some(id)
                } else {
                    None
                }
            })
            .sum()
    }

    fn part2(input: &str) -> Self::Output2 {
        // The games do not need to be split into sets anymore for part 2
        // This replacement makes it easier to get every number + color pair without extra nesting
        let input = input.replace(';', ",");

        input
            .lines()
            .map(|line| {
                // Split the game id from the actual game played
                let line = line.split_once(": ").unwrap().1;

                // Get the required minimum amount for each color
                let required = line.split(", ").fold((0, 0, 0), |acc, entry| {
                    // Get the amount and the color of the cubes
                    let (amount, color) = entry.split_once(' ').unwrap();
                    let amount: usize = amount.parse().unwrap();

                    // Update the minimum amount require as neccesary.
                    match color {
                        "red" => (max(acc.0, amount), acc.1, acc.2),
                        "green" => (acc.0, max(acc.1, amount), acc.2),
                        "blue" => (acc.0, acc.1, max(acc.2, amount)),
                        _ => unreachable!("Input is invalid"),
                    }
                });

                // Get the power of the set of cubes
                required.0 * required.1 * required.2
            })
            .sum()
    }
}
