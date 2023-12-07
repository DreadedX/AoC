#![feature(test)]

use std::cmp::max;

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
        Day::test(Day::part1, "test-1", 6440)
    }

    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 247823654)
    }

    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(Day::part2, "test-1", 5905)
    }

    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", 245461700)
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

// Assign each card a value
fn card_value(card: &char, part2: bool) -> i32 {
    match card {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => {
            if part2 {
                1
            } else {
                11
            }
        }
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => unreachable!(),
    }
}

fn hand_value(cards: &[i32], part2: bool) -> usize {
    if part2 {
        (2..=14)
            .map(|joker| {
                (2..=14)
                    .map(|index| {
                        // Count how many we have of that card, after replacing the joker
                        let count = cards
                            .iter()
                            .map(|card| if *card == 1 { joker } else { *card })
                            .filter(|card| *card == index)
                            .count();

                        // Partition a number into segments of 3 bits
                        // First three bits is amount of high cards, seconds is the amount of pairs, etc
                        // This should rank all cards based on type (ties still need to be resolved)
                        1 << (count * 3)
                    })
                    .sum()
            })
            .max()
            .unwrap()
    } else {
        // Loop over all possible cards
        (2..=14)
            .map(|index| {
                // Count how many we have of that card
                let count = cards.iter().filter(|card| **card == index).count();
                // Partition a number into segments of 3 bits
                // First three bits is amount of high cards, seconds is the amount of pairs, etc
                // This should rank all cards based on type (ties still need to be resolved)
                1 << (count * 3)
            })
            .sum()
    }
}

fn solve(input: &str, part2: bool) -> usize {
    let mut hands: Vec<_> = input
        .lines()
        .map(|line| {
            // Parse each line
            let (cards, bid) = line.split_once(' ').unwrap();
            let cards: Vec<_> = cards.chars().map(|card| card_value(&card, part2)).collect();
            let bid: usize = bid.parse().unwrap();

            // Calculate the value of the hand
            let score = hand_value(&cards, part2);

            (score, cards, bid)
        })
        .collect();

    // Sort all the hands
    hands.sort_by(|a, b| {
        // Check if the score are equal
        if a.0 == b.0 {
            // If they are, sort by the card values
            a.1.cmp(&b.1)
        } else {
            // Otherwise sort by score
            a.0.cmp(&b.0)
        }
    });

    // Calculate the total winnings
    hands
        .iter()
        .enumerate()
        .map(|(index, (_, _, bid))| bid * (index + 1))
        .sum()
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = usize;
    type Output2 = usize;

    fn day() -> u8 {
        7
    }

    fn part1(input: &str) -> Self::Output1 {
        solve(input, false)
    }

    fn part2(input: &str) -> Self::Output2 {
        solve(input, true)
    }
}
