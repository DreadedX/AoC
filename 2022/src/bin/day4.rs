#![feature(test)]
use std::cmp;
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
        Day::test(aoc::Part::ONE, "test-1", 2)
    }
    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(aoc::Part::ONE, "input", 567)
    }
    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(aoc::Part::TWO, "test-1", 4)
    }
    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(aoc::Part::TWO, "input", 907)
    }
}

// -- Benchmarks --
#[cfg(test)]
mod bench {
    use super::*;

    // Benchmarks
    extern crate test;
    #[bench]
    #[ignore]
    fn part1_solution(b: &mut test::Bencher) {
        Day::benchmark(aoc::Part::ONE, b)
    }
    #[bench]
    #[ignore]
    fn part2_solution(b: &mut test::Bencher) {
        Day::benchmark(aoc::Part::TWO, b)
    }
}

// -- Implementation --
struct Elf {
    start: u32,
    end: u32,
}

impl Elf {
    fn new(a: u32, b: u32) -> Elf {
        Elf { start: a, end: b }
    }
}

// -- Helpers --
// Check if one of the elf is fully contained by the other
fn contains((a, b): &(Elf, Elf)) -> bool {
    (a.start <= b.start && a.end >= b.end) || (b.start <= a.start && b.end >= a.end)
}

// Check if there is overlap between two elfs
fn overlaps((a, b): &(Elf, Elf)) -> bool {
    cmp::min(a.end, b.end) >= cmp::max(a.start, b.start)
}

// Transform from line to pair of Elfs
fn transform(s: &str) -> (Elf, Elf) {
    let transformed = s
        .replace("-", ",")
        .split(',')
        .flat_map(|value| value.parse::<u32>())
        .collect::<Vec<_>>();

    assert_eq!(transformed.len(), 4, "Invalid input");

    if let [a, b, c, d] = transformed[..4] {
        (Elf::new(a,b), Elf::new(c, d))
    } else {
        panic!("Invalid input")
    }
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output = u32;
    fn day() -> u8 {
        4
    }

    fn part1(input: &str) -> Self::Output {
        input
            .lines()
            .map(transform)
            .filter(contains)
            .count() as u32
    }

    fn part2(input: &str) -> Self::Output {
        input
            .lines()
            .map(transform)
            .filter(overlaps)
            .count() as u32
    }
}
