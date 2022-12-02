// -- Setup & Runners --
use aoc::Solver;
pub struct Day;
fn main() {
    Day::solve();
}
#[test]
fn part1_test1() {
    Day::test(aoc::Part::ONE, "test-1", 15);
}
#[test]
fn part2_test1() {
    Day::test(aoc::Part::TWO, "test-1", 12);
}
#[test]
fn part1_solution() {
    Day::test(aoc::Part::ONE, "input", 14264);
}
#[test]
fn part2_solution() {
    Day::test(aoc::Part::TWO, "input", 12382);
}

// -- Implementation for hand --
#[derive(Debug, Copy, Clone)]
enum Hand {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

impl Hand {
    // Return hand that this hand loses to
    fn loses(&self) -> Hand {
        // Returns the next enum (cyclical) as that one always loses from the current one
        Hand::from((*self as u8 + 1) % 3)
    }

    // Return hand that this hand wins from
    fn wins(&self) -> Hand {
        // Returns the previous enum (cyclical) as that one always wins from the current one
        Hand::from((*self as i8 - 1).rem_euclid(3) as u8)
    }

    fn strategy(&self, input: char) -> Hand {
        match input {
            'X' => self.wins(),
            'Y' => *self,
            'Z' => self.loses(),
            _ => panic!("Unexpected input")
        }
    }

    // Play agains other hand and return our score
    fn play(&self, other: &Hand) -> u32 {
        // 1 = draw, 2 = wins, 0 = loses (x3 to get the score
        ((*self as i8 - *other as i8 + 1).rem_euclid(3) * 3) as u32
    }

    // Get the score value of the current hand
    fn value(&self) -> u32 {
        *self as u32 + 1
    }
}

// -- Conversions --
impl From<u8> for Hand {
    fn from(value: u8) -> Self {
        match value {
            x if x == Hand::Rock as u8 => Hand::Rock,
            x if x == Hand::Paper as u8 => Hand::Paper,
            x if x == Hand::Scissors as u8 => Hand::Scissors,
            x => panic!("Unknown input: {}", x)
        }
    }
}

impl From<char> for Hand {
    fn from(value: char) -> Self {
        match value {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            value => panic!("Unknown input: {value}"),
        }
    }
}

// -- Helper functions --
// Convert the round to a tuple containing the actions of both players
fn round_to_letters(round: &str) -> (char, char) {
    if let &[a, _, b] = round.as_bytes() {
        (a as char, b as char)
    } else {
        panic!("Unexpected input");
    }
}

// -- Solution --
impl aoc::Solver for Day {
    fn day() -> u8 {
        2
    }

    fn part1(input: &str) -> u32 {
        input.split("\n")
            .filter(|round| round.len() > 0)
            .map(round_to_letters)
            .map(|(a, b)| (Hand::from(a), Hand::from(b)))
            .map(|(a, b)| b.play(&a) + b.value())
            .sum()
    }

    fn part2(input: &str) -> u32 {
        input.split("\n")
            .filter(|round| round.len() > 0)
            .map(round_to_letters)
            .map(|(a, b)| (Hand::from(a), b))
            .map(|(a, b)| (a, a.strategy(b)))
            .map(|(a, b)| b.play(&a) + b.value())
            .sum()
    }
}
