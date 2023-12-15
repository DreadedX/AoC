#![feature(test)]
use std::cmp::Ordering;

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
        Day::test(Day::part1, "test-1", 1320)
    }

    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 511416)
    }

    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(Day::part2, "test-1", 145)
    }

    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", 290779)
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

#[derive(Debug, Clone, PartialEq, Eq)]
enum Action<'a> {
    Add { label: &'a str, focal_length: usize },
    Remove { label: &'a str },
}

impl<'a> Action<'a> {
    fn index(&self) -> usize {
        match self {
            Action::Add { label, .. } | Action::Remove { label } => label
                .chars()
                .map(|c| c as usize)
                .fold(0, |acc, num| ((acc + num) * 17) % 256),
        }
    }
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = usize;
    type Output2 = usize;

    fn day() -> u8 {
        15
    }

    fn part1(input: &str) -> Self::Output1 {
        input
            .trim()
            .split(',')
            .map(|sequence| {
                sequence
                    .chars()
                    .map(|c| c as usize)
                    .fold(0, |acc, num| ((acc + num) * 17) % 256)
            })
            .sum()
    }

    fn part2(input: &str) -> Self::Output2 {
        let actions: Vec<_> = input
            .trim()
            .split(',')
            .map(|sequence| {
                if sequence.ends_with('-') {
                    let label = sequence.split_once('-').unwrap().0;

                    Action::Remove { label }
                } else {
                    let (label, focal_length) = sequence.split_once('=').unwrap();

                    let focal_length = focal_length.parse().unwrap();

                    Action::Add {
                        label,
                        focal_length,
                    }
                }
            })
            .collect();

        let mut boxes: [Vec<(&str, usize)>; 256] = std::array::from_fn(|_| Vec::new());

        for action in &actions {
            let index = action.index();

            match action {
                Action::Add {
                    label,
                    focal_length,
                } => {
                    // NOTE: Using a library like OrderedMap would make this trivially easy
                    if let Some(position) = boxes[index]
                        .iter()
                        .position(|lens| lens.0.cmp(label) == Ordering::Equal)
                    {
                        // If a lens with this label is already in the box, replace it
                        boxes[index][position].1 = *focal_length;
                    } else {
                        // Otherwise add it to the end of the box
                        boxes[index].push((label, *focal_length));
                    }
                }
                Action::Remove { label } => {
                    if let Some(position) = boxes[index]
                        .iter()
                        .position(|lens| lens.0.cmp(label) == Ordering::Equal)
                    {
                        // A lens with the label exists, remove it
                        boxes[index].remove(position);
                    }
                }
            }
        }

        boxes
            .iter()
            .enumerate()
            .map(|(index, b)| {
                b.iter()
                    .enumerate()
                    .map(|(position, (_, focal_length))| {
                        (1 + index) * (1 + position) * focal_length
                    })
                    .sum::<usize>()
            })
            .sum()
    }
}
