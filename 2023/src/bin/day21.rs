#![feature(test)]
use std::collections::{HashMap, HashSet, VecDeque};

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
        // The example only provides an answer for 6 steps. (16)
        // This should be the correct answer for running the example with 64 steps
        Day::test(Day::part1, "test-1", 42)
    }

    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 3642)
    }

    // There is no test case for part 2

    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", 608603023105276)
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
    type Output2 = isize;

    fn day() -> u8 {
        21
    }

    fn part1(input: &str) -> Self::Output1 {
        let mut queue = Vec::new();
        let map: HashMap<(isize, isize), char> = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, mut c)| {
                        if c == 'S' {
                            queue.push((x as isize, y as isize));
                            c = '.';
                        }

                        ((x as isize, y as isize), c)
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        let mut set = HashSet::new();
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for _ in 0..64 {
            while let Some(step) = queue.pop() {
                for direction in directions {
                    let next = (step.0 + direction.0, step.1 + direction.1);

                    // If the tile is free add it to the queue
                    if let Some(&tile) = map.get(&next) {
                        if tile == '.' {
                            set.insert(next);
                        }
                    }
                }
            }

            queue = set.into_iter().collect();
            set = HashSet::new();
        }

        queue.len()
    }

    fn part2(input: &str) -> Self::Output2 {
        // All maps are square
        let size = input.lines().count();
        // Map is square: 131 x 131 => size = 131
        // !!! Others have observed 26501365 = 202300 * size + size/2 !!!
        // Is there a pattern for i * size + size/2?
        // i = 0 => 3776
        // i = 1 => 33652
        // i = 2 => 93270
        // Is there a function of i that can fit to this?
        // Yes => Quadratic equation will fit this
        //      3642 - 14737 x + 14871 x^2
        // i = 202300 => 608603023105276
        // Could not have done this without a hint from Reddit...
        let i: isize = 202300;

        let mut queue = Vec::new();
        let map: HashMap<(isize, isize), char> = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, mut c)| {
                        if c == 'S' {
                            queue.push((x as isize, y as isize));
                            c = '.';
                        }

                        ((x as isize, y as isize), c)
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        let mut nums = Vec::new();
        let mut set = HashSet::new();
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for n in 0..(2 * size + size / 2) {
            while let Some(step) = queue.pop() {
                for direction in directions {
                    let next = (step.0 + direction.0, step.1 + direction.1);

                    let next_wrapped = (
                        next.0.rem_euclid(size as isize),
                        next.1.rem_euclid(size as isize),
                    );

                    // If the tile is free add it to the queue
                    if let Some(&tile) = map.get(&next_wrapped) {
                        if tile == '.' {
                            set.insert(next);
                        }
                    }
                }
            }

            queue = set.into_iter().collect();
            set = HashSet::new();

            if n + 1 == nums.len() * size + size / 2 {
                nums.push(queue.len() as isize);
            }
        }

        // Using linear algebra these solutions can be found
        let a = (nums[0] - 2 * nums[1] + nums[2]) / 2;
        let b = (4 * nums[1] - 3 * nums[0] - nums[2]) / 2;
        let c = nums[0];

        a * i.pow(2) + b * i + c
    }
}
