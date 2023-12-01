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
        Day::test(Day::part1, "test-1", 142)
    }

    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 54968)
    }

    #[test]
    fn part2_test2() -> Result<()> {
        Day::test(Day::part2, "test-2", 281)
    }

    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", 54094)
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
    type Output1 = u32;
    type Output2 = u32;

    fn day() -> u8 {
        1
    }

    fn part1(input: &str) -> Self::Output1 {
        input
            .lines()
            .map(|line| {
                let mut nums = line.chars().filter_map(|c| c.to_digit(10));
                // There is always at least one number in every line
                let first = nums.next().unwrap();
                // If there is only one number use the first number as the last number
                let last = nums.last().unwrap_or(first);

                first * 10 + last
            })
            .sum()
    }

    fn part2(input: &str) -> Self::Output2 {
        // NOTE: Spelled out numbers can overlap
        // To make this substitution work correctly we keep the first and last letter of every
        // spelled out number
        let input = input.replace("one", "o1e");
        let input = input.replace("two", "t2e");
        let input = input.replace("three", "t3e");
        let input = input.replace("four", "f4r");
        let input = input.replace("five", "f5e");
        let input = input.replace("six", "s6x");
        let input = input.replace("seven", "s7n");
        let input = input.replace("eight", "e8t");
        let input = input.replace("nine", "n9e");

        Self::part1(&input)
    }
}
