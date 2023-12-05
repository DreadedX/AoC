#![feature(iter_map_windows)]
#![feature(test)]
use std::{
    cmp::max,
    collections::VecDeque,
    ops::{Add, BitAnd, BitOr},
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
    fn part1_test2() -> Result<()> {
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
    pub fn new(start: isize, length: isize) -> Self {
        Self { start, length }
    }

    pub fn end(&self) -> isize {
        self.start + self.length
    }

    pub fn offset(&self, offset: isize) -> Self {
        Self {
            start: self.start + offset,
            length: self.length,
        }
    }

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

    pub fn exclude(&self, rhs: &Self) -> VecDeque<Self> {
        if let Some(overlap) = self.overlap(rhs) {
            if rhs.start <= self.start && rhs.end() >= self.end() {
                vec![].into()
            } else if rhs.start <= self.start {
                let start = rhs.end();
                let length = self.end() - start;

                vec![Range::new(start, length)].into()
            } else if rhs.end() >= self.end() {
                let start = self.start;
                let length = rhs.start - start;

                vec![Range::new(start, length)].into()
            } else {
                let a = Range::new(self.start, overlap.start - self.start);
                let b = Range::new(overlap.end(), self.end() - overlap.end());

                vec![a, b].into()
            }
        } else {
            vec![*self].into()
        }
    }
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
        let mut split = input.split("\n\n");
        let mut seeds: Vec<_> = split
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split_whitespace()
            .map(|seed| seed.parse::<isize>().unwrap())
            .collect();

        for _ in 0..7 {
            let map = split
                .next()
                .unwrap()
                .lines()
                .skip(1)
                .map(|line| {
                    let mut split = line.splitn(3, ' ');
                    (
                        split.next().unwrap().parse::<isize>().unwrap(),
                        split.next().unwrap().parse::<isize>().unwrap(),
                        split.next().unwrap().parse::<isize>().unwrap(),
                    )
                })
                .collect::<Vec<_>>();

            seeds = seeds
                .into_iter()
                .map(|mut seed| {
                    for mapping in map.iter() {
                        if seed >= mapping.1 && seed < (mapping.1 + mapping.2) {
                            seed = seed - mapping.1 + mapping.0;
                            break;
                        }
                    }

                    seed
                })
                .collect();
        }

        *seeds.iter().min().unwrap()
    }

    fn part2(input: &str) -> Self::Output2 {
        let mut split = input.split("\n\n");
        let mut seeds: VecDeque<_> = split
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split_whitespace()
            .map(|seed| seed.parse::<isize>().unwrap())
            .map_windows(|range: &[isize; 2]| Range::new(range[0], range[1]))
            .step_by(2)
            .collect();

        for _ in 0..7 {
            let map = split
                .next()
                .unwrap()
                .lines()
                .skip(1)
                .map(|line| {
                    let mut split = line.splitn(3, ' ');
                    let remap = split.next().unwrap().parse::<isize>().unwrap();
                    let start = split.next().unwrap().parse::<isize>().unwrap();
                    let length = split.next().unwrap().parse::<isize>().unwrap();

                    (Range::new(start, length), remap - start)
                })
                .collect::<Vec<_>>();

            let mut result = VecDeque::new();

            'outer: while let Some(seed) = seeds.pop_front() {
                for mapping in map.iter() {
                    if let Some(overlap) = seed.overlap(&mapping.0) {
                        result.push_back(overlap.offset(mapping.1));
                        seeds.append(&mut seed.exclude(&mapping.0));

                        continue 'outer;
                    }
                }

                result.push_back(seed);
            }

            seeds = result;
        }

        seeds.iter().map(|range| range.start).min().unwrap()
    }
}
