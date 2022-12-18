#![feature(test)]
use std::{collections::HashSet, ops::Add};

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
        Day::test(Day::part1, "test-1", 10)
    }
    #[test]
    fn part1_test2() -> Result<()> {
        Day::test(Day::part1, "test-2", 64)
    }
    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 3550)
    }
    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(Day::part2, "test-1", 10)
    }
    #[test]
    fn part2_test2() -> Result<()> {
        Day::test(Day::part2, "test-2", 58)
    }
    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", 2028)
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

#[derive(Debug, Hash, PartialEq, Eq)]
struct Vec3 {
    x: isize,
    y: isize,
    z: isize,
}

impl Vec3 {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }
}

impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

fn parse(input: &str) -> HashSet<Vec3> {
    // Create a hashmap containing all occupied spaces
    input
        .trim()
        .lines()
        .map(|line| line.split(',').map(|num| num.parse().unwrap()).collect::<Vec<isize>>())
        .map(|coords| {
            if let &[x, y, z] = coords.as_slice() {
                return Vec3::new(x, y, z);
            }

            panic!("Invalid!");
        }).collect()
}

fn offset() -> [Vec3; 6] {
    // Offset from cube for each side
    [
        Vec3::new(-1, 0, 0),
        Vec3::new(1, 0, 0),
        Vec3::new(0, -1, 0),
        Vec3::new(0, 1, 0),
        Vec3::new(0, 0, -1),
        Vec3::new(0, 0, 1),
    ]
}

fn calculate_surface(cubes: HashSet<Vec3>) -> usize {
    let offset = offset();

    // Calculate the sum of exposed sides
    cubes
        .iter()
        .map(|pos| {
            // Check for each side of the cube if it is exposed
            offset.iter()
                .filter(|offset| {
                    // Only keep the side if it is exposed
                    !cubes.contains(&(pos + offset))
                })
            .count()
        }).sum()
}

struct Check(Vec<Vec<Vec<(bool, bool)>>>);

impl Check {
    fn new(size: &Vec3) -> Self {
        Check(vec![vec![vec![(false, false); size.z as usize]; size.y as usize]; size.x as usize])
    }
    fn get_mut(&mut self, p: &Vec3) -> &mut (bool, bool) {
        &mut self.0[p.x as usize][p.y as usize][p.z as usize]
    }

    fn get(&self, p: &Vec3) -> &(bool, bool) {
        &self.0[p.x as usize][p.y as usize][p.z as usize]
    }
}


fn fill(check: &mut Check, cubes: &HashSet<Vec3>, size: &Vec3, pos: Vec3) {
    // Mark current space as air and checked
    *check.get_mut(&pos) = (true, true); // (is_checked, is_air)

    let offsets = offset();

    for offset in offsets.iter() {
        let next = &pos + offset;

        // Check if the location is within bounds
        if next.x < 0 || next.x >= size.x || next.y < 0 || next.y >= size.y || next.z < 0 || next.z >= size.z {
            continue;
        }

        // Check if we have already checked the location
        if check.get(&next).0 {
            continue;
        }

        // Check if the space is filled with lava
        if cubes.contains(&next) {
            // Mark the space as checked and not containing air
            *check.get_mut(&next) = (true, false);
            continue;
        }

        // Explore the next location
        fill(check, cubes, size, next);
    }
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = usize;
    type Output2 = usize;

    fn day() -> u8 {
        18
    }

    fn part1(input: &str) -> Self::Output1 {
        let cubes = parse(input);

        calculate_surface(cubes)
    }

    fn part2(input: &str) -> Self::Output2 {
        let mut cubes = parse(input);

        let size = cubes
            .iter()
            .fold(Vec3::new(0, 0, 0), |mut acc, cube| {
                acc.x = acc.x.max(cube.x+1);
                acc.y = acc.y.max(cube.y+1);
                acc.z = acc.z.max(cube.z+1);

                return acc;
            });

        // (is_checked, is_air)
        let mut check = Check::new(&size);

        fill(&mut check, &cubes, &size, Vec3::new(0, 0, 0));

        for x in 0..size.x {
            for y in 0..size.y {
                for z in 0..size.z {
                    if !check.get(&Vec3::new(x, y, z)).1 {
                        cubes.insert(Vec3::new(x, y, z));
                    }
                }
            }
        }


        calculate_surface(cubes)
    }
}
