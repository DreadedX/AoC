#![feature(test)]
use std::{collections::{HashMap, VecDeque, HashSet}, str::FromStr, ops::Add};

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
        Day::test(Day::part1, "test-1", 18)
    }
    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 279)
    }
    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(Day::part2, "test-1", 54)
    }
    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", 762)
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
}

impl Add for &Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct State {
    position: Vec2,
    time_passed: usize
}

#[derive(Debug)]
struct Map {
    storm: HashMap<Vec2, Direction>,
    size: Vec2,
}

impl Map {
    fn pathfind(&self, start: Vec2, end: Vec2, time_passed: usize) -> usize {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        // @TODO This might actually not be 1 if, we need to calculate the first time for which it
        // is not blocked
        queue.push_back(State{ position: start, time_passed });

        let neighbours = vec![
            Vec2::new(0, -1),
            Vec2::new(0, 1),
            Vec2::new(-1, 0),
            Vec2::new(1, 0),
        ];

        while queue.len() > 0 {
            let current = queue.pop_front().unwrap();

            // Check every neighbour
            for neighbour in &neighbours {
                let next = &current.position + neighbour;
                // Neighbour goes out of bounds
                if next.x < 0 || next.x >= self.size.x || next.y < 0 || next.y >= self.size.y {
                    continue;
                }

                // Check if the space is free of blizzards
                let next = State { position: next, time_passed: current.time_passed+1 };
                if !visited.contains(&next) && self.check_for_blizzards(&next) {
                    visited.insert(next);
                    queue.push_back(next);
                }
            }

            // Stay in our current place
            let mut next = current;
            next.time_passed += 1;
            if !visited.contains(&next) && self.check_for_blizzards(&next) {
                visited.insert(next);
                queue.push_back(next);
            }

            // Reached the end, return the time it took us to get there
            if current.position.x == end.x && current.position.y == end.y  {
                return current.time_passed + 1;
            }
        }

        panic!("Ran out before finding end");
    }

    // Returns true if the space is free next turn
    fn check_for_blizzards(&self, state: &State) -> bool {
        for x in 0..self.size.x {
            let check = Vec2::new(x, state.position.y);

            // Find all the blizzards on the same y axis as the position
            if let Some(blizzard) = self.storm.get(&check) {
                match blizzard {
                    Direction::Left => {
                        // Project the blizzard forward in time
                        if (x - state.time_passed as isize).rem_euclid(self.size.x) == state.position.x {
                            return false;
                        }
                    },
                    Direction::Right => {
                        // Project the blizzard forward in time
                        if (x + state.time_passed as isize).rem_euclid(self.size.x) == state.position.x {
                            return false;
                        }
                    },
                    _ => {}
                }
            }
        }

        for y in 0..self.size.y {
            let check = Vec2::new(state.position.x, y);

            // Find all the blizzards on the same x axis as the position
            if let Some(blizzard) = self.storm.get(&check) {
                match blizzard {
                    Direction::Up => {
                        // Project the blizzard forward in time
                        if (y - state.time_passed as isize).rem_euclid(self.size.y) == state.position.y {
                            return false;
                        }
                    },
                    Direction::Down => {
                        // Project the blizzard forward in time
                        if (y + state.time_passed as isize).rem_euclid(self.size.y) == state.position.y {
                            return false;
                        }
                    },
                    _ => {}
                }
            }
        }

        return true;
    }
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let size_y = input.lines().count()-2;
        let size_x = input.lines().next().unwrap().len()-2;

        let size = Vec2::new(size_x as isize, size_y as isize);

        let storm = input
            .lines()
            .skip(1)
            .enumerate()
            .take_while(|(_y, line)| !line.starts_with("##"))
            .flat_map(|(y, line)| {
                line
                    .chars()
                    .skip(1)
                    .enumerate()
                    .take_while(|(_x, c)| *c != '#')
                    .flat_map(move |(x, c)| {
                        match c {
                            '^' => Some((Vec2::new(x as isize, y as isize), Direction::Up)),
                            'v' => Some((Vec2::new(x as isize, y as isize), Direction::Down)),
                            '<' => Some((Vec2::new(x as isize, y as isize), Direction::Left)),
                            '>' => Some((Vec2::new(x as isize, y as isize), Direction::Right)),
                            _ => None
                        }
                    })
            }).collect::<HashMap<Vec2, Direction>>();

        Ok(Self { storm, size })
    }
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = usize;
    type Output2 = usize;

    fn day() -> u8 {
        24
    }

    fn part1(input: &str) -> Self::Output1 {
        let map = Map::from_str(input).unwrap();

        let start = Vec2::new(0, -1);
        let end = &map.size + &Vec2::new(-1, -1);

        map.pathfind(start, end, 0)
    }

    fn part2(input: &str) -> Self::Output2 {
        let map = Map::from_str(input).unwrap();

        let start = Vec2::new(0, -1);
        let end = &map.size + &Vec2::new(-1, -1);

        let trip = map.pathfind(start, end, 0);
        let trip = map.pathfind(&end + &Vec2::new(0, 1), &start + &Vec2::new(0, 1), trip);
        map.pathfind(start, end, trip)
    }
}
