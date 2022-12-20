#![feature(test)]
use std::{ops::{AddAssign, SubAssign}, str::FromStr};

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
        Day::test(Day::part1, "test-1", 33)
    }
    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 600)
    }
    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(Day::part2, "test-1", 56*62)
    }
    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", 6000)
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

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Resources {
    ore: isize,
    clay: isize,
    obsidian: isize,
}

impl Resources {
    fn new(ore: isize, clay: isize, obsidian: isize) -> Self {
        Self { ore, clay, obsidian }
    }

    fn enough_for(&self, other: &Resources) -> bool {
        self.ore >= other.ore &&
            self.clay >= other.clay &&
            self.obsidian >= other.obsidian
    }
}

impl AddAssign for Resources {
    fn add_assign(&mut self, rhs: Self) {
        self.ore += rhs.ore;
        self.clay += rhs.clay;
        self.obsidian += rhs.obsidian;
    }
}

impl SubAssign for Resources {
    fn sub_assign(&mut self, rhs: Self) {
        self.ore -= rhs.ore;
        self.clay -= rhs.clay;
        self.obsidian -= rhs.obsidian;
    }
}

#[derive(Debug)]
struct Blueprint {
    ore_robot_cost: Resources,
    clay_robot_cost: Resources,
    obsidian_robot_cost: Resources,
    geode_robot_cost: Resources,
    max: Resources,
}

impl Blueprint {
    fn new(ore_robot_cost: Resources, clay_robot_cost: Resources, obsidian_robot_cost: Resources, geode_robot_cost: Resources) -> Self {
        // Calculate how much of each resource we need at most every minute in order to construct
        // every robot
        let mut max = Resources::new(0, 0, 0);

        max.ore = max.ore.max(ore_robot_cost.ore).max(clay_robot_cost.ore).max(obsidian_robot_cost.ore).max(geode_robot_cost.ore);
        max.clay = max.clay.max(ore_robot_cost.clay).max(clay_robot_cost.clay).max(obsidian_robot_cost.clay).max(geode_robot_cost.clay);
        max.obsidian = max.obsidian.max(ore_robot_cost.obsidian).max(clay_robot_cost.obsidian).max(obsidian_robot_cost.obsidian).max(geode_robot_cost.obsidian);

        Self { ore_robot_cost, clay_robot_cost, obsidian_robot_cost, geode_robot_cost, max }
    }
}

impl FromStr for Blueprint {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" ");

        let ore = Resources::new(
            split.nth(6).unwrap().parse()?,
            0,
            0,
        );

        let clay = Resources::new(
            split.nth(5).unwrap().parse()?,
            0,
            0,
        );

        let obsidian = Resources::new(
            split.nth(5).unwrap().parse()?,
            split.nth(2).unwrap().parse()?,
            0,
        );

        let geode = Resources::new(
            split.nth(5).unwrap().parse()?,
            0,
            split.nth(2).unwrap().parse()?,
        );

        Ok(Blueprint::new(ore, clay, obsidian, geode))
    }
}

