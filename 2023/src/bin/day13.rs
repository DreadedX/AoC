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
        Day::test(Day::part1, "test-1", 405)
    }

    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 36448)
    }

    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(Day::part2, "test-1", 400)
    }

    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", 35799)
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

#[derive(Debug, PartialEq, Eq)]
enum Ground {
    Ash,
    Rocks,
}

impl Ground {
    fn flip(&self) -> Self {
        match self {
            Ground::Ash => Ground::Rocks,
            Ground::Rocks => Ground::Ash,
        }
    }
}

impl From<char> for Ground {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Ash,
            '#' => Self::Rocks,
            _ => unreachable!("Invalid input"),
        }
    }
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = usize;
    type Output2 = usize;

    fn day() -> u8 {
        13
    }

    fn part1(input: &str) -> Self::Output1 {
        input
            .split("\n\n")
            .map(|block| {
                block
                    .lines()
                    .map(|line| line.chars().map(|c| c.into()).collect::<Vec<Ground>>())
                    .collect::<Vec<Vec<_>>>()
            })
            .map(|block| {
                let height = block.len();
                let width = block[0].len();

                if let Some(y) = (1..height).find(|&y| {
                    let top = block[0..y].iter().rev();
                    let bottom = block[y..height].iter();

                    top.zip(bottom).all(|(a, b)| a == b)
                }) {
                    100 * y
                } else if let Some(x) = (1..width).find(|&x| {
                    block.iter().all(|line| {
                        let left = line[0..x].iter().rev();
                        let right = line[x..width].iter();

                        left.zip(right).all(|(a, b)| a == b)
                    })
                }) {
                    x
                } else {
                    unreachable!("Their is always a mirror line")
                }
            })
            .sum()
    }

    fn part2(input: &str) -> Self::Output2 {
        input
            .split("\n\n")
            .map(|block| {
                block
                    .lines()
                    .map(|line| line.chars().map(|c| c.into()).collect::<Vec<Ground>>())
                    .collect::<Vec<Vec<_>>>()
            })
            .map(|mut block| {
                let height = block.len();
                let width = block[0].len();

                // Get the row/column of the old mirror line
                let (x_old, y_old) = if let Some(y) = (1..height).find(|&y| {
                    let top = block[0..y].iter().rev();
                    let bottom = block[y..height].iter();

                    top.zip(bottom).all(|(a, b)| a == b)
                }) {
                    (None, Some(y))
                } else if let Some(x) = (1..width).find(|&x| {
                    block.iter().all(|line| {
                        let left = line[0..x].iter().rev();
                        let right = line[x..width].iter();

                        left.zip(right).all(|(a, b)| a == b)
                    })
                }) {
                    (Some(x), None)
                } else {
                    unreachable!("Their is always a mirror line")
                };

                // Try every possible smudge location
                for j in 0..width {
                    for i in 0..height {
                        // Flip one
                        block[i][j] = block[i][j].flip();

                        if let Some(y) = (1..height).find(|&y| {
                            // The mirror line can not be the old one
                            if y_old == Some(y) {
                                return false;
                            }

                            let top = block[0..y].iter().rev();
                            let bottom = block[y..height].iter();

                            top.zip(bottom).all(|(a, b)| a == b)
                        }) {
                            return 100 * y;
                        } else if let Some(x) = (1..width).find(|&x| {
                            // The mirror line can not be the old one
                            if x_old == Some(x) {
                                return false;
                            }

                            block.iter().all(|line| {
                                let left = line[0..x].iter().rev();
                                let right = line[x..width].iter();

                                left.zip(right).all(|(a, b)| a == b)
                            })
                        }) {
                            return x;
                        } else {
                            // Revert flip
                            block[i][j] = block[i][j].flip();
                            continue;
                        }
                    }
                }

                unreachable!("Their should always be a mirror line")
            })
            .sum()
    }
}
