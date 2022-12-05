use anyhow::Result;
use aoc::{Solver, Output};

// -- Runners --
fn main() -> Result<()> {
    Day::solve()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() -> Result<()> {
        Day::test(aoc::Part::ONE, "test-1", Output::Number(24000))
    }
    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(aoc::Part::TWO, "test-1", Output::Number(45000))
    }
    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(aoc::Part::ONE, "input", Output::Number(70116))
    }
    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(aoc::Part::TWO, "input", Output::Number(206582))
    }
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    fn day() -> u8 {
        1
    }

    fn part1(input: &str) -> Output {
        let result = input.split("\n\n")
            .map(|elf| elf.lines()
                 .flat_map(|snack| snack.parse::<u32>())
                 .sum())
            .max()
            .unwrap();

        Output::Number(result)
    }

    fn part2(input: &str) -> Output {
        let mut elfs: Vec<u32> = input.split("\n\n")
            .map(|elf| elf.lines()
                 .flat_map(|snack| snack.parse::<u32>())
                 .sum())
            .collect();

        elfs.sort_by(|a, b| b.cmp(a));

        let result = elfs.iter().take(3).sum();

        Output::Number(result)
    }
}
