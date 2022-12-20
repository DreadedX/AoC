#![feature(test)]
use core::fmt;
use std::str::FromStr;

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
        Day::test(Day::part1, "test-1", 3)
    }
    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 4914)
    }
    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(Day::part2, "test-1", 1623178306)
    }
    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", 7973051839072)
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

#[derive(Debug)]
struct Entry {
    value: isize,
    index: isize,
}

impl Entry {
    fn new(value: isize, index: isize) -> Self {
        Self {
            value,
            index,
        }
    }
}

#[derive(Debug)]
struct List {
    entries: Vec<Entry>,
}

impl List {
    fn len(&self) -> usize {
        self.entries.len()
    }

    fn get(&self, idx: isize) -> &Entry {
        let idx = idx.rem_euclid(self.entries.len() as isize);

        &self.entries[idx as usize]
    }

    fn get_raw(&self, idx: isize) -> &Entry {
        &self.entries[idx as usize]
    }

    fn remove(&mut self, idx: isize) -> Entry {
        let idx = idx.rem_euclid(self.entries.len() as isize);

        self.entries.remove(idx as usize)
    }

    fn insert(&mut self, idx: isize, entry: Entry) {
        let idx = idx.rem_euclid(self.entries.len() as isize);

        self.entries.insert(idx as usize, entry);
    }

    // @TODO This is not very fast, should probably find a better way of doing this
    fn new_idx(&self, idx: isize) -> Result<isize, anyhow::Error> {
        for (i, entry) in self.entries.iter().enumerate() {
            if entry.index == idx {
                return Ok(i as isize);
            }
        }

        return Err(anyhow::anyhow!("Unable to find entry with original index {}", idx));
    }
}

impl FromStr for List {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let entries = input
            .trim()
            .lines()
            .map(|value| value.parse().unwrap())
            .enumerate()
            .map(|(idx, value)| Entry::new(value, idx as isize))
            .collect::<Vec<_>>();

        Ok(Self { entries })
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for entry in self.entries.iter() {
            write!(f, "{}, ", entry.value)?;
        }
        write!(f, "]")
    }
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = isize;
    type Output2 = isize;

    fn day() -> u8 {
        20
    }

    fn part1(input: &str) -> Self::Output1 {
        let mut list = List::from_str(input).unwrap();

        for idx in 0..list.len() {
            let idx = idx as isize;
            let idx = list.new_idx(idx).unwrap();

            // Take out the element from the list
            let entry = list.remove(idx);
            // Insert at the new index

            list.insert(idx + entry.value, entry);
        }

        for idx in 0..list.len() {
            let idx = idx as isize;
            if list.get_raw(idx).value == 0 {
                return list.get(idx + 1000).value + list.get(idx + 2000).value + list.get(idx + 3000).value;
            }
        }

        return 0;
    }

    fn part2(input: &str) -> Self::Output2 {
        let mut list = List::from_str(input).unwrap();

        let decryption_key = 811589153;
        list.entries.iter_mut().for_each(|entry| entry.value *= decryption_key);

        for _ in 0..10 {
            for idx in 0..list.len() {
                let idx = idx as isize;
                let idx = list.new_idx(idx).unwrap();

                // Take out the element from the list
                let entry = list.remove(idx);
                // Insert at the new index

                list.insert(idx + entry.value, entry);
            }
        }

        for idx in 0..list.len() {
            let idx = idx as isize;
            if list.get_raw(idx).value == 0 {
                return list.get(idx + 1000).value + list.get(idx + 2000).value + list.get(idx + 3000).value;
            }
        }

        return 0;
    }
}
