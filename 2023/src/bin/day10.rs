#![feature(test)]
use core::panic;
use std::collections::{HashMap, VecDeque};

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
        Day::test(Day::part1, "test-1", 4)
    }

    #[test]
    fn part1_test2() -> Result<()> {
        Day::test(Day::part1, "test-2", 8)
    }

    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 6768)
    }

    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(Day::part2, "test-1", 1)
    }

    #[test]
    fn part2_test2() -> Result<()> {
        Day::test(Day::part2, "test-2", 1)
    }

    #[test]
    fn part2_test3() -> Result<()> {
        Day::test(Day::part2, "test-3", 4)
    }

    #[test]
    fn part2_test4() -> Result<()> {
        Day::test(Day::part2, "test-4", 4)
    }

    #[test]
    fn part2_test5() -> Result<()> {
        Day::test(Day::part2, "test-5", 8)
    }

    #[test]
    fn part2_test6() -> Result<()> {
        Day::test(Day::part2, "test-6", 10)
    }

    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", 351)
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Maze {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Start,
}

impl Maze {
    fn from(c: char) -> Option<Self> {
        match c {
            '|' => Some(Self::Vertical),
            '-' => Some(Self::Horizontal),
            'L' => Some(Self::NorthEast),
            'J' => Some(Self::NorthWest),
            '7' => Some(Self::SouthWest),
            'F' => Some(Self::SouthEast),
            'S' => Some(Self::Start),
            _ => None,
        }
    }

    fn connects(&self, other: &Self, direction: (isize, isize)) -> bool {
        match direction {
            (0, -1) => match self {
                Self::Vertical | Self::NorthEast | Self::NorthWest | Self::Start => {
                    *other == Self::Vertical
                        || *other == Self::SouthWest
                        || *other == Self::SouthEast
                        || *other == Self::Start
                }
                _ => false,
            },
            (1, 0) => match self {
                Self::Horizontal | Self::NorthEast | Self::SouthEast | Self::Start => {
                    *other == Self::Horizontal
                        || *other == Self::NorthWest
                        || *other == Self::SouthWest
                        || *other == Self::Start
                }
                _ => false,
            },
            (0, 1) => match self {
                Self::Vertical | Self::SouthWest | Self::SouthEast | Self::Start => {
                    *other == Self::Vertical
                        || *other == Self::NorthEast
                        || *other == Self::NorthWest
                        || *other == Self::Start
                }
                _ => false,
            },
            (-1, 0) => match self {
                Self::Horizontal | Self::NorthWest | Self::SouthWest | Self::Start => {
                    *other == Self::Horizontal
                        || *other == Self::NorthEast
                        || *other == Self::SouthEast
                        || *other == Self::Start
                }
                _ => false,
            },
            _ => unreachable!("Invalid direction"),
        }
    }

