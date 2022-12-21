#![feature(test)]
use core::fmt;
use std::{collections::HashMap, str::FromStr, borrow::Borrow};

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
        Day::test(Day::part1, "test-1", 152)
    }
    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 78342931359552)
    }
    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(Day::part2, "test-1", 301)
    }
    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", 3296135418820)
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

#[derive(Debug, Copy, Clone)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equality,
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operation::Add),
            "-" => Ok(Operation::Subtract),
            "*" => Ok(Operation::Multiply),
            "/" => Ok(Operation::Divide),
            _ => Err(anyhow::anyhow!("Invalid operation: {}", s)),
        }
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operation::Add => write!(f, "+"),
            Operation::Subtract => write!(f, "-"),
            Operation::Multiply => write!(f, "*"),
            Operation::Divide => write!(f, "/"),
            Operation::Equality => write!(f, "="),
        }
    }
}

#[derive(Debug, Clone)]
enum Expression {
    Simple(isize),
    Complex(Box<Expression>, Operation, Box<Expression>),
    Human,
}

impl Expression {
    fn evaluate(&self) -> Result<isize, anyhow::Error> {
        match self {
            Self::Simple(number) => Ok(*number),
            Self::Complex(a, op, b) => {
                let a = a.evaluate()?;
                let b = b.evaluate()?;
                match op {
                    Operation::Add => Ok(a + b),
                    Operation::Subtract => Ok(a - b),
                    Operation::Multiply => Ok(a * b),
                    Operation::Divide => Ok(a / b),
                    Operation::Equality => Err(anyhow::anyhow!("Cannot evaluate equality expression"))
                }
            },
            Self::Human => Err(anyhow::anyhow!("Cannot evaluate expression with human"))
        }
    }

    // This function assumes a lot about the shape of the expression
    // 1. Top level is an equality
    // 2. The side with the human variabl is the left side
    // 3. We never divide a simple number by a complex expression
    // Overall this function feels very poorly written/designed
    // But it works =D
    fn invert(self) -> Expression {
        if let Expression::Complex(mut left, Operation::Equality, mut right) = self {
            loop {
                match left.clone().borrow() {
                    // Once human the left side only contains human, we are done and return the right side
                    Expression::Human => return *right,
                    Expression::Complex(a, op, b) => {
                        match (a.clone().borrow(), b.clone().borrow()) {
                            (a @ (Expression::Complex(..) | Expression::Human), b @ Expression::Simple(..)) => {
                                // Update left to contain the complex side
                                *left = a.clone();
                                match op {
                                    Operation::Add => *right = Expression::Complex(right.clone(), Operation::Subtract, Box::new(b.clone())),
                                    Operation::Subtract => *right = Expression::Complex(right.clone(), Operation::Add, Box::new(b.clone())),
                                    Operation::Multiply => *right = Expression::Complex(right.clone(), Operation::Divide, Box::new(b.clone())),
                                    Operation::Divide => *right = Expression::Complex(right.clone(), Operation::Multiply, Box::new(b.clone())),
                                    _ => unreachable!("Did not expect this"),
                                }
                            },
                            (a @ Expression::Simple(..), b @ (Expression::Complex(..) | Expression::Human)) => {
                                // Update left to contain the complex side
                                *left = b.clone();
                                match op {
                                    Operation::Add => *right = Expression::Complex(right.clone(), Operation::Subtract, Box::new(a.clone())),
                                    Operation::Multiply => *right = Expression::Complex(right.clone(), Operation::Divide, Box::new(a.clone())),
                                    Operation::Subtract => *right = Expression::Complex(Box::new(a.clone()), Operation::Subtract, right.clone()),
                                    _ => unreachable!("Did not expect this"),
                                }
                            },
                            _ => unreachable!("Unexpected form"),
                        }
                    },
                    _ => unreachable!("Unexpected form to expression")
                }
            }
        } else {
            panic!("Unexpected form!");
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Simple(number) => write!(f, "{number}"),
            Self::Complex(a, op, b) => {
                match op {
                    Operation::Equality => write!(f, "{a} {op} {b}"),
                    _ => write!(f, "({a} {op} {b})")
                }
            },
            Self::Human => write!(f, "x"),
        }
    }
}

#[derive(Debug)]
enum Action {
    Number(Expression),
    Result(String, Operation, String),
    Human
}

impl Action {
    fn resolve(&self, map: &HashMap<String, Action>) -> Expression {
        match self {
            Action::Number(expr) => expr.clone(),
            Action::Result(a, op, b) => {
                let a = map.get(a).unwrap().resolve(map);
                let b = map.get(b).unwrap().resolve(map);

                if let (Expression::Simple(a), Expression::Simple(b)) = (&a,&b) {
                    match op {
                        Operation::Add => Expression::Simple(a + b),
                        Operation::Subtract => Expression::Simple(a - b),
                        Operation::Multiply => Expression::Simple(a * b),
                        Operation::Divide => Expression::Simple(a / b),
                        Operation::Equality => unreachable!("Only appears in part 2 where at least one of the sides will not be simple")
                    }
                } else {
                    Expression::Complex(Box::new(a), *op, Box::new(b))
                }
            },
            Action::Human => Expression::Human,
        }
    }
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = isize;
    type Output2 = isize;

    fn day() -> u8 {
        21
    }

    fn part1(input: &str) -> Self::Output1 {
        let map = input
            .trim()
            .lines()
            .map(|line| line.split_once(": ").unwrap())
            .map(|(name, action)| {
                let action = if let Ok(number) = action.parse() {
                    Action::Number(Expression::Simple(number))
                } else {
                    let mut split = action.split(" ");
                    Action::Result(split.next().unwrap().to_owned(), Operation::from_str(split.next().unwrap()).unwrap(), split.next().unwrap().to_owned())
                };

                (name.to_owned(), action)
            }).collect::<HashMap<_, _>>();

        // Get the root node
        let root = map.get("root").unwrap();

        // Build an expression tree
        let expression = root.resolve(&map);

        // Evaluate the expression
        expression.evaluate().unwrap()
    }

    fn part2(input: &str) -> Self::Output2 {
        let map = input
            .trim()
            .lines()
            .map(|line| line.split_once(": ").unwrap())
            .map(|(name, action)| {
                let action = if name == "humn" {
                    Action::Human
                } else if let Ok(number) = action.parse() {
                    Action::Number(Expression::Simple(number))
                } else {
                    let mut split = action.split(" ");
                    let a = split.next().unwrap().to_owned();
                    let mut op = Operation::from_str(split.next().unwrap()).unwrap();
                    let b = split.next().unwrap().to_owned();

                    if name == "root" {
                        op = Operation::Equality;
                    }

                    Action::Result(a, op, b)
                };

                (name.to_owned(), action)
            }).collect::<HashMap<_, _>>();

        // Get the root node
        let root = map.get("root").unwrap();

        // Build an expression tree
        let expression = root.resolve(&map);

        // Invert the expresion tree to get an expression that gives the value for human and then
        // evaluate that expression
        expression.invert().evaluate().unwrap()
    }
}
