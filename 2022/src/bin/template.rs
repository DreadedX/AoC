use aoc::Solver;

pub struct Day {}
fn main() {
    Day::solve();
}
#[test]
fn part1() {
    Day::test(aoc::Part::ONE, "test-1", 0);
}


impl aoc::Solver for Day {
    fn day() -> u8 {
        0
    }
    fn part1(input: &str) -> u32 {
        input.len() as u32
    }

    fn part2(input: &str) -> u32 {
        input.len() as u32
    }
}
