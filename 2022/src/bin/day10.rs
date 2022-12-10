#![feature(test)]
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
        Day::test(Day::part1, "test-1", 13140)
    }
    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 13220)
    }
    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(Day::part2, "test-1", r"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
".to_owned())
    }
    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", r"###..#..#..##..#..#.#..#.###..####.#..#.
#..#.#..#.#..#.#.#..#..#.#..#.#....#.#..
#..#.#..#.#..#.##...####.###..###..##...
###..#..#.####.#.#..#..#.#..#.#....#.#..
#.#..#..#.#..#.#.#..#..#.#..#.#....#.#..
#..#..##..#..#.#..#.#..#.###..####.#..#.
".to_owned())
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

enum Instruction {
    NoOp,
    AddX(isize),
}

struct CPU {
    x: isize
}

impl CPU {
    fn new() -> Self {
        Self { x: 1 }
    }
    // Executes the given instruction
    // Returns the number of cycles that it took to execute the instruction
    // Also returns the value of x during the execution (SO NOT THE FINAL VALUE)
    fn execute(&mut self, instruction: &Instruction) -> (isize, isize) {
        match instruction {
            Instruction::NoOp => (1, self.x),
            Instruction::AddX(value) => {
                let state = (2, self.x);
                self.x += value;
                return state;
            }
        }
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input.lines()
        .map(|line| line.split(" ").collect::<Vec<_>>())
        .map(|line| match line.as_slice() {
            ["noop"] => Instruction::NoOp,
            ["addx", value] => Instruction::AddX(value.parse().unwrap()),
            _ => panic!("Unknown instruction")
        })
        .collect::<Vec<_>>()
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = isize;
    type Output2 = String;
    fn day() -> u8 {
        10
    }

    fn part1(input: &str) -> Self::Output1 {
        let instructions = parse(input);
        let mut cpu = CPU::new();

        let mut cycle = 1;
        let mut sum = 0;
        for instruction in instructions {
            let (cycle_count, x) = cpu.execute(&instruction);

            for c in 0..cycle_count {
                let cc = cycle + c;
                if (cc+ 20) % 40 == 0 {
                    sum += x * cc;
                }
            }

            cycle += cycle_count;
        }

        sum
    }

    fn part2(input: &str) -> Self::Output2 {
        let instructions = parse(input);
        let mut cpu = CPU::new();

        let mut cycle = 1;
        let mut output = "".to_owned();
        for instruction in instructions {
            let (cycle_count, x) = cpu.execute(&instruction);

            for c in 0..cycle_count {
                let ccm = (cycle + c - 1) % 40;
                let mut sign = ".";
                if ccm-1 == x || ccm == x || ccm+1 == x {
                    sign = "#";
                }
                output += sign;

                if ccm == 39 {
                    output += "\n";
                }
            }

            cycle += cycle_count;
        }

        output
    }
}
