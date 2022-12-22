#![feature(test)]
use core::fmt;
use std::str::FromStr;

use anyhow::Result;
use aoc::Solver;
use regex::Regex;

// -- Runners --
fn main() -> Result<()> {
    Day::solve()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() -> Result<()> {
        Day::test(Day::part1, "test-1", 6032)
    }
    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 93226)
    }
    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(Day::part2, "test-1", 5031)
    }

    // Benchmarks
    extern crate test;
    #[bench]
    #[ignore]
    fn part1_bench(b: &mut test::Bencher) {
        Day::benchmark(Day::part1, b)
    }
    #[bench]
    #[ignore]
    fn part2_bench(b: &mut test::Bencher) {
        Day::benchmark(Day::part2, b)
    }
}

#[derive(PartialEq, Eq)]
enum Tile {
    Void,
    Open,
    Wall,
}

#[derive(Debug)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn rotate(&mut self, clockwise: bool) {
        *self = match self {
            Direction::Right => if clockwise { Direction::Down }  else { Direction::Up },
            Direction::Down => if clockwise { Direction::Left }  else { Direction::Right },
            Direction::Left => if clockwise { Direction::Up }  else { Direction::Down },
            Direction::Up => if clockwise { Direction::Right }  else { Direction::Left },
        }
    }
}

impl From<&Direction> for usize {
    fn from(value: &Direction) -> Self {
        match value {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            ' ' => Tile::Void,
            '.' => Tile::Open,
            '#' => Tile::Wall,
            _ => panic!("Invalid tile input: {value}"),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Tile::Void => '~',
            Tile::Open => '.',
            Tile::Wall => '#',
        })
    }
}

#[derive(Debug, Copy, Clone)]
struct Vec2 {
    x: usize,
    y: usize,
}

struct Map {
    map: Vec<Vec<Tile>>,
    pos: Vec2,
    direction: Direction,
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .lines()
            .take_while(|line| !line.is_empty())
            .map(|line| {
                line
                    .chars()
                    .map(Tile::from)
                    .collect::<Vec<_>>()
            })
        .collect::<Vec<_>>();

        // Get the starting point
        let x = map[0].iter().position(|t| *t == Tile::Open).unwrap();
        let pos = Vec2 {
            x,
            y: 0,
        };

        Ok(Self { map, pos, direction: Direction::Right })
    }
}

impl Map {
    fn movement(&mut self, steps: usize) {
        for _ in 0..steps {
            let mut np = self.pos;
            match self.direction {
                Direction::Right => {
                    np.x += 1;
                    // Wrap around if we walk out of the map on the right
                    if np.x >= self.map[np.y].len() {
                        np.x = 0;

                        // Move over the void, we only do this when we wrap around, since there are
                        // no void tiles on the right
                        while self.map[np.y][np.x] == Tile::Void {
                            np.x += 1;
                        }
                    }
                },
                Direction::Left => {
                    // Make sure we do not underflow
                    if np.x == 0 {
                        np.x = self.map[np.y].len();
                    }

                    // Update our location
                    np.x -= 1;

                    // Jump over the void
                    if self.map[np.y][np.x] == Tile::Void {
                        np.x = self.map[np.y].len()-1;
                    }
                },
                Direction::Up => {
                    // Make sure we do not underflow
                    if np.y == 0 {
                        np.y = self.map.len();
                    }

                    // Update our location
                    np.y -= 1;

                    // Jump over the void
                    if np.x >= self.map[np.y].len() || self.map[np.y][np.x] == Tile::Void {
                        np.y = self.map.len()-1;
                    }

                    while np.x >= self.map[np.y].len() || self.map[np.y][np.x] == Tile::Void {
                        np.y -= 1;
                    }
                },
                Direction::Down => {
                    // Update our location
                    np.y += 1;

                    // Wrap around
                    if np.y >= self.map.len() {
                        np.y = 0;
                    }

                    // Jump over the void
                    if np.x >= self.map[np.y].len() || self.map[np.y][np.x] == Tile::Void {
                        np.y = 0;
                    }

                    while np.x >= self.map[np.y].len() || self.map[np.y][np.x] == Tile::Void {
                        np.y += 1;
                    }
                }
            }

            // If the space is open we update our location, otherwise we stay where we are
            if self.map[np.y][np.x] == Tile::Open {
                self.pos = np;
            } else {
                // There is a wall in front of us, so no point in trying the rest of the
                // steps
                break;
            }
        }
    }

    fn rotate(&mut self, clockwise: bool) {
        self.direction.rotate(clockwise);
    }

    fn score(&self) -> usize {
        1000 * (self.pos.y+1) + 4 * (self.pos.x+1) + (usize::from(&self.direction))
    }
}

fn parse_movement(input: &str) -> Vec<(usize, Option<bool>)> {
    let re = Regex::new(r"(?P<steps>[0-9]+)(?P<direction>L|R)?").unwrap();

    re.captures_iter(input).map(|capture| {
        let steps: usize = capture.name("steps").unwrap().as_str().parse().unwrap();

        let clockwise = capture.name("direction").map(|direction| {
            match direction.as_str() {
                "L" => false,
                "R" => true,
                _ => panic!("Invalid rotation: {}", direction.as_str()),
            }
        });

        (steps, clockwise)
    }).collect()
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = usize;
    type Output2 = usize;

    fn day() -> u8 {
        22
    }

    fn part1(input: &str) -> Self::Output1 {
        let (map, movement) = input.split_once("\n\n").unwrap();
        let mut map = Map::from_str(map).unwrap();
        let movement = parse_movement(movement);

        for mov in movement {
            map.movement(mov.0);
            if let Some(clockwise) = mov.1 {
                map.rotate(clockwise);
            }
        }

        map.score()
    }

    fn part2(input: &str) -> Self::Output2 {
        0
    }
}
