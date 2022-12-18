#![feature(test)]
use std::{collections::HashMap, str::FromStr, cmp::Ordering};

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
        Day::test(Day::part1, "test-1", 1651)
    }
    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 1647)
    }
    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(Day::part2, "test-1", 1707)
    }
    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", 2169)
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

#[derive(Debug, Clone)]
struct Valve {
    name: String,
    flowrate: i32,
    connections: Vec<String>,
}

#[derive(Debug, Clone)]
struct Volcano {
    valves: Vec<Valve>,
    dist: Vec<Vec<i32>>,
    size: usize,
}

const STARTING_NAME: &str = "AA";

impl FromStr for Volcano {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // Parse the input into a vector of valves
        let mut valves = input
            .lines()
            .map(|line| {
                let mut iter = line.splitn(10, " ");

                let name = iter.nth(1).unwrap().into();
                let flowrate = iter.nth(2).unwrap().chars().filter(|c| c.is_digit(10)).collect::<String>().parse().unwrap();

                let connections = iter.nth(4).unwrap().split(", ").map(|name| name.into()).collect();

                Valve {name, flowrate, connections}
            }).collect::<Vec<_>>();

        // Sort the valves such that the starting point is first followed by the valves in order of
        // highest to lowest flowrate
        valves.sort_by(|a, b| {
            // Make sure AA is always in the first index
            if a.name == STARTING_NAME {
                return Ordering::Less;
            } else if b.name == STARTING_NAME {
                return Ordering::Greater;
            }

            b.flowrate.cmp(&a.flowrate)
        });

        // Create a lookup that allows looking up the index of a valve based on name
        let mut lookup = HashMap::new();
        for (idx, valve) in valves.iter().enumerate() {
            lookup.insert(valve.name.to_owned(), idx);
            // println!("{idx}: {} [{}]", valve.flowrate, valve.name);
        }

        // === FLOYD-WARSHALL ===
        // Create a distance array for Floyd-Warshall initialize with a very large number
        let size = valves.len();
        let mut dist = vec![vec![i32::MAX / 4; size]; size];

        // Fill the initial distances
        for (from, valve) in valves.iter().enumerate() {
            for other in valve.connections.iter() {
                let to = lookup.get(other).unwrap();

                dist[from][*to] = 1;
            }
        }

        // Distance to self is always zero
        for i in 0..size {
            dist[i][i] = 0;
        }

        // Update all the distances
        for k in 0..size {
            for i in 0..size {
                for j in 0..size {
                    if dist[i][j] > dist[i][k] + dist[k][j] {
                        dist[i][j] = dist[i][k] + dist[k][j]
                    }
                }
            }
        }

        // Count how many valves have a non-zero flowrate
        let size = valves.iter().filter(|valve| {
            valve.flowrate > 0 || valve.name == STARTING_NAME
        }).count();

        // Truncate the distance map to only include valves that have non-zero flowrate
        // As we are never interested in these as a destination
        dist.truncate(size);
        for row in dist.iter_mut() {
            row.truncate(size);
        }

        // print!("    ");
        // for idx in 0..size {
        //     print!("{idx:>2} ");
        // }
        // println!("");
        // for from in 0..size {
        //     print!("{from:>2}: ");
        //     for to in 0..size {
        //         print!("{:>2} ", dist[from][to]);
        //     }
        //     println!("");
        // }

        Ok(Volcano { valves, dist, size })
    }
}

impl Volcano {
    fn visit(&self, state: impl State) -> i32 {
        // There is no time remaining anymore, so no extra pressure is released
        if state.get_time_remaining() < 0 {
            return 0;
        }

        // We have just moved to this valve and opened it
        // Calculate how much pressure we have just released
        let released_here = state.get_time_remaining() * self.valves[state.get_pos()].flowrate;

        // Visit the next valve
        let mut best = 0;
        for idx in 1..self.size {
            // But only if it has not been opened yet
            if state.is_open(idx) {
                continue;
            }

            // Calculate how much it costs to move starting from the location of next 'person' to
            // move
            let cost = self.dist[state.get_pos_next()][idx] + 1;

            // Go to the next valve, this will give us the best we can do after going to that
            // valve
            let released = self.visit(state.next(idx, cost));

            // If it is the best option so far, store it
            best = best.max(released);
        }

        return best + released_here;
    }
}

