use std::cmp;
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
        Day::test(aoc::Part::ONE, "test-1", 2)
    }
    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(aoc::Part::TWO, "test-1", 4)
    }
    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(aoc::Part::ONE, "input", 567)
    }
}

// -- Helper --
// @TODO Implement this on the iterator?
fn apply<F, U, V>((a, b): (U, U), f: F) -> (V, V) where
    F: Fn(U) -> V {
        (f(a), f(b))
}

// This filter check if either side is fully contained in the other side
fn contains(pair: &((u32, u32), (u32, u32))) -> bool {
    (pair.0.0 <= pair.1.0 && pair.0.1 >= pair.1.1) || (pair.1.0 <= pair.0.0 && pair.1.1 >= pair.0.1)
}

fn overlaps(pair: &((u32, u32), (u32, u32))) -> bool {
    if pair.0.1 >= pair.1.0 && pair.0.1 <= pair.1.1 {
        true
    } else if pair.0.0 >= pair.1.0 && pair.0.0 <= pair.1.1 {
        true
    } else if pair.1.1 >= pair.0.0 && pair.1.1 <= pair.0.1 {
        true
    } else if pair.1.0 >= pair.0.0 && pair.1.0 <= pair.0.1 {
        true
    } else {
        false
    }
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    fn day() -> u8 {
        4
    }

    fn part1(input: &str) -> u32 {
        input
            .lines()
            .map(|line| line.split_once(',').expect("Invalid input"))
            .map(|pair| apply(pair, |pair| pair.split_once("-").expect("Invalid input")))
            .map(|pair| apply(pair, |side| {
                apply(side, |a| {
                    if let Ok(num) = a.parse::<u32>() {
                        num
                    } else {
                        panic!("Invalid input")
                    }
                })
            }))
            .filter(contains)
            .count() as u32
    }

    fn part2(input: &str) -> u32 {
        input
            .lines()
            .map(|line| line.split_once(',').expect("Invalid input"))
            .map(|pair| apply(pair, |pair| pair.split_once("-").expect("Invalid input")))
            .map(|pair| apply(pair, |side| {
                apply(side, |a| {
                    if let Ok(num) = a.parse::<u32>() {
                        num
                    } else {
                        panic!("Invalid input")
                    }
                })
            }))
            .filter(overlaps)
            .count() as u32
    }
}
