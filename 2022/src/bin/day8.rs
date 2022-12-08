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
        Day::test(aoc::Part::ONE, "test-1", 21)
    }
    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(aoc::Part::ONE, "input", 1845)
    }
    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(aoc::Part::TWO, "test-1", 8)
    }
    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(aoc::Part::TWO, "input", 230112)
    }

    // Benchmarks
    extern crate test;
    #[bench]
    #[ignore]
    fn part1_bench(b: &mut test::Bencher) {
        Day::benchmark(aoc::Part::ONE, b)
    }
    #[bench]
    #[ignore]
    fn part2_bench(b: &mut test::Bencher) {
        Day::benchmark(aoc::Part::TWO, b)
    }
}
// -- Helper --
fn parse(input: &str) -> (usize, Vec<u32>) {
    let size = input.lines().count();
    let input = input
        .lines()
        .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    (size, input)
}

fn find_highest(size: usize, highest: &mut u32, row: usize, column: usize, tree: u32) -> bool {
    match (row, column, tree) {
        (r, _, _) if r == 0 => true,
        (r, _, _) if r == size-1 => true,
        (_, c, tree) if c == 0 => {
            *highest = tree;
            true
        },
        (_, c, _) if c == size-1 => {
            true
        },
        (_ ,_, tree) => {
            if tree > *highest {
                *highest = tree;
                true
            } else {
                false
            }
        }
    }
}

// @TODO Figure out if we can do this faster
fn transponse<T: Copy>(size: usize, input: &Vec<T>) -> Vec<T> {
    let mut output = Vec::new();
    output.reserve(input.len());

    for c in 0..size {
        for r in 0..size {
            output.push(input[r*size + c]);
        }
    }

    output
}

fn process_row_reverse((row, line): (usize, &[u32])) -> Vec<bool> {
    let size = line.len();
    let mut line = line
        .iter()
        .rev()
        .enumerate()
        .scan(0, |highest, (column, tree)| { Some(find_highest(size, highest, row, column, *tree)) })
        .collect::<Vec<_>>();

    line.reverse();
    line
}

fn process_row((row, line): (usize, &[u32])) -> Vec<bool> {
    let size = line.len();
    line
        .iter()
        .enumerate()
        .scan(0, |highest, (column, tree)| { Some(find_highest(size, highest, row, column, *tree)) })
        .collect::<Vec<_>>()
}

fn generate_highest(size: usize, input: &Vec<u32>) -> Vec<bool> {
    let from_left = input
        .chunks(size)
        .enumerate()
        .flat_map(process_row);

    let from_right = input
        .chunks(size)
        .enumerate()
        .flat_map(process_row_reverse);

    let input = transponse(size, &input);
    let from_top = input
        .chunks(size)
        .enumerate()
        .flat_map(process_row);

    let from_bottom = input
        .chunks(size)
        .enumerate()
        .flat_map(process_row_reverse);

    let horizontal = from_top.zip(from_bottom).map(|(top, bottom)| top || bottom).collect::<Vec<_>>();
    let horizontal = transponse(size, &horizontal);

    from_left
        .zip(from_right)
        .zip(horizontal.iter())
        .map(|((left, right), horizontal)| left || right || *horizontal)
        .collect::<Vec<_>>()
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output = usize;
    fn day() -> u8 {
        8
    }

    fn part1(input: &str) -> Self::Output {
        let (size, input) = parse(input);
        generate_highest(size, &input)
            .iter()
            .fold(0, |acc, value| {
                if *value {
                    acc+1
                } else {
                    acc
                }
            })
    }

    fn part2(input: &str) -> Self::Output {
        let (size, input) = parse(input);
        let map = generate_highest(size, &input);

        map
            .chunks(size)
            .enumerate()
            .flat_map(|(row, line)| line.iter().map(|value| (row, value)).enumerate().collect::<Vec<_>>())
            .filter_map(|(column, (row, value))| if *value {
                Some((row, column))
            } else {
                None
            }).map(|(row, column)| {
                if row == 0 || row == size-1 || column == 0 || column == size-1 {
                    // Value is going to be set to 0
                    return 0;
                }
                let height = input[row*size + column];

                let mut distance_left = 0;
                {
                    for (idx, c) in (0..column).rev().enumerate() {
                        if input[row*size + c] >= height {
                            distance_left = idx + 1;
                            break;
                        }
                    }
                    if distance_left == 0 {
                        distance_left = column;
                    }
                }

                let mut distance_right = 0;
                {
                    for (idx, c) in (column+1..size).enumerate() {
                        if input[row*size + c] >= height {
                            distance_right = idx+1;
                            break;
                        }
                    }
                    if distance_right == 0 {
                        distance_right = size-column-1;
                    }
                }

                let mut distance_up = 0;
                {
                    for (idx, r) in (0..row).rev().enumerate() {
                        if input[r*size + column] >= height {
                            distance_up = idx + 1;
                            break;
                        }
                    }
                    if distance_up == 0 {
                        distance_up = row;
                    }
                }


                let mut distance_down = 0;
                {
                    for (idx, r) in (row+1..size).enumerate() {
                        if input[r*size + column] >= height {
                            distance_down = idx+1;
                            break;
                        }
                    }
                    if distance_down == 0 {
                        distance_down = size-row-1;
                    }
                }

                distance_up * distance_left * distance_down * distance_right
            }).max().unwrap()
    }
}
