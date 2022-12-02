// -- Setup & Runners --
use aoc::Solver;
pub struct Day;
fn main() {
    Day::solve();
}
#[test]
fn part1_test1() {
    Day::test(aoc::Part::ONE, "test-1", 24000);
}
#[test]
fn part2_test1() {
    Day::test(aoc::Part::TWO, "test-1", 45000);
}
#[test]
fn part1_solution() {
    Day::test(aoc::Part::ONE, "input", 70116);
}
#[test]
fn part2_solution() {
    Day::test(aoc::Part::TWO, "input", 206582);
}

// -- Solution --
impl aoc::Solver for Day {
    fn day() -> u8 {
        1
    }

    fn part1(input: &str) -> u32 {
        input.split("\n\n")
            .map(|elf| elf.lines()
                 .flat_map(|snack| snack.parse::<u32>())
                 .sum())
            .max()
            .unwrap()
    }

    fn part2(input: &str) -> u32 {
        let mut elfs: Vec<u32> = input.split("\n\n")
            .map(|elf| elf.lines()
                 .flat_map(|snack| snack.parse::<u32>())
                 .sum())
            .collect();

        elfs.sort_by(|a, b| b.cmp(a));

        elfs.iter().take(3).sum()
    }
}