impl Blueprint {
    fn visit(&self, state: State, mut best: isize) -> isize {
        // If we have run out of time return the amount of geodes we have broken in total
        if state.time_remaining <= 0 {
            return state.geodes;
        }

        // Using n(n+1)/2 calculate the maximum possible geodes that we can crack
        // If we can not improve the best so far, we end here
        // !!! This make a massive difference, from who knows how long to less then a second
        if state.geodes + (state.time_remaining-1) * state.time_remaining / 2 < best {
            return 0;
        }

        // Given the remaining time, calculate how much of the resource we still need in the worst
        // case scenerio. If current stockpile + future production if lower we will attempt to
        // construct a robot, otherwise we do not need them anymore
        // === Ore ===
        if state.robots.ore * state.time_remaining + state.resources.ore < self.max.ore * state.time_remaining {
            // Check if we can construct one right now
            if state.resources.enough_for(&self.ore_robot_cost) {
                // Create a new state
                let mut next = state.next();
                // Substract the resources
                next.resources -= self.ore_robot_cost;
                // Add the robot
                next.robots.ore += 1;

                best = best.max(self.visit(next, best));
            } else {
                // If we can not construct one right now, skip to the next point in time that we
                // can build one
                let mut next = state.next();
                while !next.resources.enough_for(&self.ore_robot_cost) && next.time_remaining >= 0 {
                    next = next.next();
                }

                best = best.max(self.visit(next, best));
            }
        }

        // === Clay ===
        if state.robots.clay * state.time_remaining + state.resources.clay < self.max.clay * state.time_remaining {
            if state.resources.enough_for(&self.clay_robot_cost) {
                // Create a new state
                let mut next = state.next();
                // Substract the resources
                next.resources -= self.clay_robot_cost;
                // Add the robot
                next.robots.clay += 1;

                best = best.max(self.visit(next, best));
            } else {
                // If we can not construct one right now, skip to the next point in time that we
                // can build one
                let mut next = state.next();
                while !next.resources.enough_for(&self.clay_robot_cost) && next.time_remaining >= 0 {
                    next = next.next();
                }

                best = best.max(self.visit(next, best));
            }
        }

        // === Obsidian ===
        if state.robots.obsidian * state.time_remaining + state.resources.obsidian < self.max.obsidian * state.time_remaining {
            if state.resources.enough_for(&self.obsidian_robot_cost) {
                // Create a new state
                let mut next = state.next();
                // Substract the resources
                next.resources -= self.obsidian_robot_cost;
                // Add the robot
                next.robots.obsidian += 1;

                best = best.max(self.visit(next, best));
            } else {
                // If we can not construct one right now, skip to the next point in time that we
                // can build one
                let mut next = state.next();
                while !next.resources.enough_for(&self.obsidian_robot_cost) && next.time_remaining >= 0 {
                    next = next.next();
                }

                best = best.max(self.visit(next, best));
            }
        }

        // === Geode ==
        // There is no upper limit to producing robots, so only check if we can make one
        if state.resources.enough_for(&self.geode_robot_cost) {
            // Create a new state
            let mut next = state.next();
            // Substract the resources
            next.resources -= self.geode_robot_cost;
            // Since we never use the geodes we can add all that this robot will mine at once
            next.geodes += state.time_remaining - 1;

            best = best.max(self.visit(next, best));
        } else {
            // If we can not construct one right now, skip to the next point in time that we
            // can build one
            let mut next = state.next();
            while !next.resources.enough_for(&self.geode_robot_cost) && next.time_remaining >= 0 {
                next = next.next();
            }

            best = best.max(self.visit(next, best));
        }

        return best;
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State {
    resources: Resources,
    robots: Resources,
    geodes: isize,
    time_remaining: isize,
}

impl State {
    fn new(time_remaining: isize) -> Self {
        Self {
            resources: Resources::new(0, 0, 0),
            robots: Resources::new(1, 0, 0),
            geodes: 0,
            time_remaining,
        }
    }

    fn next(&self) -> Self {
        let mut next = self.clone();

        // Update the time
        next.time_remaining -= 1;
        // Collect resources
        next.resources += self.robots;

        return next;
    }
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = isize;
    type Output2 = isize;

    fn day() -> u8 {
        19
    }

    fn part1(input: &str) -> Self::Output1 {
        let blueprints = input.trim().lines().flat_map(Blueprint::from_str).collect::<Vec<_>>();

        let state = State::new(24);
        blueprints.iter()
            .enumerate()
            .map(|(idx, blueprint)| {
                blueprint.visit(state.clone(), 0) * (idx as isize + 1)
            }).sum()
    }

    fn part2(input: &str) -> Self::Output2 {
        let blueprints = input.trim().lines().flat_map(Blueprint::from_str).collect::<Vec<_>>();
        let state = State::new(32);
        blueprints.iter()
            .take(3)
            .fold(1, |acc, blueprint| {
                acc * blueprint.visit(state.clone(), 0)
            })
    }
}
