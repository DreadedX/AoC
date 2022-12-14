#![feature(test)]
use core::fmt;
use std::{str::FromStr, cmp::{min, max}, borrow::Cow, fs::File};

use anyhow::Result;
use aoc::Solver;
use gif::{Encoder, Repeat, Frame};

// -- Runners --
fn main() -> Result<()> {
    Day::solve()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() -> Result<()> {
        Day::test(Day::part1, "test-1", 24)
    }
    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 1406)
    }
    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(Day::part2, "test-1", 93)
    }
    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", 20870)
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
struct Point {
    x: usize,
    y: usize,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(",").unwrap();
        let x = x.parse()?;
        let y = y.parse()?;

        Ok(Point{x, y})
    }
}

#[derive(Debug)]
struct Path (Vec<Point>);

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (idx, point) in self.0.iter().enumerate() {
            write!(f, "{}", point)?;
            if idx+1 != self.0.len() {
                write!(f, " -> ")?;
            }
        }

        Ok(())
    }
}

impl FromStr for Path {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points = s
            .split(" -> ")
            .flat_map(Point::from_str)
            .collect();

        Ok(Path(points))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Block {
    Air,
    Rock,
    Sand,
    Source,
    Void,
}

struct Cave {
    grid: Vec<Vec<Block>>,
    source: usize,
    size: (usize, usize),
}

impl<'a> Cave {
    // Also returns the width and height of the cave
    fn new(mut paths: Vec<Path>, floor: bool) -> Self {
        // The sand source is at 500, 0
        // This means that xmax starts at 500 and we know ymin is 0
        let mut source = 500;
        let (mut xmin, mut xmax, mut ymax) = (usize::MAX, source, 0);

        for path in paths.iter() {
            for point in path.0.iter() {
                xmin = min(xmin, point.x);
                xmax = max(xmax, point.x);
                ymax = max(ymax, point.y);
            }
        }


        if floor {
            ymax += 2;
            xmin = min(xmin, source - ymax);
            xmax = max(xmax, source + ymax);

            let path = Path(vec![Point{x: xmin, y: ymax}, Point{x: xmax, y: ymax}]);
            paths.push(path);
        }

        for path in paths.iter_mut() {
            for point in path.0.iter_mut() {
                point.x -= xmin;
            }
        }

        let width = xmax-xmin+1;
        let height = ymax+1;

        let mut grid = vec![vec![Block::Air; width]; height];

        for path in paths {
            for line in path.0.windows(2) {
                let ystart = min(line[0].y, line[1].y);
                let yend = max(line[0].y, line[1].y)+1;
                for y in ystart..yend {
                    let xstart = min(line[0].x, line[1].x);
                    let xend = max(line[0].x, line[1].x)+1;
                    for x in xstart..xend {
                        grid[y][x] = Block::Rock;
                    }
                }
            }
        }

        source -= xmin;
        grid[0][source] = Block::Source;

        Self {
            grid,
            source,
            size: (xmax-xmin+1, ymax+1)
        }
    }

    fn from_str(s: &str, floor: bool) -> Self {
        let paths = s
            .lines()
            .flat_map(Path::from_str)
            .collect();

        Self::new(paths, floor)
    }

    fn check(&self, x: isize, y: isize) -> Block {
        if x < 0 || y < 0 || x >= self.size.0 as isize || y >= self.size.1 as isize {
            return Block::Void;
        }
        return self.grid[y as usize][x as usize];
    }

    fn simulate_sand(&mut self) -> bool {
        let (mut x, mut y): (isize, isize) = (self.source as isize, -1);

        loop {
            let positions = vec![
                (x, y+1),
                (x-1, y+1),
                (x+1, y+1),
            ];

            let mut moved = false;
            for position in positions {
                match self.check(position.0, position.1) {
                    // The sand can move into the position
                    Block::Air | Block::Source => {
                        // Update the coordinates
                        x = position.0;
                        y = position.1;

                        // Mark that the sand has moved
                        moved = true;

                        // Simulate the next step
                        break;
                    },
                    // This position is occupied, check the next position
                    Block::Rock | Block::Sand => continue,
                    // The sand fell into the void, any sand we add will also fall into the void
                    // Return false to signify this
                    Block::Void => return false,
                }
            }

            // If the sand did not move, the sand has settled and we can return
            if !moved {
                let block = self.grid[y as usize][x as usize];
                self.grid[y as usize][x as usize] = Block::Sand;
                // If we replaced the source block we are also done
                return block != Block::Source;
            }
        }
    }

    fn print(&self) {
        for line in self.grid.iter() {
            for block in line {
                let c = match block {
                    Block::Air => '.',
                    Block::Rock => '#',
                    Block::Sand => 'o',
                    Block::Source => '+',
                    Block::Void => 'x',
                };

                print!("{}", c);
            }
            println!("");
        }
        println!("");
    }

    fn frame(&self) -> Cow<[u8]> {
        let buffer = self.grid.iter().flat_map(|line| line.iter().map(|block| match block {
                    Block::Air => 0,
                    Block::Rock => 1,
                    Block::Sand => 2,
                    Block::Source => 3,
                    Block::Void => 4,
        }).collect::<Vec<_>>()).collect::<Vec<u8>>();

        Cow::from(buffer)
    }

    fn count_sand(&self) -> usize {
        self.grid.iter().flatten().filter(|&block| *block == Block::Sand).count()
    }
}

fn write_frame<T: std::io::Write>(encoder: &mut Encoder<T>, cave: &Cave, delay: u16) {
    let mut frame = Frame::default();
    frame.width = cave.size.0 as u16;
    frame.height = cave.size.1 as u16;
    frame.buffer = cave.frame();
    frame.delay = delay;
    encoder.write_frame(&frame).unwrap();
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = usize;
    type Output2 = usize;

    fn day() -> u8 {
        14
    }

    fn part1(input: &str) -> Self::Output1 {
        let mut cave = Cave::from_str(input, false);

        // let color_map = &[0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0xea, 0xc7, 0x99, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF];
        // let mut image = File::create("visualize/14/part1.gif").unwrap();
        // let mut encoder = Encoder::new(&mut image, cave.size.0 as u16, cave.size.1 as u16, color_map).unwrap();
        // encoder.set_repeat(Repeat::Infinite).unwrap();

        // write_frame(&mut encoder, &mut cave, 100);

        while cave.simulate_sand() {
            // write_frame(&mut encoder, &mut cave, 1);
        }

        // write_frame(&mut encoder, &mut cave, 1000);

        cave.count_sand()
    }

    fn part2(input: &str) -> Self::Output2 {
        let mut cave = Cave::from_str(input, true);

        // let color_map = &[0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0xea, 0xc7, 0x99, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF];
        // let mut image = File::create("visualize/14/part2.gif").unwrap();
        // let mut encoder = Encoder::new(&mut image, cave.size.0 as u16, cave.size.1 as u16, color_map).unwrap();
        // encoder.set_repeat(Repeat::Infinite).unwrap();

        // write_frame(&mut encoder, &mut cave, 100);

        while cave.simulate_sand() {
            // write_frame(&mut encoder, &mut cave, 1);
        }

        // write_frame(&mut encoder, &mut cave, 1000);

        cave.count_sand()
    }
}
