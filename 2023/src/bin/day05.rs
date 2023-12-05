#![feature(iter_map_windows)]
#![feature(test)]
use std::{
    cmp::max,
    collections::VecDeque,
};

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
        Day::test(Day::part1, "test-1", 35)
    }

    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 88151870)
    }

    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(Day::part2, "test-1", 46)
    }

    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", 2008785)
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

#[derive(Debug, Clone, Copy)]
struct Range {
    start: isize,
    length: isize,
}

impl Range {
    /// Create a new range
    pub fn new(start: isize, length: isize) -> Self {
        Self { start, length }
    }

    /// Get the position of the first element after the range
    pub fn end(&self) -> isize {
        self.start + self.length
    }

    /// Apply an offset of the range
    pub fn offset(&self, offset: isize) -> Self {
        Self {
            start: self.start + offset,
            length: self.length,
        }
    }

    /// Get the overlap between this range and another range.
    /// Returns [None] if there is no overlap
    pub fn overlap(&self, rhs: &Self) -> Option<Self> {
        let start = max(self.start, rhs.start);
        let end = std::cmp::min(self.end(), rhs.end());

        if end > start {
            Some(Range {
                start,
                length: end - start,
            })
        } else {
            None
        }
    }

    /// Exclude a different range from the current one.
    /// This returns a vector containing all ranges that form after excluding the different range
    /// from this range.
    /// The vector can be of size 0 if this range is included entirely in the different one.
    /// Size 2 if the different range is included entirely in this one.
    /// And size 1 if there is a partial overlap between the two ranges.
    pub fn exclude(&self, rhs: &Self) -> Vec<Self> {
        if let Some(overlap) = self.overlap(rhs) {
            if rhs.start <= self.start && rhs.end() >= self.end() {
                vec![]
            } else if rhs.start <= self.start {
                let start = rhs.end();
                let length = self.end() - start;

                vec![Range::new(start, length)]
            } else if rhs.end() >= self.end() {
                let start = self.start;
                let length = rhs.start - start;

                vec![Range::new(start, length)]
            } else {
                let a = Range::new(self.start, overlap.start - self.start);
                let b = Range::new(overlap.end(), self.end() - overlap.end());

                vec![a, b]
            }
        } else {
            vec![*self]
        }
    }

    /// Check if a value is contained in this range
    pub fn contains(&self, entry: isize) -> bool {
        entry >= self.start && entry < self.end()
    }
}

/// Parse a mapping block into a vector containing the each mapping range and the value that needs
/// to be added to perform the remap
fn parse_mapping_block(block: &str) -> Vec<(Range, isize)> {
    // Each line of the mapping block is formatted as
    // dst src len
    block
        // Take each line of the block
        .lines()
        // Skip the first line giving the name of the map
        .skip(1)
        .map(|line| {
            let mut split = line.splitn(3, ' ').map(|num| num.parse::<isize>().unwrap());
            let destination = split.next().unwrap();
            let source = split.next().unwrap();
            let length = split.next().unwrap();

            // To move an entry from the source to the destination subtract the source and add the
            // destination, adding the remap value will do this
            let remap = destination - source;

            (Range::new(source, length), remap)
        })
        .collect()
}

/// Process the input containing the seeds into a interator of all seed numbers
fn process_seeds(seeds: &str) -> impl Iterator<Item = isize> + '_ {
    seeds
        // Split the string at the whitespace
        .split_whitespace()
        // The first part contains "seed: " which we can ignore
        .skip(1)
        // Parse the numbers
        .map(|num| num.parse().unwrap())
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = isize;
    type Output2 = isize;

    fn day() -> u8 {
        5
    }

    fn part1(input: &str) -> Self::Output1 {
        // Split all the input blocks
        let mut split = input.split("\n\n");

        // Get a list of all seeds
        let mut seeds: Vec<_> = process_seeds(split.next().unwrap()).collect();

        // There are 7 mapping blocks, so loop 7 times
        for _ in 0..7 {
            let map = parse_mapping_block(split.next().unwrap());

            // Update the list of seeds
            seeds = seeds
                .into_iter()
                .map(|seed| {
                    // For each mapping check if the seed is in the mapping, if it is remap it
                    for mapping in map.iter() {
                        if mapping.0.contains(seed) {
                            return seed + mapping.1;
                        }
                    }

                    // Otherwise keep the current value
                    seed
                })
                .collect();
        }

        // Get the minimum value
        *seeds.iter().min().unwrap()
    }

    fn part2(input: &str) -> Self::Output2 {
        // Split all the input blocks
        let mut split = input.split("\n\n");

        // Get a list of all seeds
        let mut seeds: VecDeque<_> = process_seeds(split.next().unwrap())
            // Take the entries pairwise and construct a range
            .map_windows(|range: &[isize; 2]| Range::new(range[0], range[1]))
            // Make sure we move over 2 every time instead of only 1
            .step_by(2)
            .collect();

        // There are 7 mapping blocks, so loop 7 times
        for _ in 0..7 {
            let map = parse_mapping_block(split.next().unwrap());

            // Output list
            let mut result = VecDeque::new();

            // Keep processing the input list until it is empty
            'outer: while let Some(seed) = seeds.pop_front() {
                for mapping in map.iter() {
                    // For each mapping check if there is overlap
                    if let Some(overlap) = seed.overlap(&mapping.0) {
                        // Remap the overlapping part and add it to the output list
                        result.push_back(overlap.offset(mapping.1));

                        // Add the remainder back to the input list
                        seeds.append(&mut seed.exclude(&mapping.0).into());

                        // Continue with the next input
                        continue 'outer;
                    }
                }

                // There was no overlap with any of the mappings, the current range can be added to the output list
                result.push_back(seed);
            }

            // The input for the next iterator is the output of this iteration
            seeds = result;
        }

        // Find the lowest seed value
        seeds
            .iter()
            // The start of each range is the lowest value of that range
            .map(|range| range.start)
            // Get the minimum value
            .min()
            .unwrap()
    }
}
