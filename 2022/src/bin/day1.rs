use aoc::Solver;

pub struct Day {}
fn main() {
    Day::solve();
}
#[test]
fn part1() {
    Day::test(aoc::Part::ONE);
}
#[test]
fn part2() {
    Day::test(aoc::Part::TWO);
}


impl aoc::Solver for Day {
    fn day() -> u8 {
        1
    }
    fn part1(input: &str) -> u32 {
        input.split("\n\n")
            .map(|elf| elf.split("\n")
                 .flat_map(|snack| snack.parse::<u32>())
                 .sum())
            .max()
            .unwrap()
    }

    fn part2(input: &str) -> u32 {
        let mut elfs: Vec<u32> = input.split("\n\n")
            .map(|elf| elf.split("\n")
                 .flat_map(|snack| snack.parse::<u32>())
                 .sum())
            .collect();

        elfs.sort_by(|a, b| b.cmp(a));

        elfs.iter().take(3).sum()
    }
}
