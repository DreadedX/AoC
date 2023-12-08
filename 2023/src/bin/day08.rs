#![feature(test)]
use std::collections::HashMap;

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
        Day::test(Day::part1, "test-1", 2)
    }

    #[test]
    fn part1_test2() -> Result<()> {
        Day::test(Day::part1, "test-2", 6)
    }

    #[test]
    fn part2_test3() -> Result<()> {
        Day::test(Day::part2, "test-3", 6)
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

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

fn lcm(a: usize, b: usize) -> usize {
    if a > b {
        (a / gcd(a, b)) * b
    } else {
        (b / gcd(a, b)) * a
    }
}

enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => unreachable!("Invalid input"),
        }
    }
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
        let mut lines = input.lines();
        let mut directions = lines.next().unwrap().chars().map(Direction::from).cycle();

        let map: HashMap<String, (String, String)> = lines
            .skip(1)
            .map(|line| {
                // chars is a mutable reference, this prevents take from taking ownership of chars
                let chars = &mut line.chars();

                let location = chars.take(3).collect();
                let left = chars.skip(4).take(3).collect();
                let right = chars.skip(2).take(3).collect();

                (location, (left, right))
            })
            .collect();

        let mut steps = 0;
        let mut location = "AAA";
        while location != "ZZZ" {
            let next = map.get(location).unwrap();
            location = match directions.next().unwrap() {
                Direction::Left => &next.0,
                Direction::Right => &next.1,
            };
            steps += 1;
        }

        steps
    }

    fn part2(input: &str) -> Self::Output2 {
        let mut lines = input.lines();
        let directions = lines.next().unwrap().chars().map(Direction::from).cycle();

        let map: HashMap<String, (String, String)> = lines
            .skip(1)
            .map(|line| {
                // chars is a mutable reference, this prevents take from taking ownership of chars
                let chars = &mut line.chars();

                let location = chars.take(3).collect();
                let left = chars.skip(4).take(3).collect();
                let right = chars.skip(2).take(3).collect();

                (location, (left, right))
            })
            .collect();

        map.iter()
            .filter_map(|(key, _)| {
                if key.ends_with('A') {
                    Some(key.as_str())
                } else {
                    None
                }
            })
            .map(|mut location| {
                let mut steps = 0;
                let mut directions = directions.clone();
                while !location.ends_with('Z') {
                    let next = map.get(location).unwrap();
                    location = match directions.next().unwrap() {
                        Direction::Left => &next.0,
                        Direction::Right => &next.1,
                    };
                    steps += 1;
                }

                steps
            })
            .fold(1, lcm)
    }
}
