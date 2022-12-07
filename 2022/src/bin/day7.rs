#![feature(test)]
use core::fmt;
use std::{collections::HashMap, rc::Rc, cell::RefCell};

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
        Day::test(aoc::Part::ONE, "test-1", 95437)
    }
    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(aoc::Part::ONE, "input", 2031851)
    }
    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(aoc::Part::TWO, "test-1", 24933642)
    }
    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(aoc::Part::TWO, "input", 2568781)
    }

    // Benchmarks
    extern crate test;
    #[bench]
    #[ignore]
    fn part1_bench(b: &mut test::Bencher) {
        Day::benchmark(aoc::Part::ONE, b)
    }
    #[bench]
    #[ignore]
    fn part2_bench(b: &mut test::Bencher) {
        Day::benchmark(aoc::Part::TWO, b)
    }
}

// -- Implentation --
#[derive(Default)]
struct Node {
    // Size of all files in this directory
    file_size: u32,
    nodes: HashMap<String, Rc<RefCell<Node>>>,
    parent: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn add_file(&mut self, size: u32) {
        self.file_size += size
    }

    fn add_directory(&mut self, name: String, node: Rc<RefCell<Node>>) {
        self.nodes.insert(name, node);
    }

    fn get_directory(&self, name: &str) -> Option<Rc<RefCell<Node>>> {
        match self.nodes.get(name) {
            Some(node) => Some(Rc::clone(node)),
            None => None,
        }
    }

    fn get_parent(&mut self) -> Option<Rc<RefCell<Node>>> {
        match &self.parent {
            Some(parent) => Some(Rc::clone(parent)),
            None => None
        }
    }

    fn get_size(&self) -> u32 {
        let mut size = self.file_size;

        for (_, node) in &self.nodes {
            size += node.borrow_mut().get_size()
        }

        size
    }

    fn get_sizes_flat(&self) -> Vec<u32> {
        let mut dirs = vec![self.get_size()];

        for (_, node) in &self.nodes {
            dirs.extend(node.borrow_mut().get_sizes_flat())
        }

        dirs
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n", self.get_size())?;
        for (name, node) in &self.nodes {
            write!(f, "{}: {}", name, node.borrow_mut())?;
        }

        Ok(())
    }
}

// -- Helpers --
fn parse(input: &str) -> Rc<RefCell<Node>> {
    let root = Rc::new(RefCell::new(Node::default()));
    let mut current = Rc::clone(&root);

    input
        .lines()
        .flat_map(|line| line.split_once(" "))
        .for_each(|(typ, rest)| {
            match typ {
                "$" => {
                    // There are two cmds: ls, cd ${dir}, we only care about dir
                    if let Some((_, dir)) = rest.split_once(" ") {
                        match dir {
                            "/" => current = {
                                Rc::clone(&root)
                            },
                            ".." => {
                                let parent = current.borrow_mut().get_parent().unwrap();
                                current = parent;
                            }
                            name => {
                                let dir = current.borrow_mut().get_directory(name).unwrap();
                                current = dir;
                            },
                        };
                    }
                },
                "dir" => {
                    let dir = Rc::new(RefCell::new(Node::default()));
                    dir.borrow_mut().parent = Some(Rc::clone(&current));
                    current.borrow_mut().add_directory(rest.to_owned(), Rc::clone(&dir));
                },
                size => {
                    current.borrow_mut().add_file(size.parse().unwrap())
                },

            }
        });

    root
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output = u32;
    fn day() -> u8 {
        7
    }

    fn part1(input: &str) -> Self::Output {
        let root = parse(input);

        let sizes = root.borrow_mut().get_sizes_flat();
        sizes.iter()
            .filter(|&&size| size < 100000)
            .sum()
    }

    fn part2(input: &str) -> Self::Output {
        let root = parse(input);

        let mut sizes = root.borrow_mut().get_sizes_flat();

        let need_to_free = sizes[0] - 40000000;

        sizes.sort();
        sizes.iter()
            .find_map(|&size| {
            if size > need_to_free {
                Some(size)
            } else {
                None
            }
        }).unwrap()
    }
}
