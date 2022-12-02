// -- Setup & Runners --
use anyhow::Result;
use aoc::Solver;
pub struct Day;
fn main() -> Result<()> {
    Day::solve()
}
#[test]
fn part1_test1() -> Result<()> {
    Day::test(aoc::Part::ONE, "test-1", 0)
}

// -- Solution --
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
