#![feature(test)]

use std::{fmt::Display, collections::{hash_map::DefaultHasher, HashMap}, hash::{Hash, Hasher}};

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
        Day::test(Day::part1, "test-1", 136)
    }

    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 110407)
    }

    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(Day::part2, "test-1", 64)
    }

    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", 87273)
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Space {
    Empty,
    Cube,
    Round,
}

impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Space::Empty => write!(f, "."),
            Space::Cube => write!(f, "#"),
            Space::Round => write!(f, "O"),
        }
    }
}

impl From<char> for Space {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Cube,
            'O' => Self::Round,
            _ => unreachable!("Invalid input"),
        }
    }
}

fn print_grid(grid: &[Vec<Space>]) {
    for line in grid {
        for space in line {
            print!("{space}");
        }
        println!();
    }
}

fn tilt_north(grid: &mut Vec<Vec<Space>>) {
    let width = grid[0].len();
    let height = grid.len();

    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == Space::Round {
                let mut new_y = y;
                for yy in (0..y).rev() {
                    if grid[yy][x] == Space::Empty {
                        new_y = yy
                    } else {
                        break;
                    }
                }
                if new_y < y {
                    grid[y][x] = Space::Empty;
                    grid[new_y][x] = Space::Round;
                }
            }
        }
    }
}

fn tilt_east(grid: &mut [Vec<Space>]) {
    let width = grid[0].len();

    for x in (0..width).rev() {
        for line in grid.iter_mut() {
            if line[x] == Space::Round {
                let mut new_x = x;
                for (xx, space) in line.iter().enumerate().skip(x+1) {
                    if space == &Space::Empty {
                        new_x = xx
                    } else {
                        break;
                    }
                }
                if new_x > x {
                    line[x] = Space::Empty;
                    line[new_x] = Space::Round;
                }
            }
        }
    }
}

fn tilt_south(grid: &mut Vec<Vec<Space>>) {
    let width = grid[0].len();
    let height = grid.len();

    for y in (0..height).rev() {
        for x in 0..width {
            if grid[y][x] == Space::Round {
                let mut new_y = y;
                for (yy, line) in grid.iter().enumerate().skip(y+1) {
                    if line[x] == Space::Empty {
                        new_y = yy
                    } else {
                        break;
                    }
                }
                if new_y > y {
                    grid[y][x] = Space::Empty;
                    grid[new_y][x] = Space::Round;
                }
            }
        }
    }
}

fn tilt_west(grid: &mut [Vec<Space>]) {
    let width = grid[0].len();

    for x in 0..width {
        for line in grid.iter_mut() {
            if line[x] == Space::Round {
                let mut new_x = x;
                for xx in (0..x).rev() {
                    if line[xx] == Space::Empty {
                        new_x = xx
                    } else {
                        break;
                    }
                }

                if new_x < x {
                    line[x] = Space::Empty;
                    line[new_x] = Space::Round;
                }
            }
        }
    }
}

fn calculate_load(grid: &Vec<Vec<Space>>) -> usize {
    grid.iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .filter_map(|space| {
                    if space == &Space::Round {
                        Some(grid.len() - y)
                    } else {
                        None
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

fn cycle(grid: &mut Vec<Vec<Space>>) {
    tilt_north(grid);
    tilt_west(grid);
    tilt_south(grid);
    tilt_east(grid);
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = usize;
    type Output2 = usize;

    fn day() -> u8 {
        14
    }

    fn part1(input: &str) -> Self::Output1 {
        let mut grid: Vec<Vec<Space>> = input
            .lines()
            .map(|line| line.chars().map(|c| c.into()).collect())
            .collect();

        tilt_north(&mut grid);

        calculate_load(&grid)
    }

    fn part2(input: &str) -> Self::Output2 {
        let mut grid: Vec<Vec<Space>> = input
            .lines()
            .map(|line| line.chars().map(|c| c.into()).collect())
            .collect();

        let mut cache = HashMap::new();
        let mut cycles = 0;
        let loop_length = loop {
            cycle(&mut grid);
            cycles += 1;

            // Calculate the hash of the current state
            let mut hash = DefaultHasher::new();
            grid.hash(&mut hash);
            let hash = hash.finish();

            // Check if we have encountered this state before
            if let Some(c) = cache.get(&hash) {
                break cycles - c;
            }

            // Insert the state in the map
            cache.insert(hash, cycles);
        };

        let remaining = (1000000000 - cycles) % loop_length;
        for _ in 0..remaining {
            cycle(&mut grid);
        }

        calculate_load(&grid)
    }
}
