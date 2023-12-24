#![feature(test)]
use std::collections::HashSet;

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
        Day::test(Day::part1, "test-1", 94)
    }

    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 2130)
    }

    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(Day::part2, "test-1", 154)
    }

    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", 6710)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Path,
    Forest,
    Up,
    Down,
    Left,
    Right,
}

fn find_path(
    position: (isize, isize),
    end: (isize, isize),
    map: &Vec<Vec<Tile>>,
    visited: &mut Vec<Vec<bool>>,
    slippery: bool,
) -> (usize, (isize, isize)) {
    visited[position.1 as usize][position.0 as usize] = true;
    let size = (map[0].len() as isize, map.len() as isize);

    let mut final_position = position;

    let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    let mut steps = 0;
    for direction in directions {
        let next_position = (position.0 + direction.0, position.1 + direction.1);

        if next_position.0 < 0
            || next_position.1 < 0
            || next_position.0 >= size.0
            || next_position.1 >= size.1
            || visited[next_position.1 as usize][next_position.0 as usize]
        {
            continue;
        }

        let tile = map[next_position.1 as usize][next_position.0 as usize];
        if tile == Tile::Forest {
            continue;
        }

        if slippery
            && (tile == Tile::Up && direction != (0, -1)
                || tile == Tile::Down && direction != (0, 1)
                || tile == Tile::Left && direction != (-1, 0)
                || tile == Tile::Right && direction != (1, 0))
        {
            continue;
        }

        let (mut a, b) = find_path(next_position, end, map, visited, slippery);
        a += 1;

        if a > steps && b == end {
            steps = a;
            final_position = b;
        }
    }

    visited[position.1 as usize][position.0 as usize] = false;

    (steps, final_position)
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = usize;
    type Output2 = usize;

    fn day() -> u8 {
        23
    }

    fn part1(input: &str) -> Self::Output1 {
        let map: Vec<Vec<_>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Tile::Path,
                        '#' => Tile::Forest,
                        '^' => Tile::Up,
                        'v' => Tile::Down,
                        '<' => Tile::Left,
                        '>' => Tile::Right,
                        _ => unreachable!("Invalid input"),
                    })
                    .collect()
            })
            .collect();

        let size = (map[0].len() as isize, map.len() as isize);
        let mut visited = vec![vec![false; size.0 as usize]; size.1 as usize];
        find_path((1, 0), (size.0 - 2, size.1 - 1), &map, &mut visited, true).0
    }

    fn part2(input: &str) -> Self::Output2 {
        let map: Vec<Vec<_>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Tile::Path,
                        '#' => Tile::Forest,
                        '^' => Tile::Up,
                        'v' => Tile::Down,
                        '<' => Tile::Left,
                        '>' => Tile::Right,
                        _ => unreachable!("Invalid input"),
                    })
                    .collect()
            })
            .collect();

        let size = (map[0].len() as isize, map.len() as isize);
        let mut visited = vec![vec![false; size.0 as usize]; size.1 as usize];
        find_path((1, 0), (size.0 - 2, size.1 - 1), &map, &mut visited, false).0
    }
}
