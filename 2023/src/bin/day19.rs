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
        Day::test(Day::part1, "test-1", 19114)
    }

    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 480738)
    }

    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(Day::part2, "test-1", 167409079868000)
    }

    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", 131550418841958)
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
enum Next<'a> {
    Step(&'a str),
    Accept,
    Reject,
}

#[derive(Debug, Clone, Copy)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, Clone, Copy)]
struct Part2 {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}

impl Part2 {
    fn split(&self, category: Category, value: usize) -> (Option<Part2>, Option<Part2>) {
        let range = match category {
            Category::Cool => self.x,
            Category::Musical => self.m,
            Category::Aerodynamic => self.a,
            Category::Shiny => self.s,
        };

        let ranges = if range.0 > value {
            (None, Some(range))
        } else if range.1 < value {
            (Some(range), None)
        } else {
            (Some((range.0, value - 1)), Some((value, range.1)))
        };

        (
            self.replace(category, ranges.0),
            self.replace(category, ranges.1),
        )
    }

    fn replace(&self, category: Category, range: Option<(usize, usize)>) -> Option<Part2> {
        range.map(|range| match category {
            Category::Cool => Part2 {
                x: range,
                m: self.m,
                a: self.a,
                s: self.s,
            },
            Category::Musical => Part2 {
                x: self.x,
                m: range,
                a: self.a,
                s: self.s,
            },
            Category::Aerodynamic => Part2 {
                x: self.x,
                m: self.m,
                a: range,
                s: self.s,
            },
            Category::Shiny => Part2 {
                x: self.x,
                m: self.m,
                a: self.a,
                s: range,
            },
        })
    }

    fn combinations(&self) -> usize {
        (self.x.1 - self.x.0 + 1)
            * (self.m.1 - self.m.0 + 1)
            * (self.a.1 - self.a.0 + 1)
            * (self.s.1 - self.s.0 + 1)
    }
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    GreaterThan(Category, usize),
    LessThan(Category, usize),
    Otherwise,
}

#[derive(Debug, Clone, Copy)]
enum Category {
    Cool,
    Musical,
    Aerodynamic,
    Shiny,
}

#[derive(Debug, Clone, Copy)]
struct Step<'a> {
    op: Operator,
    next: Next<'a>,
}

impl<'a> Step<'a> {
    fn process(&self, part: &Part) -> Option<Next<'a>> {
        let ok = match self.op {
            Operator::GreaterThan(Category::Cool, value) => part.x > value,
            Operator::GreaterThan(Category::Musical, value) => part.m > value,
            Operator::GreaterThan(Category::Aerodynamic, value) => part.a > value,
            Operator::GreaterThan(Category::Shiny, value) => part.s > value,
            Operator::LessThan(Category::Cool, value) => part.x < value,
            Operator::LessThan(Category::Musical, value) => part.m < value,
            Operator::LessThan(Category::Aerodynamic, value) => part.a < value,
            Operator::LessThan(Category::Shiny, value) => part.s < value,
            Operator::Otherwise => true,
        };

        if ok {
            Some(self.next)
        } else {
            None
        }
    }
}

fn process_step(step: &str) -> Step {
    if let Some((instruction, next)) = step.split_once(':') {
        let next = match next {
            "A" => Next::Accept,
            "R" => Next::Reject,
            n => Next::Step(n),
        };

        let (category, value) = instruction.split_once(['<', '>']).unwrap();
        let category = match category {
            "x" => Category::Cool,
            "m" => Category::Musical,
            "a" => Category::Aerodynamic,
            "s" => Category::Shiny,
            _ => unreachable!("Invalid input"),
        };
        let value = value.parse().unwrap();

        let op = if instruction.contains('<') {
            Operator::LessThan(category, value)
        } else {
            Operator::GreaterThan(category, value)
        };

        Step { op, next }
    } else {
        let next = match step {
            "A" => Next::Accept,
            "R" => Next::Reject,
            n => Next::Step(n),
        };

        Step {
            op: Operator::Otherwise,
            next,
        }
    }
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = usize;
    type Output2 = usize;

    fn day() -> u8 {
        19
    }

    fn part1(input: &str) -> Self::Output1 {
        let (workflows, parts) = input.split_once("\n\n").unwrap();

        let workflows = workflows
            .lines()
            .map(|workflow| {
                let (name, mut rest) = workflow.split_once('{').unwrap();

                let mut steps = Vec::new();

                while let Some((step, remaining)) = rest.split_once(',') {
                    rest = remaining;
                    steps.push(process_step(step));
                }

                steps.push(process_step(rest.split_once('}').unwrap().0));

                (name, steps)
            })
            .collect::<HashMap<_, _>>();

        let parts = parts
            .lines()
            .map(|part| {
                let part: Vec<usize> = part
                    .split_once('{')
                    .unwrap()
                    .1
                    .split_once('}')
                    .unwrap()
                    .0
                    .splitn(4, ',')
                    .map(|category| category.split_once('=').unwrap().1.parse().unwrap())
                    .collect();

                if let [x, m, a, s] = part.as_slice() {
                    Part {
                        x: *x,
                        m: *m,
                        a: *a,
                        s: *s,
                    }
                } else {
                    unreachable!("Invalid input");
                }
            })
            .collect::<Vec<_>>();

        parts
            .iter()
            .map(|part| {
                let mut workflow = workflows.get("in").unwrap();

                while let Some(next) = workflow.iter().find_map(|step| step.process(part)) {
                    match next {
                        Next::Step(name) => workflow = workflows.get(name).unwrap(),
                        Next::Accept => return part.sum(),
                        Next::Reject => return 0,
                    }
                }

                0
            })
            .sum()
    }

    fn part2(input: &str) -> Self::Output2 {
        let (workflows, _) = input.split_once("\n\n").unwrap();

        let workflows = workflows
            .lines()
            .map(|workflow| {
                let (name, mut rest) = workflow.split_once('{').unwrap();

                let mut steps = Vec::new();

                while let Some((step, remaining)) = rest.split_once(',') {
                    rest = remaining;
                    steps.push(process_step(step));
                }

                steps.push(process_step(rest.split_once('}').unwrap().0));

                (name, steps)
            })
            .collect::<HashMap<_, _>>();

        let part = Part2 {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        };

        let mut queue = VecDeque::new();
        queue.push_back((part, "in"));

        let mut combinations = 0;
        while let Some((mut part, name)) = queue.pop_front() {
            let workflow = workflows.get(name).unwrap();

            for step in workflow {
                let (ok, fail) = match step.op {
                    Operator::GreaterThan(category, value) => {
                        let (a, b) = part.split(category, value + 1);

                        (b, a)
                    }
                    Operator::LessThan(category, value) => {
                        let (a, b) = part.split(category, value);

                        (a, b)
                    }
                    Operator::Otherwise => (Some(part), None),
                };

                // The part that matches goes to the next workflow
                if let Some(ok) = ok {
                    match step.next {
                        Next::Step(name) => queue.push_back((ok, name)),
                        Next::Accept => {
                            let a = ok.combinations();
                            combinations += a;
                        }
                        Next::Reject => {}
                    }
                }

                // The part that failed goes to the next step
                if let Some(fail) = fail {
                    part = fail;
                }
            }
        }

        combinations
    }
}
