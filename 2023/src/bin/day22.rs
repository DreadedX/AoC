#![feature(test)]
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
        Day::test(Day::part1, "test-1", 5)
    }

    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 488)
    }

    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(Day::part2, "test-1", 7)
    }

    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", 79465)
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

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Brick {
    start: (isize, isize, isize),
    end: (isize, isize, isize),
}

impl Brick {
    fn fall(
        &self,
        index: usize,
        grid: &mut HashMap<(isize, isize, isize), usize>,
    ) -> HashSet<usize> {
        let mut down = 0;
        // Keep track of all the unique bricks supporting the current brick
        let mut supports = HashSet::new();
        for o in 1..self.start.2 {
            for x in self.start.0..=self.end.0 {
                for y in self.start.1..=self.end.1 {
                    for z in self.start.2..=self.end.2 {
                        if let Some(&support) = grid.get(&(x, y, z - o)) {
                            supports.insert(support);
                        }
                    }
                }
            }

            // We have landed on a support
            if !supports.is_empty() {
                break;
            }

            down += 1;
        }

        // Occupy the space in the grid
        for x in self.start.0..=self.end.0 {
            for y in self.start.1..=self.end.1 {
                for z in self.start.2..=self.end.2 {
                    grid.insert((x, y, z - down), index);
                }
            }
        }

        // Return all the unique supports
        supports
    }
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = usize;
    type Output2 = usize;

    fn day() -> u8 {
        22
    }

    fn part1(input: &str) -> Self::Output1 {
        let mut bricks: Vec<_> = input
            .lines()
            .map(|line| {
                let (start, end) = line.split_once('~').unwrap();

                let start: Vec<_> = start
                    .splitn(3, ',')
                    .map(|num| num.parse::<isize>().unwrap())
                    .collect();
                let start = (start[0], start[1], start[2]);

                let end: Vec<_> = end
                    .splitn(3, ',')
                    .map(|num| num.parse::<isize>().unwrap())
                    .collect();
                let end = (end[0], end[1], end[2]);

                Brick { start, end }
            })
            .collect();

        // Sort the bricks from top to bottom
        bricks.sort_by(|a, b| a.start.2.cmp(&b.start.2));

        // Figure out which bricks are essential and can not be disintegrated
        let mut grid = HashMap::new();
        let essential_bricks: HashSet<_> = bricks
            .iter()
            .enumerate()
            // Drop down each brick and get all the supporting bricks
            .map(|(index, brick)| brick.fall(index, &mut grid))
            // Only keep bricks that are supported by one other brick
            // If that other brick is disintegrated this brick will fall
            .filter(|supports| supports.len() == 1)
            // Flatten out and collect to take out duplicate entries
            .flatten()
            .collect();

        bricks.len() - essential_bricks.len()
    }

    fn part2(input: &str) -> Self::Output2 {
        let mut bricks: Vec<_> = input
            .lines()
            .map(|line| {
                let (start, end) = line.split_once('~').unwrap();

                let start: Vec<_> = start
                    .splitn(3, ',')
                    .map(|num| num.parse::<isize>().unwrap())
                    .collect();
                let start = (start[0], start[1], start[2]);

                let end: Vec<_> = end
                    .splitn(3, ',')
                    .map(|num| num.parse::<isize>().unwrap())
                    .collect();
                let end = (end[0], end[1], end[2]);

                Brick { start, end }
            })
            .collect();

        // Sort the bricks from top to bottom
        bricks.sort_by(|a, b| a.start.2.cmp(&b.start.2));

        // Figure out which bricks are essential and can not be disintegrated
        let mut grid = HashMap::new();
        let supports: Vec<_> = bricks
            .iter()
            .enumerate()
            // Drop down each brick and get all the supporting bricks
            .map(|(index, brick)| brick.fall(index, &mut grid))
            .collect();

        let essential_bricks: HashSet<_> = supports
            .iter()
            .filter(|supports| supports.len() == 1)
            // Flatten out and collect to take out duplicate entries
            .flatten()
            .collect();

        let mut sum = 0;
        for eliminate in essential_bricks {
            let mut falling = HashSet::new();
            falling.insert(*eliminate);
            let mut previous = 0;

            while previous != falling.len() {
                previous = falling.len();

                let new: HashSet<_> = supports
                    .iter()
                    .enumerate()
                    .filter(|(_, supports)| {
                        if supports.is_empty() {
                            return false;
                        }

                        for support in *supports {
                            if !falling.contains(support) {
                                return false;
                            }
                        }

                        true
                    })
                    .map(|(index, _)| index)
                    .collect();

                falling.extend(new);
            }

            if !falling.is_empty() {
                sum += falling.len() - 1;
            }
        }

        sum
    }
}
