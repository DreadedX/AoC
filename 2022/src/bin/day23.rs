#![feature(test)]
use std::{collections::HashMap, str::FromStr};

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
        Day::test(Day::part1, "test-1", 25)
    }
    #[test]
    fn part1_test2() -> Result<()> {
        Day::test(Day::part1, "test-2", 110)
    }
    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 3849)
    }
    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(Day::part2, "test-1", 4)
    }
    #[test]
    fn part2_test2() -> Result<()> {
        Day::test(Day::part2, "test-2", 20)
    }
    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", 995)
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

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct Vec2 {
    x: isize,
    y: isize,
}

impl Vec2 {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn offset(&self, other: &Vec2) -> Self {
        Self { x: self.x+other.x, y: self.y+other.y }
    }
}

#[derive(Debug, Copy, Clone)]
struct Elf {
    considering: Vec2,
}

impl Elf {
    fn new() -> Self {
        Self { considering: Vec2::new(0, 0) }
    }
}

#[derive(Debug)]
struct Map {
    elves: HashMap<Vec2, Elf>,
    considered: HashMap<Vec2, usize>,
    offset: usize,
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let elves = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line
                    .chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '#')
                    .map(move |(x, _)| {
                        (Vec2::new(x as isize, y as isize), Elf::new())
                    })
            }).collect();

        Ok(Self { elves, considered: HashMap::new(), offset: 0})
    }
}

impl Map {
    fn consider(&mut self) {
        let north = ([Vec2::new(-1, -1), Vec2::new(0, -1), Vec2::new(1, -1)], Vec2::new(0, -1));
        let south = ([Vec2::new(-1, 1), Vec2::new(0, 1), Vec2::new(1, 1)], Vec2::new(0, 1));
        let west = ([Vec2::new(-1, -1), Vec2::new(-1, 0), Vec2::new(-1, 1)], Vec2::new(-1, 0));
        let east = ([Vec2::new(1, -1), Vec2::new(1, 0), Vec2::new(1, 1)], Vec2::new(1, 0));

        let moves = [north, south, west, east];

        for pos in self.elves.clone().keys() {
            // Default action is to stay in place
            let mut next = *pos;

            if self.need_to_move(&pos) {
                for m in moves.iter().cycle().skip(self.offset).take(4) {
                    if !self.elves.contains_key(&pos.offset(&m.0[0])) && !self.elves.contains_key(&pos.offset(&m.0[1])) && !self.elves.contains_key(&pos.offset(&m.0[2])) {
                        next = pos.offset(&m.1);
                        break;
                    }
                }
            }

            // Set the position that the elf is considering
            self.elves.get_mut(&pos).unwrap().considering = next;

            // Increment counter for that position
            *self.considered.entry(next).or_default() += 1;
        }
    }

    fn need_to_move(&self, pos: &Vec2) -> bool {
        // Check all the spaces around the elf to determine if we need to move
        for y in -1..2 {
            for x in -1..2 {
                if x == 0 && y == 0 {
                    continue;
                }

                if self.elves.contains_key(&pos.offset(&Vec2::new(x, y))) {
                    return true;
                }
            }
        }

        return false;
    }

    fn make_moves(&mut self) -> bool {
        let mut elves = HashMap::new();
        let mut moved = false;

        for (pos, elf) in &self.elves {
            // Check if the move is a valid move
            if *self.considered.get(&elf.considering).unwrap() == 1 {
                // Check if the position of the elf has actually changed
                if elf.considering.x != pos.x || elf.considering.y != pos.y {
                    moved = true;
                }

                // Insert the elf with the updated position
                elves.insert(elf.considering, *elf);
            } else {
                // Otherwise the elf will not move and stay at its old position
                elves.insert(*pos, *elf);
            }
        }

        self.elves = elves;
        self.considered.clear();
        self.offset += 1;

        return moved;
    }

    // Gives the min and max of the smallest rectangle containing all the elves
    fn get_size(&self) -> (Vec2, Vec2) {
        let mut min = Vec2::new(isize::MAX, isize::MAX);
        let mut max = Vec2::new(0, 0);

        for pos in self.elves.keys() {
            min.x = min.x.min(pos.x);
            min.y = min.y.min(pos.y);

            max.x = max.x.max(pos.x+1);
            max.y = max.y.max(pos.y+1);
        }

        (min, max)
    }

    // Calculate the amount of empty tiles in the smallest rectangle containing all the elves
    fn empty_tiles(&self) -> isize {
        let (min, max) = self.get_size();
        (max.x - min.x) * (max.y - min.y) - self.elves.len() as isize
    }

    // fn print(&self) {
    //     let (min, max) = self.get_size();
    //     for y in min.y..max.y {
    //         for x in min.x..max.x {
    //             if self.elves.contains_key(&Vec2::new(x, y)) {
    //                 print!("#");
    //             } else {
    //                 print!(".");
    //             }
    //         }
    //         println!("");
    //     }
    //     println!("");
    // }
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = isize;
    type Output2 = isize;

    fn day() -> u8 {
        23
    }

    fn part1(input: &str) -> Self::Output1 {
        let mut map = Map::from_str(input).unwrap();

        for _ in 0..10 {
            map.consider();
            map.make_moves();
        }

        map.empty_tiles()
    }

    fn part2(input: &str) -> Self::Output2 {
        let mut map = Map::from_str(input).unwrap();

        let mut counter = 0;
        loop {
            counter += 1;

            map.consider();
            if !map.make_moves() {
                return counter;
            }
        }
    }
}
