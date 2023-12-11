#![feature(test)]
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
        Day::test(Day::part1, "test-1", 374)
    }

    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 10490062)
    }

    // NOTE: The given example for part2 are only for 10x and 100x, while part2 uses 1000000x
    // Therefore there are now part2 example test

    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", 382979724122)
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
struct GridPoint {
    galaxy: bool,
    vertical_expansion: bool,
    horizontal_expansion: bool,
}

fn distance(
    a: (usize, usize),
    b: (usize, usize),
    grid: &[Vec<GridPoint>],
    expansion: usize,
) -> usize {
    let (range_x, x_end) = if a.0 > b.0 {
        (b.0..a.0, a.0)
    } else {
        (a.0..b.0, b.0)
    };
    let (range_y, y_start) = if a.1 > b.1 {
        (b.1..a.1, b.1)
    } else {
        (a.1..b.1, a.1)
    };

    let mut steps = 0;
    for x in range_x {
        if grid[y_start][x].horizontal_expansion {
            steps += expansion;
        } else {
            steps += 1;
        }
    }
    for y in range_y {
        if grid[y][x_end].vertical_expansion {
            steps += expansion;
        } else {
            steps += 1;
        }
    }

    steps
}

fn solve(input: &str, expansion: usize) -> usize {
    let mut galaxies = Vec::<(usize, usize)>::new();
    let mut grid: Vec<Vec<_>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let has_galaxy = c == '#';

                    if has_galaxy {
                        galaxies.push((x, y));
                    }

                    GridPoint {
                        galaxy: has_galaxy,
                        vertical_expansion: false,
                        horizontal_expansion: false,
                    }
                })
                .collect()
        })
        .collect();

    // The grid is square
    let size = grid.len();

    for row in &mut grid {
        let has_galaxy = row.iter().any(|point| point.galaxy);

        row.iter_mut()
            .for_each(|point| point.vertical_expansion = !has_galaxy)
    }

    for x in 0..size {
        let mut has_galaxy = false;
        for row in &grid {
            has_galaxy |= row[x].galaxy;
        }

        for row in &mut grid {
            row[x].horizontal_expansion = !has_galaxy;
        }
    }

    let mut sum = 0;
    for a in &galaxies {
        for b in &galaxies {
            if a == b {
                continue;
            }

            sum += distance(*a, *b, &grid, expansion);
        }
    }

    // We are computing every pair twice, dividing by two is a simpler solution then keep keeping
    // track of every pair, but it does require more computation
    sum / 2
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = usize;
    type Output2 = usize;

    fn day() -> u8 {
        11
    }

    fn part1(input: &str) -> Self::Output1 {
        solve(input, 2)
    }

    fn part2(input: &str) -> Self::Output2 {
        solve(input, 1000000)
    }
}
