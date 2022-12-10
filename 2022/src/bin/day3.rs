#![feature(test)]
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
        Day::test(Day::part1, "test-1", 157)
    }
    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 8298)
    }
    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(Day::part2, "test-1", 70)
    }
    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", 2708)
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
        Day::benchmark(Day::part1, b)
    }
    #[bench]
    #[ignore]
    fn part2_solution(b: &mut test::Bencher) {
        Day::benchmark(Day::part2, b)
    }
}

// -- Helpers --
fn convert(c: char) -> u32 {
    let result = match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => panic!("Invalid input"),
    };
    result
}

// Very nice alternative way to find the overlap
// Based on something I say online
// Part 1 could also be adapted to use this, but for historical sake I will leave that as is
// Part 2 was originally done the same as part 1, just with an extra condition in the if
fn find_common(group: &[&str]) -> char {
    let mut common = group[0].chars().collect::<Vec<_>>();

    // Only keep characters that appear in all other strings
    common.retain(|&c| {
        group[1..].iter().all(|g| g.contains(c))
    });

    *common.first().expect("Should be one overlap")
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = u32;
    type Output2 = u32;
    fn day() -> u8 {
        3
    }

    fn part1(input: &str) -> Self::Output1 {
        input.lines()
            .map(|line| line.split_at(line.len()/2))
            .map(|(a, b)| {
                for c in a.chars() {
                    // There is always one character in common between the two sides
                    if b.contains(c) {
                        return c;
                    }
                }
                unreachable!("No characters in common, this should never happen")
            })
            .map(convert)
            .sum()
    }

    fn part2(input: &str) -> Self::Output2 {
        input.lines()
            .collect::<Vec<_>>()
            .chunks(3)
            .map(find_common)
            .map(convert)
            .sum()
    }
}
