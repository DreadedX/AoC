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
fn card_value(card: &char, part2: bool) -> u64 {
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

fn hand_value(cards: &[u64], part2: bool) -> u64 {
    // Partition the base into segments of four bits, where the first four represent the value of
    // the right most card, the next four bits the second card from the right, etc
    let base: u64 = cards
        .iter()
        .rev()
        .enumerate()
        .map(|(index, card)| card << (index * 4))
        .sum();

    // Count how many of each card we have
    let mut counts: Vec<_> = (1..=14)
        .map(|index| cards.iter().filter(|card| **card == index).count())
        .collect();

    if part2 {
        // Find of which card we have the most
        let index_most = counts
            .iter()
            .enumerate()
            .skip(1)
            .max_by_key(|(_, count)| **count)
            .unwrap()
            .0;

        // Add the jokes to that count
        counts[index_most] += counts[0];
        counts[0] = 0;
    };

    // Partition the rank into segments of 3 bits, lowest three bits is amount of single cards,
    // next three bits is amount of pairs, etc
    let rank: u64 = counts
        .iter()
        .map(|count| {
            if *count > 0 {
                1 << ((count - 1) * 3)
            } else {
                0
            }
        })
        .sum();

    // Shift over the rank and add the base two it
    // Sorting by this number should rank all the cards according to the rules
    (rank << (5 * 4)) + base
}

fn solve(input: &str, part2: bool) -> u64 {
    // Parse each hand
    let mut hands: Vec<_> = input
        .lines()
        .map(|line| {
            // Parse each line
            let (cards, bid) = line.split_once(' ').unwrap();
            let cards: Vec<_> = cards.chars().map(|card| card_value(&card, part2)).collect();
            let bid: u64 = bid.parse().unwrap();

            // Calculate the value of the hand
            let value = hand_value(&cards, part2);

            (value, bid)
        })
        .collect();

    // Sort all the hands
    hands.sort_by(|a, b| a.0.cmp(&b.0));

    // Calculate the total winnings
    hands
        .iter()
        .enumerate()
        .map(|(index, (_, bid))| bid * (index as u64 + 1))
        .sum()
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = u64;
    type Output2 = u64;

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
