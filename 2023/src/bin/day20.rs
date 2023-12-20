#![feature(test)]
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
        Day::test(Day::part1, "test-1", 32000000)
    }

    #[test]
    fn part1_test2() -> Result<()> {
        Day::test(Day::part1, "test-2", 11687500)
    }

    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 666795063)
    }

    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", 253302889093151)
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
enum ModuleType<'a> {
    Broadcaster,
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, bool>),
}

#[derive(Debug, Clone)]
struct Module<'a> {
    module_type: ModuleType<'a>,
    destinations: Vec<&'a str>,
}

impl<'a> Module<'a> {
    fn process(&mut self, source: &'a str, high: bool) -> Option<bool> {
        match &mut self.module_type {
            ModuleType::Broadcaster => Some(high),
            ModuleType::FlipFlop(current) => {
                if high {
                    None
                } else {
                    *current = !*current;
                    Some(*current)
                }
            }
            ModuleType::Conjunction(inputs) => {
                let input = inputs.get_mut(source).unwrap();
                *input = high;

                Some(!inputs.iter().all(|(_, prev)| *prev))
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Pulse<'a> {
    source: &'a str,
    destination: &'a str,
    high: bool,
}

// Copied from day 8
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

// Copied from day 8
fn lcm(a: usize, b: usize) -> usize {
    if a > b {
        (a / gcd(a, b)) * b
    } else {
        (b / gcd(a, b)) * a
    }
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = usize;
    type Output2 = usize;

    fn day() -> u8 {
        20
    }

    fn part1(input: &str) -> Self::Output1 {
        let mut modules: HashMap<_, _> = input
            .lines()
            .map(|line| {
                let (name, destinations) = line.split_once(" -> ").unwrap();
                let destinations: Vec<_> = destinations.split(", ").collect();

                let (name, module_type) = if name.starts_with('%') {
                    (name.split_at(1).1, ModuleType::FlipFlop(false))
                } else if name.starts_with('&') {
                    (name.split_at(1).1, ModuleType::Conjunction(HashMap::new()))
                } else if name == "broadcaster" {
                    (name, ModuleType::Broadcaster)
                } else {
                    unreachable!("Invalid input");
                };

                (
                    name,
                    Module {
                        module_type,
                        destinations,
                    },
                )
            })
            .collect();

        // TODO: Because of the borrow check we have to create a clone of modules here
        for (name, module) in modules.clone() {
            for destination in &module.destinations {
                if let Some(module) = modules.get_mut(destination) {
                    if let ModuleType::Conjunction(inputs) = &mut module.module_type {
                        inputs.insert(name, false);
                    }
                }
            }
        }

        let mut count = (0, 0);
        let mut pulses = VecDeque::new();
        for _ in 0..1000 {
            pulses.push_back(Pulse {
                source: "button",
                destination: "broadcaster",
                high: false,
            });

            while let Some(pulse) = pulses.pop_front() {
                if pulse.high {
                    count.1 += 1;
                } else {
                    count.0 += 1;
                }

                if let Some(module) = modules.get_mut(pulse.destination) {
                    if let Some(high) = module.process(pulse.source, pulse.high) {
                        for destination in &module.destinations {
                            pulses.push_back(Pulse {
                                source: pulse.destination,
                                destination,
                                high,
                            });
                        }
                    }
                }
            }
        }

        count.0 * count.1
    }

    fn part2(input: &str) -> Self::Output2 {
        let mut modules: HashMap<_, _> = input
            .lines()
            .map(|line| {
                let (name, destinations) = line.split_once(" -> ").unwrap();
                let destinations: Vec<_> = destinations.split(", ").collect();

                let (name, module_type) = if name.starts_with('%') {
                    (name.split_at(1).1, ModuleType::FlipFlop(false))
                } else if name.starts_with('&') {
                    (name.split_at(1).1, ModuleType::Conjunction(HashMap::new()))
                } else if name == "broadcaster" {
                    (name, ModuleType::Broadcaster)
                } else {
                    unreachable!("Invalid input");
                };

                (
                    name,
                    Module {
                        module_type,
                        destinations,
                    },
                )
            })
            .collect();

        // TODO: Because of the borrow check we have to create a clone of modules here
        for (name, module) in modules.clone() {
            for destination in &module.destinations {
                if let Some(module) = modules.get_mut(destination) {
                    if let ModuleType::Conjunction(inputs) = &mut module.module_type {
                        inputs.insert(name, false);
                    }
                }
            }
        }

        let r#final = modules
            .iter()
            .find(|(_, module)| module.destinations.contains(&"rx"))
            .unwrap();

        let final_name = r#final.0.to_owned();
        let final_module = r#final.1.clone();
        let mut frequencies = HashMap::new();

        println!("{final_name}: {final_module:?}");

        let mut pulses = VecDeque::new();
        for i in 0..10000 {
            pulses.push_back(Pulse {
                source: "button",
                destination: "broadcaster",
                high: false,
            });

            while let Some(pulse) = pulses.pop_front() {
                if let Some(module) = modules.get_mut(pulse.destination) {
                    if pulse.destination == final_name && pulse.high {
                        frequencies.insert(pulse.source, i + 1);
                        if let ModuleType::Conjunction(ref inputs) = final_module.module_type {
                            if frequencies.len() == inputs.len() {
                                return frequencies.values().copied().fold(1, lcm);
                            }
                        }
                    }

                    if let Some(high) = module.process(pulse.source, pulse.high) {
                        for destination in &module.destinations {
                            pulses.push_back(Pulse {
                                source: pulse.destination,
                                destination,
                                high,
                            });
                        }
                    }
                }
            }
        }

        unreachable!("No solution found");
    }
}
