#![feature(test)]
use core::fmt;
use std::{collections::HashSet, str::FromStr};

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
        Day::test(aoc::Part::ONE, "test-1", 13)
    }
    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(aoc::Part::ONE, "input", 5695)
    }
    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(aoc::Part::TWO, "test-1", 1)
    }
    #[test]
    fn part2_test2() -> Result<()> {
        Day::test(aoc::Part::TWO, "test-2", 36)
    }
    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(aoc::Part::TWO, "input", 2434)
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

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Calculates the distance apart on either axis and then returns the largest
    fn distance(&self, other: &Self) -> i32 {
        std::cmp::max((self.x - other.x).abs(), (self.y - other.y).abs())
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "R" => Ok(Direction::Right),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            _ => Err(anyhow::anyhow!("Invalid input")),
        }
    }
}

#[derive(Debug)]
struct Rope {
    elements: Vec<Position>,
    visited: HashSet<Position>,
}

impl Rope {
    fn new(length: usize) -> Self {
        let start = Position::new(0, 0);
        let mut visited = HashSet::new();
        visited.insert(start);

        let mut elements = Vec::new();
        for _ in 0..length {
            elements.push(start);
        }

        Self {elements, visited}
    }

    fn step(&mut self, d: Direction) {
        // Get a &mut to the first element
        let mut head = self.elements.first_mut().unwrap();

        // Update the first element
        match d {
            Direction::Up => head.y += 1,
            Direction::Right => head.x += 1,
            Direction::Down => head.y -= 1,
            Direction::Left => head.x -= 1,
        }

        // Head is now a copy of the position of the first element
        let mut head = *head;

        // Update all remaining elements of the rope
        for tail in self.elements.iter_mut().skip(1) {
            // Check if tail is touching the head
            if tail.distance(&head) > 1 {
                if tail.x < head.x {
                    tail.x += 1;
                } else if tail.x > head.x {
                    tail.x -= 1;
                }

                if tail.y < head.y {
                    tail.y += 1;
                } else if tail.y > head.y {
                    tail.y -= 1;
                }
            }

            head = *tail;
        }

        // Mark the location of the last element as visited
        self.visited.insert(*self.elements.last().unwrap());

    }
}

impl fmt::Display for Rope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // @TODO Automatically pick the right value for this
        let xmin = -15;
        let xmax = 15;
        let ymin = -10;
        let ymax = 10;

        for y in (ymin..ymax).rev() {
            for x in xmin..xmax {
                let pos = Position::new(x, y);
                let c;
                if *self.elements.first().unwrap() == pos {
                    c = 'H';
                } else if self.elements.contains(&pos) {
                    let index = self.elements.iter().position(|p| *p == pos).unwrap();
                    c = char::from(index as u8 + b'0');
                } else if self.visited.contains(&pos) {
                    c = '#';
                } else {
                    c = '.'
                }

                write!(f, "{}", c)?;
            }
            writeln!(f, "")?;
        }

        writeln!(f, "")
    }
}

fn parse(input: &str) -> Vec<(Direction, usize)> {
    input
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(d, c)| (d.parse().unwrap(), c.parse().unwrap()))
        .collect()
}

fn solution(input: &str, length: usize) -> usize {
    let instructions = parse(input);

    let mut rope = Rope::new(length);

    for instruction in instructions {
        for _ in 0..instruction.1 {
            rope.step(instruction.0);
        }
    }


    rope.visited.len()
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output = usize;
    fn day() -> u8 {
        9
    }

    fn part1(input: &str) -> Self::Output {
        solution(input, 2)
    }

    fn part2(input: &str) -> Self::Output {
        solution(input, 10)
    }
}
