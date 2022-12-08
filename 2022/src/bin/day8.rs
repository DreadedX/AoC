#![feature(test)]
use core::fmt;
use std::ptr;

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
        Day::test(aoc::Part::ONE, "test-1", 21)
    }
    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(aoc::Part::ONE, "input", 1845)
    }
    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(aoc::Part::TWO, "test-1", 8)
    }
    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(aoc::Part::TWO, "input", 230112)
    }

    // Benchmarks
    extern crate test;
    #[bench]
    #[ignore]
    fn part1_bench(b: &mut test::Bencher) {
        Day::benchmark(aoc::Part::ONE, b)
    }
    #[bench]
    #[ignore]
    fn part2_bench(b: &mut test::Bencher) {
        Day::benchmark(aoc::Part::TWO, b)
    }
}
// -- Helpers --
fn parse(input: &str) -> (usize, Vec<Vec<u32>>) {
    let size = input.lines().count();
    let input = input
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    (size, input)
}

fn is_visible(size: usize, highest: &mut u32, height: u32, y: usize, x: usize) -> bool {
    match (y, x, height) {
        (r, _, _) if r == 0 || r == size-1 => true,
        (_, c, _) if c == 0 || c == size-1 => {
            *highest = height;
            true
        },
        (_, _, h) => {
            if h > *highest {
                *highest = h;
                true
            } else {
                false
            }
        }
    }
}

// Consume the vector and perform the transpose by swapping around elements
fn transpose<T: Copy + fmt::Display>(size: usize, mut input: Vec<Vec<T>>) -> Vec<Vec<T>> {
    for y in 0..size {
        for x in 0..y {
            unsafe {
                let pa: *mut T = &mut input[x][y];
                let pb: *mut T = &mut input[y][x];

                ptr::swap(pa, pb);
            }
        }
    }

    input
}

fn process_1d(input: &Vec<Vec<u32>>) -> Vec<Vec<bool>> {
    input.iter()
        .enumerate()
        .map(|(y, row)| {
            let size = row.len();
            let left = row.iter()
                .enumerate()
                .scan(0, |mut highest, (x, &height)| Some(is_visible(size, &mut highest, height, y, x)));

            let mut right = row.iter()
                .enumerate()
                .rev()
                .scan(0, |mut highest, (x, &height)| Some(is_visible(size, &mut highest, height, y, x)))
                .collect::<Vec<_>>();

            right.reverse();

            left.zip(right.iter())
                .map(|(left, &right)| left || right)
                .collect::<Vec<_>>()

        }).collect()
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output = usize;
    fn day() -> u8 {
        8
    }

    fn part1(input: &str) -> Self::Output {
        let (size, input) = parse(input);

        let horizontal = process_1d(&input);
        let vertical = transpose(size, process_1d(&transpose(size, input)));

        horizontal.iter().flatten().zip(vertical.iter().flatten()).filter(|(&horizontal, &vertical)| horizontal || vertical).count()
    }

    fn part2(input: &str) -> Self::Output {
        let (size, input) = parse(input);

        let mut score_highest = 0;
        for y in 0..size {
            for x in 0..size {
                let height = input[y][x];

                let mut distance_left = 0;
                let mut distance_right = 0;
                let mut distance_up = 0;
                let mut distance_down = 0;

                for c in (0..x).rev() {
                    distance_left += 1;
                    if input[y][c] >= height {
                        break;
                    }
                }

                for c in x+1..size {
                    distance_right += 1;
                    if input[y][c] >= height {
                        break;
                    }
                }

                for r in (0..y).rev() {
                    distance_up += 1;
                    if input[r][x] >= height {
                        break;
                    }
                }

                for r in y+1..size {
                    distance_down += 1;
                    if input[r][x] >= height {
                        break;
                    }
                }

                let score = distance_up * distance_left * distance_down * distance_right;

                if score > score_highest {
                    score_highest = score
                }
            }
        }

        score_highest
    }
}