trait State {
    fn new(time_remaining: i32) -> Self;
    fn next(&self, idx: usize, cost: i32) -> Self;
    fn is_open(&self, idx: usize) -> bool;
    fn get_pos(&self) -> usize;
    fn get_pos_next(&self) -> usize;
    fn get_time_remaining(&self) -> i32;
}

#[derive(Copy, Clone)]
struct StateSingle {
    pos_player: usize,
    time_remaining: i32,
    // We can have a max of 64 valves if we store it like this
    opened: i64,
}

impl State for StateSingle {
    fn new(time_remaining: i32) -> Self {
        Self {
            // Player starts in AA (idx: 0)
            pos_player: 0,
            time_remaining,
            // Start with AA marked as opened so we do not visit it again
            opened: 1,
        }
    }

    fn next(&self, idx: usize, cost: i32) -> Self {
        Self {
            pos_player: idx,
            time_remaining: self.time_remaining - cost,
            opened: self.opened | 1 << idx
        }
    }

    fn is_open(&self, idx: usize) -> bool {
        (self.opened >> idx) & 0x01 == 1
    }

    fn get_pos(&self) -> usize {
        self.pos_player
    }

    fn get_pos_next(&self) -> usize {
        self.pos_player
    }

    fn get_time_remaining(&self) -> i32 {
        self.time_remaining
    }
}

#[derive(Copy, Clone)]
struct StateDouble {
    pos_player: usize,
    pos_elephant: usize,
    time_remaining_player: i32,
    time_remaining_elephant: i32,
    player_turn: bool,
    // We can have a max of 64 valves if we store it like this
    opened: i64,
}

impl State for StateDouble {
    fn new(time_remaining: i32) -> Self {
        Self {
            // Player starts in AA (idx: 0)
            pos_player: 0,
            pos_elephant: 0,
            time_remaining_player: time_remaining,
            time_remaining_elephant: time_remaining,
            player_turn: true,
            // Start with AA marked as opened so we do not visit it again
            opened: 1,
        }
    }

    fn next(&self, idx: usize, cost: i32) -> Self {
        let player_turn = !self.player_turn;

        let time_remaining_player = if player_turn {
            self.time_remaining_player - cost
        } else {
            self.time_remaining_player
        };

        let time_remaining_elephant = if !player_turn {
            self.time_remaining_elephant - cost
        } else {
            self.time_remaining_elephant
        };

        let pos_player = if player_turn {
            idx
        } else {
            self.pos_player
        };
        let pos_elephant = if !player_turn {
            idx
        } else {
            self.pos_elephant
        };

        Self {
            pos_player,
            pos_elephant,
            time_remaining_player,
            time_remaining_elephant,
            player_turn: !self.player_turn,
            opened: self.opened | (1 << idx),
        }
    }

    fn get_pos(&self) -> usize {
        if self.player_turn {
            self.pos_player
        } else {
            self.pos_elephant
        }
    }

    fn get_pos_next(&self) -> usize {
        if self.player_turn {
            self.pos_elephant
        } else {
            self.pos_player
        }
    }

    fn get_time_remaining(&self) -> i32 {
        if self.player_turn {
            self.time_remaining_player
        } else {
            self.time_remaining_elephant
        }
    }

    fn is_open(&self, idx: usize) -> bool {
        (self.opened >> idx) & 0x01 == 1
    }
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = i32;
    type Output2 = i32;

    fn day() -> u8 {
        16
    }

    fn part1(input: &str) -> Self::Output1 {
        let volcano = Volcano::from_str(input).unwrap();
        volcano.visit(StateSingle::new(30))
    }

    fn part2(input: &str) -> Self::Output2 {
        let volcano = Volcano::from_str(input).unwrap();
        volcano.visit(StateDouble::new(26))
    }
}
