#![feature(test)]
extern crate nalgebra as na;
use std::{collections::HashMap, convert::Infallible, str::FromStr};

use anyhow::Result;
use aoc::Solver;
use na::{Matrix6, Vector6};

// -- Runners --
fn main() -> Result<()> {
    Day::solve()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() -> Result<()> {
        Day::test(Day::part1, "test-1", 2)
    }

    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 13149)
    }

    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(Day::part2, "test-1", 47)
    }

    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", 1033770143421619)
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

#[derive(Debug, Clone, Copy)]
struct Hailstone {
    px: f64,
    py: f64,
    pz: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

impl Hailstone {
    fn intersect_2d(&self, other: &Hailstone) -> Option<(f64, f64)> {
        let dx = self.px - other.px;
        let dy = self.py - other.py;

        let d = self.vy * other.vx - self.vx * other.vy;

        let t1 = (other.vy * dx - other.vx * dy) / d;
        let t2 = (self.vy * dx - self.vx * dy) / d;

        if t1.is_sign_negative() || t2.is_sign_negative() {
            // Intersection is in the past
            return None;
        }

        let x = self.px + self.vx * t1;
        let y = self.py + self.vy * t1;

        if x.is_infinite() || y.is_infinite() {
            // Paths are parallel
            return None;
        }

        Some((x, y))
    }
}

impl FromStr for Hailstone {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<f64> = s
            .split([',', '@'])
            .map(|part| part.trim().parse().unwrap())
            .collect();

        Ok(Hailstone {
            px: parts[0],
            py: parts[1],
            pz: parts[2],
            vx: parts[3],
            vy: parts[4],
            vz: parts[5],
        })
    }
}

fn mode(numbers: &[usize]) -> usize {
    let mut occurrences = HashMap::<_, usize>::new();

    for &value in numbers {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    println!("{occurrences:?}");

    occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(value, _)| value)
        .unwrap()
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = usize;
    type Output2 = usize;

    fn day() -> u8 {
        24
    }

    fn part1(input: &str) -> Self::Output1 {
        let hailstones: Vec<_> = input.lines().flat_map(Hailstone::from_str).collect();

        let range = if hailstones.len() == 5 {
            7.0..=27.0
        } else {
            200000000000000.0..=400000000000000.0
        };

        hailstones
            .iter()
            .enumerate()
            .flat_map(|(index_a, a)| {
                hailstones
                    .iter()
                    .enumerate()
                    .filter(|(index_b, _)| index_b < &index_a)
                    .filter_map(|(_, b)| a.intersect_2d(b))
                    .collect::<Vec<_>>()
            })
            .filter(|&(x, y)| range.contains(&x) && range.contains(&y))
            .count()
    }

    fn part2(input: &str) -> Self::Output2 {
        let h: Vec<_> = input.lines().flat_map(Hailstone::from_str).collect();

        // Key insight for part 2 is
        //      p_rock + v_rock * t_i = p_i + v_i * t_i
        // for every hailstone i
        //
        // This can be rewritten as
        //      p_rock - p_i = t_i * (v_i - v_rock)
        // and since t_i is a scalar this means that
        // (p_rock - p_i) is parallel to (v_i - v_rock)
        // and therefore
        //      c_i = (p_rock - p_i) x (v_i - v_rock) = 0
        // This holds true for every hailstone i, so
        //      c_i = c_j
        //      c_i = c_k
        // with i != j != k
        // Writing out these equations leads to a set of six linear equations
        // And can therefore be written in matrix form
        //      A * solution = constant
        // And the solution can now be found using
        //      solution = A^-1 * constant

        let i = 0;
        let j = 1;
        // Due to numerical instability we run this with several different options for the third
        // hailstone
        let solutions: Vec<_> = (2..h.len())
            .map(|k| {
                // Constant in the matrix
                let c1 = h[i].vz - h[j].vz;
                let c2 = h[i].py - h[j].py;
                let c3 = h[j].vy - h[i].vy;
                let c4 = h[j].pz - h[i].pz;
                let c5 = h[i].vx - h[j].vx;
                let c6 = h[j].px - h[i].px;

                let c7 = h[i].vz - h[k].vz;
                let c8 = h[i].py - h[k].py;
                let c9 = h[k].vy - h[i].vy;
                let c10 = h[k].pz - h[i].pz;
                let c11 = h[i].vx - h[k].vx;
                let c12 = h[k].px - h[i].px;

                // Setup the matrix
                let matrix = Matrix6::new(
                    0.0, c1, c3, 0.0, c4, c2, -c1, 0.0, c5, -c4, 0.0, c6, -c3, -c5, 0.0, -c2, -c6,
                    0.0, 0.0, c7, c9, 0.0, c10, c8, -c7, 0.0, c11, -c10, 0.0, c12, -c9, -c11, 0.0,
                    -c8, -c12, 0.0,
                );

                // Get the inverse of the matrix
                let inverse = matrix.try_inverse().unwrap();

                // Constant on the rhs
                let k1 =
                    h[i].py * h[i].vz - h[j].py * h[j].vz + h[j].pz * h[j].vy - h[i].pz * h[i].vy;
                let k2 =
                    h[i].pz * h[i].vx - h[j].pz * h[j].vx + h[j].px * h[j].vz - h[i].px * h[i].vz;
                let k3 =
                    h[i].px * h[i].vy - h[j].px * h[j].vy + h[j].py * h[j].vx - h[i].py * h[i].vx;
                let k4 =
                    h[i].py * h[i].vz - h[k].py * h[k].vz + h[k].pz * h[k].vy - h[i].pz * h[i].vy;
                let k5 =
                    h[i].pz * h[i].vx - h[k].pz * h[k].vx + h[k].px * h[k].vz - h[i].px * h[i].vz;
                let k6 =
                    h[i].px * h[i].vy - h[k].px * h[k].vy + h[k].py * h[k].vx - h[i].py * h[i].vx;

                // Put them into a vector
                let k = Vector6::new(k1, k2, k3, k4, k5, k6);

                // Calclate the solution
                let solution = inverse * k;

                // The sum of all elements of the starting position is the answer
                (solution[0] + solution[1] + solution[2]).round() as usize
            })
            .collect();

        // The most common solution is the actual solution
        mode(&solutions)
    }
}
