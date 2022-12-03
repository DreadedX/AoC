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
        Day::test(aoc::Part::ONE, "test-1", 0)
    }
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    fn day() -> u8 {
        todo!("Day not set")
    }

    fn part1(input: &str) -> u32 {
        0
    }

    fn part2(input: &str) -> u32 {
        0
    }
}