    fn sides(&self, direction: (isize, isize)) -> (usize, usize) {
        match direction {
            (0, -1) => match self {
                Maze::Vertical => (1, 1),
                Maze::NorthEast => (2, 0),
                Maze::NorthWest => (0, 2),
                Maze::Start => (0, 0),
                _ => unreachable!("Invalid movement"),
            },
            (1, 0) => match self {
                Maze::Horizontal => (1, 1),
                Maze::NorthEast => (0, 2),
                Maze::SouthEast => (2, 0),
                Maze::Start => (0, 0),
                _ => unreachable!("Invalid movement"),
            },
            (0, 1) => match self {
                Maze::Vertical => (1, 1),
                Maze::SouthWest => (2, 0),
                Maze::SouthEast => (0, 2),
                Maze::Start => (0, 0),
                _ => unreachable!("Invalid movement"),
            },
            (-1, 0) => match self {
                Maze::Horizontal => (1, 1),
                Maze::NorthWest => (2, 0),
                Maze::SouthWest => (0, 2),
                Maze::Start => (0, 0),
                _ => unreachable!("Invalid movement"),
            },
            _ => unreachable!("Invalid direction"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Loop {
    North,
    East,
    South,
    West,
    Inner,
}

impl Loop {
    fn from(direction: (isize, isize)) -> Self {
        match direction {
            (0, -1) => Self::North,
            (1, 0) => Self::East,
            (0, 1) => Self::South,
            (-1, 0) => Self::West,
            _ => unreachable!("Invalid direction"),
        }
    }

    fn offset(&self) -> (isize, isize) {
        match self {
            Loop::North => (0, -1),
            Loop::East => (1, 0),
            Loop::South => (0, 1),
            Loop::West => (-1, 0),
            Loop::Inner => (0, 0),
        }
    }
}

fn flood_fill(position: (isize, isize), map: &mut HashMap<(isize, isize), Loop>) {
    let mut queue = VecDeque::new();
    queue.push_back(position);

    while let Some(position) = queue.pop_front() {
        if map.get(&position).is_none() {
            map.insert(position, Loop::Inner);

            for direction in DIRECTIONS {
                queue.push_back((position.0 + direction.0, position.1 + direction.1));
            }
        }
    }
}

static DIRECTIONS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = usize;
    type Output2 = usize;

    fn day() -> u8 {
        10
    }

    fn part1(input: &str) -> Self::Output1 {
        let mut position_start = (-1, -1);

        let maze: HashMap<(isize, isize), Maze> = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(x, c)| Maze::from(c).map(|maze| ((x as isize, y as isize), maze)))
                    .inspect(|(position, maze)| {
                        if *maze == Maze::Start {
                            position_start = *position
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        if position_start == (-1, -1) {
            panic!("No valid start in input");
        }

        let mut position_current = position_start;
        let mut position_previous = position_start;
        let mut length = 0;
        'outer: loop {
            let tile_current = maze.get(&position_current).unwrap();

            for direction in DIRECTIONS {
                let position_next = (
                    position_current.0 + direction.0,
                    position_current.1 + direction.1,
                );

                // We are now allowed to go back to the previous position
                if position_next == position_previous {
                    continue;
                }

                if let Some(tile_next) = maze.get(&position_next) {
                    if tile_current.connects(tile_next, direction) {
                        position_previous = position_current;
                        position_current = position_next;
                        length += 1;

                        if position_current == position_start {
                            return length / 2;
                        }

                        continue 'outer;
                    }
                }
            }

            unreachable!("Unable to move forward...");
        }
    }

    // Alternate solution that works for the examples and is off-by-one for the actual input
    fn part2(input: &str) -> Self::Output2 {
        let mut position_start = (-1, -1);
        let end_direction;
        let mut maze: HashMap<(isize, isize), Maze> = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(x, c)| Maze::from(c).map(|maze| ((x as isize, y as isize), maze)))
                    .inspect(|(position, maze)| {
                        if *maze == Maze::Start {
                            position_start = *position
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        let mut map = HashMap::new();

        if position_start == (-1, -1) {
            panic!("No valid start in input");
        }

        let mut sides = (0, 0);

        let mut position_current = position_start;
        let mut position_previous = position_start;
        'outer: loop {
            let tile_current = maze.get(&position_current).unwrap();

            for direction in DIRECTIONS {
                let position_next = (
                    position_current.0 + direction.0,
                    position_current.1 + direction.1,
                );

                // We are now allowed to go back to the previous position
                if position_next == position_previous {
                    continue;
                }

                if let Some(tile_next) = maze.get(&position_next) {
                    if tile_current.connects(tile_next, direction) {
                        map.insert(position_current, Loop::from(direction));

                        position_previous = position_current;
                        position_current = position_next;

                        let s = tile_current.sides(direction);
                        sides.0 += s.0;
                        sides.1 += s.1;

                        if position_current == position_start {
                            end_direction = Loop::from(direction);
                            break 'outer;
                        }

                        continue 'outer;
                    }
                }
            }

            unreachable!("Unable to move forward...");
        }

        // Replace the start with the correct piece
        let start = match (end_direction, map.get(&position_start).unwrap()) {
            (Loop::North, Loop::North) | (Loop::South, Loop::South) => Maze::Vertical,
            (Loop::East, Loop::East) | (Loop::West, Loop::West) => Maze::Horizontal,
            (Loop::South, Loop::East) | (Loop::West, Loop::North) => Maze::NorthEast,
            (Loop::South, Loop::West) | (Loop::East, Loop::North) => Maze::NorthWest,
            (Loop::East, Loop::South) | (Loop::North, Loop::West) => Maze::SouthWest,
            (Loop::North, Loop::East) | (Loop::West, Loop::South) => Maze::SouthEast,
            _ => unreachable!(),
        };
        maze.insert(position_start, start);

        // Add the sides of the start
        let start_sides = start.sides(map.get(&position_start).unwrap().offset());
        sides.0 += start_sides.0;
        sides.1 += start_sides.1;

        // Determine if the loop was travelled clockwise or anti clockwise
        let clockwise = sides.0 > sides.1;

        let mut position_current = position_start;
        loop {
            let &direction = map.get(&position_current).unwrap();
            let tile = maze.get(&position_current).unwrap();

            // Get neighbouring tiles that are inside of the loop
            let checks = match direction {
                Loop::North => match tile {
                    Maze::Vertical => {
                        if clockwise {
                            vec![(1, 0)]
                        } else {
                            vec![(-1, 0)]
                        }
                    }
                    Maze::NorthEast => {
                        if clockwise {
                            vec![]
                        } else {
                            vec![(-1, 0), (0, 1)]
                        }
                    }
                    Maze::NorthWest => {
                        if clockwise {
                            vec![(1, 0), (0, 1)]
                        } else {
                            vec![]
                        }
                    }
                    _ => unreachable!(),
                },
                Loop::East => match tile {
                    Maze::Horizontal => {
                        if clockwise {
                            vec![(0, 1)]
                        } else {
                            vec![(0, -1)]
                        }
                    }
                    Maze::NorthEast => {
                        if clockwise {
                            vec![(-1, 0), (0, 1)]
                        } else {
                            vec![]
                        }
                    }
                    Maze::SouthEast => {
                        if clockwise {
                            vec![]
                        } else {
                            vec![(-1, 0), (0, -1)]
                        }
                    }
                    _ => unreachable!(),
                },
                Loop::South => match tile {
                    Maze::Vertical => {
                        if clockwise {
                            vec![(-1, 0)]
                        } else {
                            vec![(1, 0)]
                        }
                    }
                    Maze::SouthWest => {
                        if clockwise {
                            vec![]
                        } else {
                            vec![(0, -1), (1, 0)]
                        }
                    }
                    Maze::SouthEast => {
                        if clockwise {
                            vec![(0, -1), (-1, 0)]
                        } else {
                            vec![]
                        }
                    }
                    _ => unreachable!(),
                },
                Loop::West => match tile {
                    Maze::Horizontal => {
                        if clockwise {
                            vec![(0, -1)]
                        } else {
                            vec![(0, 1)]
                        }
                    }
                    Maze::NorthWest => {
                        if clockwise {
                            vec![]
                        } else {
                            vec![(1, 0), (0, 1)]
                        }
                    }
                    Maze::SouthWest => {
                        if clockwise {
                            vec![(1, 0), (0, -1)]
                        } else {
                            vec![]
                        }
                    }
                    _ => unreachable!(),
                },
                Loop::Inner => unreachable!("Loop should not contain Inner"),
            };

            // Perform a floodfill from those neighbouring tiles
            for check in checks {
                let check = (position_current.0 + check.0, position_current.1 + check.1);
                flood_fill(check, &mut map);
            }

            let direction = direction.offset();

            position_current.0 += direction.0;
            position_current.1 += direction.1;

            if position_current == position_start {
                break;
            }
        }

        // Count how many tiles are marked as inner
        map.iter()
            .filter(|(_, &value)| value == Loop::Inner)
            .count()
    }
}
