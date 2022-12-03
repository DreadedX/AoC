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
        Day::test(aoc::Part::ONE, "test-1", 15)
    }
    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(aoc::Part::TWO, "test-1", 12)
    }
    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(aoc::Part::ONE, "input", 14264)
    }
    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(aoc::Part::TWO, "input", 12382)
    }
}

// -- Implementation for hand --
#[derive(Debug, Copy, Clone)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    // Return hand that this hand loses to
    fn loses_to(&self) -> Hand {
        // Returns the next enum (cyclical) as that one always loses from the current one
        Hand::from((*self as u8 + 1) % 3)
    }

    // Return hand that this hand wins from
    fn wins_from(&self) -> Hand {
        // Returns the previous enum (cyclical) as that one always wins from the current one
        Hand::from((*self as i8 - 1).rem_euclid(3) as u8)
    }

    fn strategy(&self, input: &str) -> Hand {
        match input {
            "X" => self.wins_from(),
            "Y" => *self,
            "Z" => self.loses_to(),
            _ => panic!("Unexpected input")
        }
    }

    // Play agains other hand and return our score
    fn play(&self, other: &Hand) -> u32 {
        // 1 = draw, 2 = wins, 0 = loses (x3 to get the score
        ((*self as i8 - *other as i8 + 1).rem_euclid(3) * 3) as u32
    }
}

// -- Conversions --
impl From<u8> for Hand {
    fn from(value: u8) -> Self {
        match value {
            x if x == Self::Rock as u8 => Self::Rock,
            x if x == Self::Paper as u8 => Self::Paper,
            x if x == Self::Scissors as u8 => Self::Scissors,
            x => panic!("Unknown input: {}", x)
        }
    }
}

impl From<&str> for Hand {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("Invalid input: {s}"),
        }
    }
}

impl From<&Hand> for u32 {
    fn from(value: &Hand) -> Self {
        *value as u32 + 1
    }
}

// -- Helper functions --
// Convert the round to a tuple containing the actions of both players
fn convert(round: &str) -> (&str, &str) {
    match round.split_once(" ") {
        Some((a, b)) => (a, b),
        None => panic!("Invalid input: {round}"),
    }
}

fn calc_score(sum: u32, (a, b): (Hand, Hand)) -> u32 {
    sum + b.play(&a) + u32::from(&b)
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    fn day() -> u8 {
        2
    }

    fn part1(input: &str) -> u32 {
        input.lines()
            .filter(|round| round.len() > 0)
            .map(convert)
            .map(|(a, b)| (Hand::from(a), Hand::from(b)))
            .fold(0, calc_score)
    }

    fn part2(input: &str) -> u32 {
        input.lines()
            .filter(|round| round.len() > 0)
            .map(convert)
            .map(|(a, b)| {
                let opponent = Hand::from(a);
                (opponent, opponent.strategy(b))
            })
            .fold(0, calc_score)
    }
}
