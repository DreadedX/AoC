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
        Day::test(aoc::Part::ONE, "test-1", 157)
    }
    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(aoc::Part::TWO, "test-1", 70)
    }
    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(aoc::Part::ONE, "input", 8298)
    }
    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(aoc::Part::TWO, "input", 2708)
    }
}

// -- Helpers --
fn convert(c: &u8) -> u32 {
    let result = match c {
        b'a'..=b'z' => c - b'a' + 1,
        b'A'..=b'Z' => c - b'A' + 27,
        _ => panic!("Invalid input"),
    };
    result as u32
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    fn day() -> u8 {
        3
    }

    fn part1(input: &str) -> u32 {
        input.lines()
            .map(|line| line.split_at(line.len()/2))
            .map(|(a, b)| {
                // @NOTE This is not really ok if the string contains multi byte characters, this
                // is however not the case here
                for c in a.as_bytes() {
                    // There is always one character in common between the two sides
                    if b.contains(*c as char) {
                        return c;
                    }
                }
                unreachable!("No characters in common, this should never happen")
            })
            .map(convert)
            .sum()
    }

    fn part2(input: &str) -> u32 {
        input.lines()
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|group| {
                if let [a, b, c] = group {
                    (a, b, c)
                } else {
                    panic!("Invalid input")
                }
            })
            .map(|(a, b, c)| {
                // @NOTE This is not really ok if the string contains multi byte characters, this
                // is however not the case here
                for l in a.as_bytes() {
                    // There is always one character in common between the three rucksacks
                    if b.contains(*l as char) && c.contains(*l as char) {
                        return l;
                    }
                }
                unreachable!("No characters in common, this should never happen")
            })
            .map(convert)
            .sum()
    }
}
