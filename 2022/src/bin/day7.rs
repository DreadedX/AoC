#![feature(test)]
use anyhow::Result;
use aoc::Solver;
use implementation::Node;

// -- Runners --
fn main() -> Result<()> {
    Day::solve()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() -> Result<()> {
        Day::test(Day::part1, "test-1", 95437)
    }
    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 2031851)
    }
    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(Day::part2, "test-1", 24933642)
    }
    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", 2568781)
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

mod implementation {
    use std::collections::HashMap;

    pub struct Node {
        size: u32,
        children: HashMap<String, Self>,
        parent: *mut Self,
    }

    impl Node {
        pub fn default() -> Self {
            Self {
                size: 0,
                children: HashMap::new(),
                parent: std::ptr::null_mut(),
            }
        }

        pub fn new(input: &str) -> Self {
            let mut root = Node::default();
            let mut current = &mut root as *mut Node;

            input
                .lines()
                .map(|line| line.rsplit_once(" ").unwrap())
                .for_each(|split| {
                    match split {
                        ("$ cd", "/") => current = &mut root as *mut Node,
                        ("$ cd", "..") => unsafe {
                            let parent = (*current).parent;
                            if parent == std::ptr::null_mut() {
                                panic!("Node has no parent")
                            }
                            current = parent;
                        },
                        ("$ cd", name) => unsafe {
                            current = (*current).get_directory(name).unwrap();
                        },
                        ("$", "ls") => {},
                        ("dir", name) => unsafe {
                            (*current).add_directory(name.to_owned())
                        },
                        (size, _name) => unsafe {
                            (*current).add_file(size.parse().unwrap())
                        },
                    }
                });

            root
        }

        fn add_file(&mut self, size: u32) {
            self.size += size;
        }

        fn add_directory(&mut self, name: String) {
            let mut child = Self::default();
            child.parent = self as *mut Self;
            self.children.insert(name, child);
        }

        fn get_directory(&mut self, name: &str) -> Option<&mut Self> {
            self.children.get_mut(name)
        }

        pub fn get_size(&self) -> u32 {
            let mut size = self.size;

            for (_, node) in &self.children {
                size += node.get_size();
            }

            size
        }

        pub fn flatten_sizes(&self) -> Vec<u32> {
            let mut dirs = vec![self.get_size()];

            for (_, node) in &self.children {
                dirs.extend(node.flatten_sizes())
            }

            dirs
        }
    }
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = u32;
    type Output2 = u32;
    fn day() -> u8 {
        7
    }

    fn part1(input: &str) -> Self::Output1 {
        Node::new(input)
            .flatten_sizes()
            .iter()
            .filter(|&&size| size < 100000)
            .sum()
    }

    fn part2(input: &str) -> Self::Output2 {
        let root = Node::new(input);
        let need_to_free = root.get_size() - 40000000;

        let mut sizes = root.flatten_sizes();
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
