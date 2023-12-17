#![feature(test)]
use std::collections::{HashMap, HashSet};

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
        Day::test(Day::part1, "test-1", 46)
    }

    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 8021)
    }

    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(Day::part2, "test-1", 51)
    }

    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", 8216)
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

fn energize(
    mut position: (isize, isize),
    mut direction: (isize, isize),
    size: (isize, isize),
    map: &HashMap<(isize, isize), char>,
    energized: &mut HashSet<((isize, isize), (isize, isize))>,
) {
    while position.0 >= 0 && position.0 < size.0 && position.1 >= 0 && position.1 < size.1 {
        if !energized.insert((position, direction)) {
            // We already proccessed this path before
            break;
        }

        if let Some(c) = map.get(&position) {
            direction = match (c, direction) {
                ('\\', (1, 0)) => (0, 1),
                ('\\', (-1, 0)) => (0, -1),
                ('\\', (0, 1)) => (1, 0),
                ('\\', (0, -1)) => (-1, 0),
                ('/', (1, 0)) => (0, -1),
                ('/', (-1, 0)) => (0, 1),
                ('/', (0, 1)) => (-1, 0),
                ('/', (0, -1)) => (1, 0),
                ('-', (1 | -1, 0)) => direction,
                ('-', (0, 1 | -1)) => {
                    energize((position.0 - 1, position.1), (-1, 0), size, map, energized);
                    energize((position.0 + 1, position.1), (1, 0), size, map, energized);
                    break;
                }
                ('|', (1 | -1, 0)) => {
                    energize((position.0, position.1 - 1), (0, -1), size, map, energized);
                    energize((position.0, position.1 + 1), (0, 1), size, map, energized);
                    break;
                }
                ('|', (0, 1 | -1)) => direction,
                _ => unreachable!("Invalid state"),
            };
        }

        position.0 += direction.0;
        position.1 += direction.1;
    }
}

fn energized(
    start: (isize, isize),
    direction: (isize, isize),
    size: (isize, isize),
    map: &HashMap<(isize, isize), char>,
) -> usize {
    let mut energized = HashSet::new();
    energize(start, direction, size, &map, &mut energized);

    energized
        .iter()
        .map(|(position, _)| position)
        .collect::<HashSet<_>>()
        .len()
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = usize;
    type Output2 = usize;

    fn day() -> u8 {
        16
    }

    fn part1(input: &str) -> Self::Output1 {
        let mut width = 0;
        let mut height = 0;

        let map = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                let y = y as isize;
                if y + 1 > height {
                    height = y + 1;
                }

                line.chars()
                    .enumerate()
                    .filter_map(|(x, c)| {
                        let x = x as isize;
                        if x + 1 > width {
                            width = x + 1;
                        }
                        match c {
                            '.' => None,
                            '\\' | '/' | '|' | '-' => Some(((x, y), c)),
                            _ => unreachable!(),
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<HashMap<_, _>>();

        energized((0, 0), (1, 0), (width, height), &map)
    }

    fn part2(input: &str) -> Self::Output2 {
        let mut width = 0;
        let mut height = 0;

        let map = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                let y = y as isize;
                if y + 1 > height {
                    height = y + 1;
                }

                line.chars()
                    .enumerate()
                    .filter_map(|(x, c)| {
                        let x = x as isize;
                        if x + 1 > width {
                            width = x + 1;
                        }
                        match c {
                            '.' => None,
                            '\\' | '/' | '|' | '-' => Some(((x, y), c)),
                            _ => unreachable!(),
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<HashMap<_, _>>();

        let mut max = 0;
        for x in 0..width {
            let a = energized((x, 0), (0, 1), (width, height), &map);
            let b = energized((x, height - 1), (0, -1), (width, height), &map);

            max = std::cmp::max(max, std::cmp::max(a, b));
        }

        for y in 0..height {
            let a = energized((0, y), (1, 0), (width, height), &map);
            let b = energized((width - 1, y), (-1, 0), (width, height), &map);

            max = std::cmp::max(max, std::cmp::max(a, b));
        }

        max
    }
}
