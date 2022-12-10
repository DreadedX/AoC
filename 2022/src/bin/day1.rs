#![feature(test)]
use anyhow::Result;
use aoc::Solver;

// -- Runners --
fn main() -> Result<()> {
    Day::solve()
}

// -- Tests --
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() -> Result<()> {
        Day::test(Day::part1, "test-1", 24000)
    }
    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 70116)
    }
    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(Day::part2, "test-1", 45000)
    }
    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", 206582)
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

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = u32;
    type Output2 = u32;
    fn day() -> u8 {
        1
    }

    fn part1(input: &str) -> Self::Output1 {
        input.split("\n\n")
            .map(|elf| elf.lines()
                 .flat_map(|snack| snack.parse::<u32>())
                 .sum())
            .max()
            .unwrap()
    }

    fn part2(input: &str) -> Self::Output2 {
        let mut elfs: Vec<u32> = input.split("\n\n")
            .map(|elf| elf.lines()
                 .flat_map(|snack| snack.parse::<u32>())
                 .sum())
            .collect();

        elfs.sort_by(|a, b| b.cmp(a));

        elfs.iter().take(3).sum()
    }
}
