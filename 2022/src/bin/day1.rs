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
        Day::test(aoc::Part::ONE, "test-1", 24000)
    }
    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(aoc::Part::TWO, "test-1", 45000)
    }
    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(aoc::Part::ONE, "input", 70116)
    }
    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(aoc::Part::TWO, "input", 206582)
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

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output = u32;
    fn day() -> u8 {
        1
    }

    fn part1(input: &str) -> Self::Output {
        input.split("\n\n")
            .map(|elf| elf.lines()
                 .flat_map(|snack| snack.parse::<u32>())
                 .sum())
            .max()
            .unwrap()
    }

    fn part2(input: &str) -> Self::Output {
        let mut elfs: Vec<u32> = input.split("\n\n")
            .map(|elf| elf.lines()
                 .flat_map(|snack| snack.parse::<u32>())
                 .sum())
            .collect();

        elfs.sort_by(|a, b| b.cmp(a));

        elfs.iter().take(3).sum()
    }
}
