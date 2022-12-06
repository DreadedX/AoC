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
        Day::test(aoc::Part::ONE, "test-1", TEST)
    }
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output = TYPE;
    fn day() -> u8 {
        DAY
    }

    fn part1(input: &str) -> Self::Output {
        DEFAULT
    }

    fn part2(input: &str) -> Self::Output {
        DEFAULT
    }
}
