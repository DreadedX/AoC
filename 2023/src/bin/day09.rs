#![feature(test)]
#![feature(iter_map_windows)]
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
        Day::test(Day::part1, "test-1", 114)
    }

    #[test]
    fn part1_test2() -> Result<()> {
        Day::test(Day::part1, "test-2", 18)
    }

    #[test]
    fn part1_test3() -> Result<()> {
        Day::test(Day::part1, "test-3", 28)
    }

    #[test]
    fn part1_test4() -> Result<()> {
        Day::test(Day::part1, "test-4", 68)
    }

    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 1708206096)
    }

    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(Day::part2, "test-1", 2)
    }

    #[test]
    fn part2_test2() -> Result<()> {
        Day::test(Day::part2, "test-2", -3)
    }

    #[test]
    fn part2_test3() -> Result<()> {
        Day::test(Day::part2, "test-3", 0)
    }

    #[test]
    fn part2_test4() -> Result<()> {
        Day::test(Day::part2, "test-4", 5)
    }

    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", 1050)
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
    type Output1 = isize;
    type Output2 = isize;

    fn day() -> u8 {
        9
    }

    fn part1(input: &str) -> Self::Output1 {
        input
            .lines()
            .map(|line| {
                let mut nums: Vec<_> = line
                    .split_whitespace()
                    .map(|num| num.parse::<isize>().unwrap())
                    .collect();

                let mut next = *nums.last().unwrap();

                while nums.iter().any(|&num| num != 0) {
                    nums = nums.iter().map_windows(|[&a, &b]| b - a).collect();
                    next += nums.last().unwrap();
                }

                next
            })
            .sum()
    }

    fn part2(input: &str) -> Self::Output2 {
        input
            .lines()
            .map(|line| {
                let mut nums: Vec<_> = line
                    .split_whitespace()
                    .map(|num| num.parse::<isize>().unwrap())
                    .collect();

                let mut firsts = vec![*nums.first().unwrap()];
                while nums.iter().any(|&num| num != 0) {
                    nums = nums.iter().map_windows(|[&a, &b]| b - a).collect();
                    firsts.push(*nums.first().unwrap());
                }

                firsts.iter().rev().skip(1).fold(0, |diff, num| num - diff)
            })
            .sum()
    }
}
