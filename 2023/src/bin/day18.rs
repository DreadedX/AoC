#![feature(test)]
use std::{
    collections::{HashSet, VecDeque},
    fmt::Pointer,
};

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
        Day::test(Day::part1, "test-1", 62)
    }

    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 95356)
    }

    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(Day::part2, "test-1", 952408144115)
    }

    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", 92291468914147)
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
        18
    }

    fn part1(input: &str) -> Self::Output1 {
        let instructions: Vec<_> = input
            .lines()
            .map(|line| {
                let (direction, rest) = line.split_once(' ').unwrap();
                let direction = direction.chars().next().unwrap();
                let (distance, _) = rest.split_once(" (#").unwrap();
                let distance: usize = distance.parse().unwrap();

                (direction, distance)
            })
            .collect();

        let mut location = (0_isize, 0_isize);
        let mut points = Vec::new();
        let mut boundary = 0;

        for (direction, distance) in &instructions {
            for _ in 0..*distance {
                match direction {
                    'U' => location.1 -= 1,
                    'D' => location.1 += 1,
                    'L' => location.0 -= 1,
                    'R' => location.0 += 1,
                    _ => unreachable!("Invalid input"),
                }
            }
            boundary += distance;
            points.push(location);
        }

        let mut area = 0;
        for i in 0..points.len() as isize {
            let n_min = (i - 1).rem_euclid(points.len() as isize);
            let n_plus = (i + 1).rem_euclid(points.len() as isize);

            area += points[i as usize].1 * (points[n_min as usize].0 - points[n_plus as usize].0);
        }
        let area = area as usize / 2;
        let interior = area - boundary / 2 + 1;

        interior + boundary
    }

    fn part2(input: &str) -> Self::Output2 {
        let instructions: Vec<_> = input
            .lines()
            .map(|line| {
                let (_, rest) = line.split_once(' ').unwrap();
                let (_, rest) = rest.split_once(" (#").unwrap();

                let (distance, direction) = rest.split_at(5);
                let distance = usize::from_str_radix(distance, 16).unwrap();
                let direction = direction.chars().next().unwrap();

                (direction, distance)
            })
            .collect();

        let mut location = (0_isize, 0_isize);
        let mut points = Vec::new();
        let mut boundary = 0;

        for (direction, distance) in &instructions {
            for _ in 0..*distance {
                match direction {
                    '3' => location.1 -= 1,
                    '1' => location.1 += 1,
                    '2' => location.0 -= 1,
                    '0' => location.0 += 1,
                    _ => unreachable!("Invalid input"),
                }
            }
            boundary += distance;
            points.push(location);
        }

        let mut area = 0;
        for i in 0..points.len() as isize {
            let n_min = (i - 1).rem_euclid(points.len() as isize);
            let n_plus = (i + 1).rem_euclid(points.len() as isize);

            area += points[i as usize].1 * (points[n_min as usize].0 - points[n_plus as usize].0);
        }
        let area = area as usize / 2;

        area + boundary / 2 + 1
    }
}
