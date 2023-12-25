#![feature(test)]
#![feature(iter_map_windows)]
use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::Result;
use aoc::Solver;
use petgraph::{
    algo::{condensation, has_path_connecting},
    graphmap::{GraphMap, UnGraphMap},
    visit::IntoNodeReferences,
    Undirected,
};

// -- Runners --
fn main() -> Result<()> {
    Day::solve()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() -> Result<()> {
        Day::test(Day::part1, "test-1", 54)
    }

    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 552695)
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

// Totally copied this from: https://github.com/Zemogus/AOC-2023/blob/328dc6618f3a360c3d3851ad1b10513a6c133336/src/day25.rs
// For some reason the graph library has no dijkstra that returns the actual path
fn find_shortest_path<'a>(
    graph: &GraphMap<&'a str, (), Undirected>,
    start: &'a str,
    end: &'a str,
) -> Option<Vec<&'a str>> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut parents = HashMap::new();

    queue.push_back(start);
    while let Some(node) = queue.pop_front() {
        // Already visited this node
        if !visited.insert(node) {
            continue;
        }

        // Reached the destination
        if node == end {
            break;
        }

        for neighbour in graph.neighbors(node) {
            if !visited.contains(neighbour) {
                parents.insert(neighbour, node);
                queue.push_back(neighbour);
            }
        }
    }

    let mut path = Vec::new();
    let mut node = end;
    while node != start {
        path.push(node);
        if let Some(parent) = parents.get(&node) {
            node = parent;
        } else {
            return None;
        }
    }

    path.push(start);

    Some(path)
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = usize;
    type Output2 = usize;

    fn day() -> u8 {
        25
    }

    fn part1(input: &str) -> Self::Output1 {
        // Create a list of all edges
        let edges: Vec<_> = input
            .lines()
            .flat_map(|line| {
                let (a, rest) = line.split_once(": ").unwrap();
                rest.split(' ').map(|b| (a, b)).collect::<Vec<_>>()
            })
            .collect();

        // Create a graph from all the edges
        let graph = UnGraphMap::<_, ()>::from_edges(edges);

        // Take a node as the starting point
        let start = graph.nodes().next().unwrap();
        // Loop over all other nodes
        for end in graph.nodes() {
            // Make a copy of the graph so we can modify it and undo changes later
            let mut graph = graph.clone();
            if start == end {
                continue;
            }

            // If the two nodes are on the same side there should be more then three paths
            // connecting the nodes together
            // At least I think???
            // This solution worked, so ¯\_(ツ)_/¯
            for _ in 0..3 {
                // Find the current shortest path
                let path = find_shortest_path(&graph, start, end).unwrap();

                // Remove the path
                for slice in path.windows(2) {
                    match slice {
                        [a, b] => graph.remove_edge(a, b),
                        _ => unreachable!(
                            "There should be three paths connecting all the nodes together"
                        ),
                    };
                }
            }

            // If there is no path connecting the two nodes we have removed the three edges
            // connecting the two halves
            if !has_path_connecting(&graph, start, end, None) {
                // Condense the graph, creates a new graph where each node contains all nodes that
                // where connected in the input node
                let condensed = condensation(graph.into_graph::<usize>(), false);

                // The should give us two nodes each containing all the nodes in their respective
                // half if we split the graph
                if condensed.node_count() != 2 {
                    continue;
                }

                // Multiply the size of each of the halves together giving the final solution
                return condensed
                    .node_references()
                    .fold(1, |acc, (_, nodes)| acc * nodes.len());
            }
        }

        unreachable!("No solution found");
    }

    fn part2(_input: &str) -> Self::Output2 {
        0
    }
}
