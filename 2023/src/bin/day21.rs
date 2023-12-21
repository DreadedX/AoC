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
        // The example provided uses 6 steps instead of 64, however this should be the correct
        // answer for 64 steps with the example
        Day::test(Day::part1, "test-1", 42)
    }

    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 3642)
    }

    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(Day::part2, "test-1", 154)
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
                            queue.push(((x as isize, y as isize), 64));
                            c = '.';
                        }

                        ((x as isize, y as isize), c)
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        let mut visited = HashSet::new();
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        // Depth first
        while let Some(step) = queue.pop() {
            // If we have already evaluated this state, do not add it to the queue
            if visited.contains(&step) {
                continue;
            }

            // Mark this node as visited
            visited.insert(step);

            // Check if we have ran out of steps
            if step.1 == 0 {
                continue;
            }

            // Try moving in all directions
            for direction in directions {
                let next = (
                    (step.0 .0 + direction.0, step.0 .1 + direction.1),
                    step.1 - 1,
                );

                // If the tile is free add it to the queue
                if let Some(&tile) = map.get(&next.0) {
                    if tile == '.' {
                        queue.push(next);
                    }
                }
            }
        }

        visited.iter().filter(|step| step.1 == 0).count()
    }

    fn part2(input: &str) -> Self::Output2 {
        let height = input.lines().count();
        let width = input.lines().next().unwrap().chars().count();
        println!("{width}, {height}");
        // Map is square: 131 x 131
        // !!! Others have observed 26501365 = 202300 * 131 + 65 !!!
        // Is there a pattern for i * 131 + 65?
        // i = 0 => 3776
        // i = 1 => 33652
        // i = 2 => 93270
        // Is there a function of i that can fit to this?
        // Yes => Quadratic equation will fit this
        //      3642 - 14737 x + 14871 x^2
        // i = 202300 => 608603023105276
        // Could not have done this without a hint from Reddit...
        const N: usize = 2 * 131 + 65;

        let mut queue = VecDeque::new();
        let map: HashMap<(isize, isize), char> = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, mut c)| {
                        if c == 'S' {
                            queue.push_back(((x as isize, y as isize), 0));
                            c = '.';
                        }

                        ((x as isize, y as isize), c)
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        let mut visited = HashSet::new();
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        // Depth first
        while let Some(step) = queue.pop_front() {
            // If we have already evaluated this state, do not add it to the queue
            if visited.contains(&step) {
                continue;
            }

            // Mark this node as visited
            visited.insert(step);

            // Check if we have ran out of steps
            if step.1 == N {
                continue;
            }

            // Try moving in all directions
            for direction in directions {
                let next = (
                    (step.0 .0 + direction.0, step.0 .1 + direction.1),
                    step.1 + 1,
                );

                let next_wrapped = (
                    next.0 .0.rem_euclid(width as isize),
                    next.0 .1.rem_euclid(height as isize),
                );

                // If the tile is free add it to the queue
                if let Some(&tile) = map.get(&next_wrapped) {
                    if tile == '.' {
                        queue.push_back(next);
                    }
                }
            }
        }

        visited.iter().filter(|step| step.1 == N).count()
    }
}
