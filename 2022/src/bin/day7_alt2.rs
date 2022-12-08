#![feature(test)]
use anyhow::Result;
use aoc::Solver;
use implementation::Tree;

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

mod implementation {
    use std::{collections::HashMap, cell::RefCell, rc::{Weak, Rc}};

    struct Node {
        size: u32,
        children: HashMap<String, Rc<RefCell<Self>>>,
        parent: Weak<RefCell<Self>>,
    }

    impl Node {
        fn default() -> Self {
            Self {
                size: 0,
                children: HashMap::new(),
                parent: Weak::new(),
            }
        }

        fn add_file(&mut self, size: u32) {
            self.size += size;
        }

        fn add_directory(&mut self, name: String, parent: Weak<RefCell<Self>>) {
            let child = Rc::new(RefCell::new(Self::default()));
            child.borrow_mut().parent = parent;
            self.children.insert(name, child);
        }

        fn get_directory(&mut self, name: &str) -> Option<Rc<RefCell<Self>>> {
            self.children.get(name).cloned()
        }

        fn get_size(&self) -> u32 {
            let mut size = self.size;

            for (_, node) in &self.children {
                size += node.borrow().get_size();
            }

            size
        }

        fn flatten_sizes(&self) -> Vec<u32> {
            let mut dirs = vec![self.get_size()];

            for (_, node) in &self.children {
                dirs.extend(node.borrow().flatten_sizes())
            }

            dirs
        }
    }

    pub struct Tree {
        root: Rc<RefCell<Node>>
    }

    impl Tree {
        pub fn new(input: &str) -> Self {
            let tree = Self {
                root: Rc::new(RefCell::new(Node::default()))
            };
            let mut current = Rc::downgrade(&tree.root);

            input
                .lines()
                .map(|line| line.rsplit_once(" ").unwrap())
                .for_each(|split| {
                    match split {
                        ("$ cd", "/") => current = Rc::downgrade(&tree.root),
                        ("$ cd", "..") => {
                            current = current.upgrade().unwrap().borrow().parent.clone();
                        },
                        ("$ cd", name) => {
                            current = Rc::downgrade(&current.upgrade().unwrap().borrow_mut().get_directory(name).unwrap())
                        },
                        ("$", "ls") => {},
                        ("dir", name) => {
                            current.upgrade().unwrap().borrow_mut().add_directory(name.to_owned(), current.clone());
                        },
                        (size, _name) => {
                            current.upgrade().unwrap().borrow_mut().add_file(size.parse().unwrap())
                        },
                    }
                });

            tree
        }

        pub fn get_size(&self) -> u32 {
            self.root.borrow().get_size()
        }

        pub fn flatten_sizes(&self) -> Vec<u32> {
            self.root.borrow().flatten_sizes()
        }
    }
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output = u32;
    fn day() -> u8 {
        7
    }

    fn part1(input: &str) -> Self::Output {
        Tree::new(input)
            .flatten_sizes()
            .iter()
            .filter(|&&size| size < 100000)
            .sum()
    }

    fn part2(input: &str) -> Self::Output {
        let tree = Tree::new(input);
        let need_to_free = tree.get_size() - 40000000;

        let mut sizes = tree.flatten_sizes();
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
