#![feature(test)]
#![feature(let_chains)]

use std::collections::{HashMap, HashSet};

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
        Day::test(Day::part1, "test-1", 102)
    }

    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 1256)
    }

    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(Day::part2, "test-1", 94)
    }

    #[test]
    fn part2_test2() -> Result<()> {
        Day::test(Day::part2, "test-2", 71)
    }

    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", 1382)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Key {
    pos_x: usize,
    pos_y: usize,
    dir_x: isize,
    dir_y: isize,
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = usize;
    type Output2 = usize;

    fn day() -> u8 {
        17
    }

    fn part1(input: &str) -> Self::Output1 {
        let map: Vec<Vec<_>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect();

        let size = (map[0].len(), map.len());
        let destination = (size.0 - 1, size.1 - 1);

        let mut unvisited = HashMap::new();
        unvisited.insert(
            Key {
                pos_x: 0,
                pos_y: 0,
                dir_x: 0,
                dir_y: 0,
            },
            0,
        );

        let mut seen = HashSet::new();

        loop {
            let current = unvisited
                .iter()
                .min_by_key(|(_key, cost)| *cost)
                .map(|(key, cost)| (*key, *cost))
                .unwrap();

            if current.0.pos_x == destination.0 && current.0.pos_y == destination.1 {
                return current.1;
            }

            // Check above
            if current.0.dir_y > -3
                && current.0.dir_y <= 0
                && let Some(y) = current.0.pos_y.checked_sub(1)
            {
                let x = current.0.pos_x;
                let cost = current.1 + map[y][x];

                let key = Key {
                    pos_x: x,
                    pos_y: y,
                    dir_x: 0,
                    dir_y: current.0.dir_y - 1,
                };

                if let Some(next) = unvisited.get_mut(&key) {
                    *next = std::cmp::min(*next, cost);
                } else if !seen.contains(&key) {
                    unvisited.insert(key, cost);
                }
            }

            // Check below
            let y = current.0.pos_y + 1;
            if current.0.dir_y < 3 && current.0.dir_y >= 0 && y < size.1 {
                let x = current.0.pos_x;
                let cost = current.1 + map[y][x];

                let key = Key {
                    pos_x: x,
                    pos_y: y,
                    dir_x: 0,
                    dir_y: current.0.dir_y + 1,
                };

                if let Some(next) = unvisited.get_mut(&key) {
                    *next = std::cmp::min(*next, cost);
                } else if !seen.contains(&key) {
                    unvisited.insert(key, cost);
                }
            }

            // Check left
            if current.0.dir_x > -3
                && current.0.dir_x <= 0
                && let Some(x) = current.0.pos_x.checked_sub(1)
            {
                let y = current.0.pos_y;
                let cost = current.1 + map[y][x];

                let key = Key {
                    pos_x: x,
                    pos_y: y,
                    dir_x: current.0.dir_x - 1,
                    dir_y: 0,
                };

                if let Some(next) = unvisited.get_mut(&key) {
                    *next = std::cmp::min(*next, cost);
                } else if !seen.contains(&key) {
                    unvisited.insert(key, cost);
                }
            }

            // Check right
            let x = current.0.pos_x + 1;
            if current.0.dir_x < 3 && current.0.dir_x >= 0 && x < size.0 {
                let y = current.0.pos_y;
                let cost = current.1 + map[y][x];

                let key = Key {
                    pos_x: x,
                    pos_y: y,
                    dir_x: current.0.dir_x + 1,
                    dir_y: 0,
                };

                if let Some(next) = unvisited.get_mut(&key) {
                    *next = std::cmp::min(*next, cost);
                } else if !seen.contains(&key) {
                    unvisited.insert(key, cost);
                }
            }

            // Mark the current node as visited
            unvisited.remove(&current.0);
            seen.insert(current.0);
        }
    }

    fn part2(input: &str) -> Self::Output2 {
        let map: Vec<Vec<_>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect();

        let size = (map[0].len(), map.len());
        let destination = (size.0 - 1, size.1 - 1);

        let mut unvisited = HashMap::new();
        unvisited.insert(
            Key {
                pos_x: 0,
                pos_y: 0,
                dir_x: 0,
                dir_y: 0,
            },
            0,
        );

        let mut seen = HashSet::new();

        loop {
            let current = unvisited
                .iter()
                .min_by_key(|(_key, cost)| *cost)
                .map(|(key, cost)| (*key, *cost))
                .unwrap();

            if current.0.pos_x == destination.0
                && current.0.pos_y == destination.1
                && (current.0.dir_x >= 4 || current.0.dir_y >= 4)
            {
                return current.1;
            }

            // Check above
            if current.0.dir_y > -10
                && current.0.dir_y <= 0
                && (current.0.dir_x == 0 || current.0.dir_x >= 4 || current.0.dir_x <= -4)
                && let Some(y) = current.0.pos_y.checked_sub(1)
            {
                let x = current.0.pos_x;
                let cost = current.1 + map[y][x];

                let key = Key {
                    pos_x: x,
                    pos_y: y,
                    dir_x: 0,
                    dir_y: current.0.dir_y - 1,
                };

                if let Some(next) = unvisited.get_mut(&key) {
                    *next = std::cmp::min(*next, cost);
                } else if !seen.contains(&key) {
                    unvisited.insert(key, cost);
                }
            }

            // Check below
            let y = current.0.pos_y + 1;
            if current.0.dir_y < 10
                && current.0.dir_y >= 0
                && (current.0.dir_x == 0 || current.0.dir_x >= 4 || current.0.dir_x <= -4)
                && y < size.1
            {
                let x = current.0.pos_x;
                let cost = current.1 + map[y][x];

                let key = Key {
                    pos_x: x,
                    pos_y: y,
                    dir_x: 0,
                    dir_y: current.0.dir_y + 1,
                };

                if let Some(next) = unvisited.get_mut(&key) {
                    *next = std::cmp::min(*next, cost);
                } else if !seen.contains(&key) {
                    unvisited.insert(key, cost);
                }
            }

            // Check left
            if current.0.dir_x > -10
                && current.0.dir_x <= 0
                && (current.0.dir_y == 0 || current.0.dir_y >= 4 || current.0.dir_y <= -4)
                && let Some(x) = current.0.pos_x.checked_sub(1)
            {
                let y = current.0.pos_y;
                let cost = current.1 + map[y][x];

                let key = Key {
                    pos_x: x,
                    pos_y: y,
                    dir_x: current.0.dir_x - 1,
                    dir_y: 0,
                };

                if let Some(next) = unvisited.get_mut(&key) {
                    *next = std::cmp::min(*next, cost);
                } else if !seen.contains(&key) {
                    unvisited.insert(key, cost);
                }
            }

            // Check right
            let x = current.0.pos_x + 1;
            if current.0.dir_x < 10
                && current.0.dir_x >= 0
                && (current.0.dir_y == 0 || current.0.dir_y >= 4 || current.0.dir_y <= -4)
                && x < size.0
            {
                let y = current.0.pos_y;
                let cost = current.1 + map[y][x];

                let key = Key {
                    pos_x: x,
                    pos_y: y,
                    dir_x: current.0.dir_x + 1,
                    dir_y: 0,
                };

                if let Some(next) = unvisited.get_mut(&key) {
                    *next = std::cmp::min(*next, cost);
                } else if !seen.contains(&key) {
                    unvisited.insert(key, cost);
                }
            }

            // Mark the current node as visited
            unvisited.remove(&current.0);
            seen.insert(current.0);
        }
    }
}
