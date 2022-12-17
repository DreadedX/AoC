#![feature(test)]
use std::{str::FromStr, cmp::{min, max}};

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
        Day::test(Day::part1, "test-1", 26)
    }
    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 4883971)
    }
    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(Day::part2, "test-1", 56000011)
    }
    #[test]
    fn part2_solutin() -> Result<()> {
        Day::test(Day::part2, "input", 12691026767556)
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

#[derive(Debug)]
struct Position {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Sensor {
    position: Position,
    beacon: Position,
}

impl Sensor {
    fn distance(&self) -> isize {
        return (self.position.x - self.beacon.x).abs() + (self.position.y - self.beacon.y).abs();
    }
}

impl FromStr for Sensor {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" ").map(|entry| {
            entry.chars().filter(|c| c.is_digit(10) || *c == '-').collect::<String>()
        }).filter(|entry| !entry.is_empty())
        .map(|num| num.parse().unwrap());

        // Sensor
        let x = split.next().unwrap();
        let y = split.next().unwrap();
        let position = Position{ x, y };

        // Beacon
        let x = split.next().unwrap();
        let y = split.next().unwrap();
        let beacon = Position{ x, y };

        Ok(Self { position, beacon })
    }
}

#[derive(Debug, Copy, Clone)]
struct Range {
    start: isize,
    end: isize,
}

impl Range {
    fn len(&self) -> isize {
        self.end - self.start
    }
}

fn merge_step(ranges: &mut Vec<Range>) {
    let mut merged = vec![false; ranges.len()];
    let mut counter = 0;

    for idx in 0..ranges.len() {
        if merged[idx] {
            continue;
        }

        let mut r = ranges[idx];
        for (jdx, other) in ranges.iter().enumerate().skip(idx+1) {
            if ranges[idx].end < other.start || other.end < ranges[idx].start {
                continue;
            }

            merged[idx] = true;
            merged[jdx] = true;

            r.start = min(r.start, other.start);
            r.end = max(r.end, other.end);
        }

        ranges[counter] = r;
        counter += 1;
    }

    ranges.truncate(counter);
}

fn merge(ranges: &mut Vec<Range>) {
    loop {
        let old_len = ranges.len();
        merge_step(ranges);

        if old_len == ranges.len() {
            break;
        }
    }
}

// @TODO Implement this without doing a bunch of memory allocation
fn get_ranges(sensors: &Vec<Sensor>, y_level: isize, exclude_beacons: bool) -> Vec<Range> {
    let mut ranges = Vec::new();
    for sensor in sensors {
        let offset = sensor.distance() - (sensor.position.y - y_level).abs();
        if offset < 0 {
            continue;
        }

        let mut start = sensor.position.x - offset;
        let mut end = sensor.position.x + offset + 1;

        if exclude_beacons && sensor.beacon.y == y_level && sensor.beacon.x >= start && sensor.beacon.x < end {
            // If there is a beacon in the range we actually have to create two different
            // ranges that exclude the beacon
            if sensor.beacon.x != start && sensor.beacon.x+1 != end {
                ranges.push(Range { start, end: sensor.beacon.x });
                start = sensor.beacon.x + 1;
            } else if sensor.beacon.x == start {
                start += 1;
            } else if sensor.beacon.x+1 == end {
                end -= 1;
            }
        }

        if end > start {
            ranges.push(Range{start,end});
        }
    }

    merge(&mut ranges);

    return ranges;
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = isize;
    type Output2 = isize;

    fn day() -> u8 {
        15
    }

    fn part1(input: &str) -> Self::Output1 {
        let sensors = input
            .lines()
            .flat_map(Sensor::from_str)
            .collect::<Vec<_>>();

        let y_level = if input.lines().count() > 14 {
            2000000
        } else {
            10
        };

        get_ranges(&sensors, y_level, true)
            .iter()
            // .inspect(|range| println!("{range:?}"))
            .map(Range::len)
            .sum()
    }

    fn part2(input: &str) -> Self::Output2 {
        let sensors = input
            .lines()
            .flat_map(Sensor::from_str)
            .collect::<Vec<_>>();

        let max = if input.lines().count() > 14 {
            4000000
        } else {
            20
        };

        for y_level in 0..(max+1) {
            let ranges = get_ranges(&sensors, y_level, false);

            if ranges.len() > 1 {
                let x = ranges[0].end;

                return x * 4000000 + y_level;
            }
        }

        return -1;
    }
}
