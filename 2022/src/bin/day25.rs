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
        Day::test(Day::part1, "test-1", "2=-1=0".to_owned())
    }
    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", "2-20=01--0=0=0=2-120".to_owned())
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
    type Output1 = String;
    type Output2 = String;

    fn day() -> u8 {
        25
    }

    fn part1(input: &str) -> Self::Output1 {
        let mut sum: isize = input
            .trim()
            .lines()
            .map(|line| {
                line.chars().rev().enumerate().fold(0, |acc, (idx, c)| {
                    let num = match c {
                        '2' => 2,
                        '1' => 1,
                        '0' => 0,
                        '-' => -1,
                        '=' => -2,
                        _ => panic!("Invalid number!"),
                    };

                    acc + num * 5_isize.pow(idx as u32)
                })
            }).sum();

        let mut result = String::new();
        while sum > 0 {
            let mut carry = 0;
            let rem = sum % 5;
            match rem {
                0 => result += "0",
                1 => result += "1",
                2 => result += "2",
                3 => {
                    result += "=";
                    carry += 1;
                },
                4 => {
                    result += "-";
                    carry += 1;
                },
                _ => unreachable!("Number is mod 5 so should always be 0..=4"),
            }

            sum = sum/5 + carry;
        }
        // Reverse the string, since we construct it the wrong way around
        result = result.chars().rev().collect();

        result
    }

    fn part2(_input: &str) -> Self::Output2 {
        // There is no part 2!
        "Merry Christmas!".to_owned()
    }
}
