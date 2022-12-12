#![feature(test)]
use core::fmt;
use std::{str::FromStr, cmp::{max, min}};

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
        Day::test(Day::part1, "test-1", 31)
    }
    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 481)
    }
    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(Day::part2, "test-1", 29)
    }
    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", 480)
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Position {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Node {
    height: u32,
    visited: bool,
    distance: u32,
}

#[derive(Debug)]
struct Hill {
    nodes: Vec<Vec<Node>>,
    size: (isize, isize),
    start: Position,
    end: Position,
}

impl fmt::Display for Hill {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.nodes.iter() {
            for node in line {
                if node.distance == u32::MAX {
                    write!(f, "[--.]")?;
                } else {
                    let mut v = '.';
                    if node.visited {
                        v = 'v';
                    }
                    write!(f, "[{:>2}{}]", node.distance, v)?;
                }
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

impl FromStr for Hill {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {

        let mut start = Position{x: 0, y: 0};
        let mut end = Position{x: 0, y: 0};

        let mut xmax = 0;
        let mut ymax = 0;

        let nodes = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                ymax = max(ymax, y+1);
                line
                    .chars()
                    .enumerate()
                    .map(|(x, mut letter)| {
                        xmax = max(xmax, x+1);
                        if letter == 'S' {
                            start = Position{x: x as isize, y: y as isize};
                            letter = 'a';
                        } else if letter == 'E' {
                            end = Position{x: x as isize, y: y as isize};
                            letter = 'z';
                        }
                        let height = convert_height(letter);
                        Node {height, visited: false, distance: u32::MAX}
                    }).collect()
            }).collect();

        Ok(Self { nodes, size: (xmax as isize, ymax as isize), start, end })
    }
}

impl Hill {
    fn dijkstra<T: Fn(u32, u32) -> bool>(&mut self, f: T) {
        let mut unvisited = (0..self.size.1).flat_map(|y| (0..self.size.0).map(move |x| Position{ x, y })).collect::<Vec<_>>();
        let mut current = self.start;
        self.nodes[current.y as usize][current.x as usize].distance = 0;
        loop {
            let mut neighbours = vec![
                Position{x: current.x, y: current.y-1}, // Up
                Position{x: current.x-1, y: current.y}, // Left
                Position{x: current.x, y: current.y+1}, // Down
                Position{x: current.x+1, y: current.y}, // Right
            ];

            let height = self.nodes[current.y as usize][current.x as usize].height;

            // We only want to update valid neighbours that have not been visited yet
            neighbours.retain(|n| {
                n.x >= 0 && n.x < self.size.0 && n.y >= 0 && n.y < self.size.1 && !self.nodes[n.y as usize][n.x as usize].visited && f(height, self.nodes[n.y as usize][n.x as usize].height)
            });

            // Update the distance of all the neighbours
            let distance = self.nodes[current.y as usize][current.x as usize].distance;
            for n in neighbours.iter() {
                let node = &mut self.nodes[n.y as usize][n.x as usize];
                node.distance = min(node.distance, distance+1);
            }

            // Mark current node as visited and remove it from the list of unvisited nodes
            self.nodes[current.y as usize][current.x as usize].visited = true;
            unvisited.retain(|p| *p != current);

            // We have visited all the nodes, so we are done
            if unvisited.len() == 0 {
                return;
            }

            // Get the next position we are going to visit, this will be the node with the lowest
            // distance value
            current = *unvisited.iter().reduce(|s, p| {
                if self.nodes[p.y as usize][p.x as usize].distance < self.nodes[s.y as usize][s.x as usize].distance {
                    p
                } else {
                    s
                }
            }).unwrap();

            // Remaining nodes are unreachable, we are done
            if self.nodes[current.y as usize][current.x as usize].distance == u32::MAX {
                return;
            }
        }

    }

    fn step_required(&self, position: Position) -> u32 {
        self.nodes[position.y as usize][position.x as usize].distance
    }
}

fn convert_height(letter: char) -> u32 {
    (letter as u8 - b'a') as u32
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = u32;
    type Output2 = u32;

    fn day() -> u8 {
        12
    }

    fn part1(input: &str) -> Self::Output1 {
        let mut hill = Hill::from_str(input).unwrap();
        // We can go down as many as we want, but only one up
        hill.dijkstra(|current, neighbour| neighbour <= current+1);
        hill.step_required(hill.end)
    }

    fn part2(input: &str) -> Self::Output2 {
        let mut hill = Hill::from_str(input).unwrap();

        // We now want to start from the end position
        std::mem::swap(&mut hill.start, &mut hill.end);

        // We are now starting from the end, so we need to flip the condition around
        // We can go up as many as we want, but we can only go down one
        hill.dijkstra(|current, neighbour| neighbour+1 >= current);

        hill.nodes.iter().flatten().filter(|node| node.height == 0).fold(u32::MAX, |acc, node| min(acc, node.distance))
    }
}
