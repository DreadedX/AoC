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
        Day::test(aoc::Part::ONE, "test-1", 7)
    }
    #[test]
    fn part1_test2() -> Result<()> {
        Day::test(aoc::Part::ONE, "test-2", 5)
    }
    #[test]
    fn part1_test3() -> Result<()> {
        Day::test(aoc::Part::ONE, "test-3", 6)
    }
    #[test]
    fn part1_test4() -> Result<()> {
        Day::test(aoc::Part::ONE, "test-4", 10)
    }
    #[test]
    fn part1_test5() -> Result<()> {
        Day::test(aoc::Part::ONE, "test-5", 11)
    }
    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(aoc::Part::ONE, "input", 1275)
    }

    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(aoc::Part::TWO, "test-1", 19)
    }
    #[test]
    fn part2_test2() -> Result<()> {
        Day::test(aoc::Part::TWO, "test-2", 23)
    }
    #[test]
    fn part2_test3() -> Result<()> {
        Day::test(aoc::Part::TWO, "test-3", 23)
    }
    #[test]
    fn part2_test4() -> Result<()> {
        Day::test(aoc::Part::TWO, "test-4", 29)
    }
    #[test]
    fn part2_test5() -> Result<()> {
        Day::test(aoc::Part::TWO, "test-5", 26)
    }
    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(aoc::Part::TWO, "input", 3605)
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

// -- Helpers --
fn is_start_marker(window: &[u8]) -> bool {
    for i in 0..window.len() {
        for j in i+1..window.len() {
            if window[i] == window[j] {
                return false;
            }
        }
    }

    return true;
}

fn solution(input: &str, length: usize) -> usize {
    input
        .as_bytes()
        .windows(length)
        .position(is_start_marker)
        .expect("Invalid input") + length
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output = usize;
    fn day() -> u8 {
        6
    }

    fn part1(input: &str) -> Self::Output {
        solution(input, 4)
    }

    fn part2(input: &str) -> Self::Output {
        solution(input, 14)
    }
}
