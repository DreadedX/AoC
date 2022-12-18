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
        let cubes = parse(input);

        let size = cubes
            .iter()
            .fold(Vec3::new(0, 0, 0), |mut acc, cube| {
                acc.x = acc.x.max(cube.x+1);
                acc.y = acc.y.max(cube.y+1);
                acc.z = acc.z.max(cube.z+1);

                return acc;
            });

        let mut check = vec![vec![vec![[false; 6]; size.z as usize]; size.y as usize]; size.x as usize];

        // Z+
        for x in 0..size.x {
            for y in 0..size.y {
                if let Some(end) = (0..size.z)
                    .position(|z| {
                        cubes.contains(&Vec3::new(x, y, z))
                    }) {
                        for z in end..size.z as usize {
                            check[x as usize][y as usize][z][0] = true;
                        }
                    }
            }
        }

        // Z-
        for x in 0..size.x {
            for y in 0..size.y {
                if let Some(end) = (0..size.z)
                    .rev()
                    .position(|z| {
                        cubes.contains(&Vec3::new(x, y, z))
                    }) {
                        for z in 0..size.z as usize - end {
                            check[x as usize][y as usize][z][1] = true;
                        }
                    }
            }
        }

        // Y+
        for z in 0..size.z {
            for x in 0..size.x {
                if let Some(end) = (0..size.y)
                    .position(|y| {
                        cubes.contains(&Vec3::new(x, y, z))
                    }) {
                        for y in end..size.y as usize {
                            check[x as usize][y][z as usize][2] = true;
                        }
                    }
            }
        }

        // Y-
        for z in 0..size.z {
            for x in 0..size.x {
                if let Some(end) = (0..size.y)
                    .rev()
                    .position(|y| {
                        cubes.contains(&Vec3::new(x, y, z))
                    }) {
                        for y in 0..size.y as usize - end {
                            check[x as usize][y][z as usize][3] = true;
                        }
                    }
            }
        }

        // X+
        for y in 0..size.y {
            for z in 0..size.z {
                if let Some(end) = (0..size.x)
                    .position(|x| {
                        cubes.contains(&Vec3::new(x, y, z))
                    }) {
                        for x in end..size.x as usize {
                            check[x][y as usize][z as usize][4] = true;
                        }
                    }
            }
        }

        // X-
        for y in 0..size.y {
            for z in 0..size.z {
                if let Some(end) = (0..size.x)
                    .rev()
                    .position(|x| {
                        cubes.contains(&Vec3::new(x, y, z))
                    }) {
                        for x in 0..size.x as usize - end {
                            check[x][y as usize][z as usize][5] = true;
                        }
                    }
            }
        }

        let mut cubes = HashSet::new();

        for x in 0..size.x {
            for y in 0..size.y {
                for z in 0..size.z {
                    if check[x as usize][y as usize][z as usize].iter().all(|v| *v) {
                        cubes.insert(Vec3::new(x, y, z));
                    }
                }
            }
        }

        calculate_surface(cubes)
    }
}
