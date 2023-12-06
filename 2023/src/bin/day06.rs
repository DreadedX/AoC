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
        Day::test(Day::part1, "test-1", 288)
    }

    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 440000)
    }

    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(Day::part2, "test-1", 71503)
    }

    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", 26187338)
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

// distance = x * (time - x) = -x^2 + x*time > record
// Solving for -x^2 + x*time - record = 0 gives times between which we win
// which we win
fn number_of_ways(time: f64, record: f64) -> usize {
    let a = -1.0;
    let b = time;
    let c = -record;

    let one = (-b + (b.powi(2) - 4.0 * a * c).sqrt()) / (2.0 * a);
    let two = (-b - (b.powi(2) - 4.0 * a * c).sqrt()) / (2.0 * a);

    (two.ceil() - one.floor() - 1.0) as usize
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = usize;
    type Output2 = usize;

    fn day() -> u8 {
        6
    }

    fn part1(input: &str) -> Self::Output1 {
        let mut lines = input.lines();

        let times = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|num| num.parse().unwrap());

        let records = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|num| num.parse().unwrap());

        times
            .zip(records)
            .map(|(time, record)| number_of_ways(time, record))
            .product()
    }

    fn part2(input: &str) -> Self::Output2 {
        let mut lines = input.lines();

        let time = lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .trim()
            .replace(' ', "")
            .parse()
            .unwrap();

        let record = lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .trim()
            .replace(' ', "")
            .parse()
            .unwrap();

        number_of_ways(time, record)
    }
}
