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
        Day::test(Day::part1, "test-1", 13)
    }

    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 24706)
    }

    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(Day::part2, "test-1", 30)
    }

    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", 13114317)
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
    type Output1 = usize;
    type Output2 = usize;

    fn day() -> u8 {
        4
    }

    fn part1(input: &str) -> Self::Output1 {
        // Calculate the score for every game and sum it
        input
            .lines()
            .map(|line| {
                // Get rid of the first part
                let (_, line) = line
                    .split_once(": ")
                    .expect("Input should be formatted properly");

                // Seperate the winning numbers and numbers we have
                let (winning, numbers) = line
                    .split_once(" | ")
                    .expect("Input should be formatted properly");

                // Parse the winning numbers
                let winning: Vec<_> = winning.split(' ').flat_map(str::parse::<usize>).collect();
                // Parse the numbers we have, check if they are a winning number and count the
                // amount of winning numbers we have
                let count = numbers
                    .split(' ')
                    .flat_map(str::parse::<usize>)
                    .filter(|num| winning.contains(num))
                    .count();

                // Calculate the score
                if count > 0 {
                    return 2_usize.pow(count as u32 - 1);
                }

                0
            })
            .sum()
    }

    fn part2(input: &str) -> Self::Output2 {
        // Start out with one copy of every card
        let mut copies = vec![1; input.lines().count()];

        input.lines().enumerate().for_each(|(i, line)| {
            // Get rid of the first part
            let (_, line) = line
                .split_once(": ")
                .expect("Input should be formatted properly");

            // Seperate the winning numbers and numbers we have
            let (winning, numbers) = line
                .split_once(" | ")
                .expect("Input should be formatted properly");

            // Parse the winning numbers
            let winning: Vec<_> = winning.split(' ').flat_map(str::parse::<usize>).collect();
            // Parse the numbers we have, check if they are a winning number and count the
            // amount of winning numbers we have
            let wins = numbers
                .split(' ')
                .flat_map(str::parse::<usize>)
                .filter(|num| winning.contains(num))
                .count();

            // Increment the amount that we have of the cards that we win by the number we have of
            // the current card
            for j in (i + 1)..(i + wins + 1) {
                // e.g. we have 2 copies of the current card that has 3 wins, this means we add 2
                // to the next 3 cards
                copies[j] += copies[i]
            }
        });

        copies.iter().sum()
    }
}
