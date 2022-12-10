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
        Day::test(Day::part1, "test-1", 21)
    }
    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 1845)
    }
    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(Day::part2, "test-1", 8)
    }
    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", 230112)
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
// -- Helpers --
fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).map(|d| d as i32).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn is_visible(highest: &mut i32, height: i32) -> Option<bool> {
    if height > *highest {
        *highest = height;
        Some(true)
    } else {
        Some(false)
    }
}

// Consume the vector and perform the transpose by swapping around elements
fn transpose<T: Copy + fmt::Display>( mut input: Vec<Vec<T>>) -> Vec<Vec<T>> {
    for y in 0..input.len() {
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

fn process_1d(input: &Vec<Vec<i32>>) -> Vec<Vec<bool>> {
    input.iter()
        .map(|row| {
            let mut right = row.iter()
                .rev()
                .scan(-1, |mut highest, &height| is_visible(&mut highest, height))
                .collect::<Vec<_>>();
            right.reverse();

            row.iter()
                .scan(-1, |mut highest, &height| is_visible(&mut highest, height))
                .zip(right.iter())
                .map(|(left, &right)| left || right)
                .collect::<Vec<_>>()
        }).collect()
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = usize;
    type Output2 = usize;
    fn day() -> u8 {
        8
    }

    fn part1(input: &str) -> Self::Output1 {
        let input = parse(input);

        let horizontal = process_1d(&input);
        let vertical = transpose(process_1d(&transpose(input)));

        horizontal.iter().flatten().zip(vertical.iter().flatten()).filter(|(&horizontal, &vertical)| horizontal || vertical).count()
    }

    fn part2(input: &str) -> Self::Output2 {
        let input = parse(input);

        let mut score_highest = 0;
        for y in 0..input.len() {
            for x in 0..input[y].len() {
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

                for c in x+1..input[y].len() {
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

                for r in y+1..input[y].len() {
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
