#![feature(test)]
use std::{collections::{HashMap, VecDeque, HashSet}, str::FromStr};

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
    flowrate: i32,
    connections: Vec<String>,
}

#[derive(Debug)]
struct Volcano {
    valves: HashMap<String, Valve>,
}

impl FromStr for Volcano {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let valves = input
            .lines()
            .map(|line| {
                let mut iter = line.splitn(10, " ");

                let name = iter.nth(1).unwrap().into();
                let flowrate = iter.nth(2).unwrap().chars().filter(|c| c.is_digit(10)).collect::<String>().parse().unwrap();

                let connections = iter.nth(4).unwrap().split(", ").map(|name| name.into()).collect();

                (name, Valve {flowrate, connections})
            }).collect();

        Ok(Volcano { valves })
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct State {
    name: String,
    time: i32,
    opened: Vec<String>,
}

fn find_best(root: String, volcano: &Volcano, opened: Vec<String>, time: i32) -> (i32, Vec<String>) {
    let mut queue = VecDeque::new();
    queue.push_back((State{name: root, time, opened}, 0));

    let mut best = 0;
    let mut best_opened = Vec::new();
    let mut evaluated = HashSet::new();
    loop {
        // We are done now
        if queue.is_empty() {
            break;
        }

        let mut state = queue.pop_front().unwrap();

        // Check if we have run out of time
        if state.0.time <= 1 {
            if state.1 > best {
                best = state.1;
                best_opened = state.0.opened;
            }
            continue;
        }

        if evaluated.contains(&state.0) {
            continue;
        }

        let current_valve = volcano.valves.get(&state.0.name).unwrap();

        // Two options:
        // 1: Open valve [Only if current valve is not opened and has a
        //    non-zero flowrate]
        // 2: Move to other valve

        // Option 1
        if !state.0.opened.contains(&state.0.name) && current_valve.flowrate != 0 {
            // Add the current valve to the list of opened valves
            state.0.opened.push(state.0.name.clone());

            let new_value = state.1 + (state.0.time-1) * current_valve.flowrate;
            let ns = (State {name: state.0.name.to_owned(), time: state.0.time-1, opened: state.0.opened.clone()}, new_value);
            queue.push_back(ns);

            state.0.opened.pop();
        }

        // Option 2
        for connection in current_valve.connections.iter() {
            let ns = (State {name: connection.to_owned(), time: state.0.time-1, opened: state.0.opened.clone()}, state.1);
            queue.push_back(ns);
        }

        evaluated.insert(State{name: state.0.name.to_owned(), time: state.0.time, opened: state.0.opened.clone()});
    }

    return (best, best_opened);
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

        find_best("AA".to_owned(), &volcano, Vec::new(), 30).0
    }

    fn part2(input: &str) -> Self::Output2 {
        let volcano = Volcano::from_str(input).unwrap();

        let time = 26;


        // This solution is very much a hack
        // In the 26 minutes we can not turn on all the valves
        // So the player tries to go for the best possible solution before running out of time
        // The elephant will then look at the remaining valves and find the best remaing solution
        // Problem with this solution is that it assumes we run out of time before opening all
        // non-zero valves
        // However this is not the case in the example, so it will actually fail the example
        // @TODO Implement a proper solution that can also solve the example
        let player = find_best("AA".to_owned(), &volcano, Vec::new(), time);
        let elephant = find_best("AA".to_owned(), &volcano, player.1, time);

        player.0 + elephant.0
    }
}
