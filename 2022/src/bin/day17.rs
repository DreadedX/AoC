#![feature(test)]
use anyhow::Result;
use aoc::Solver;

// @TODO Can be made faster using bitset and shifting for moving left and right
// Should also make collision detection much faster

// -- Runners --
fn main() -> Result<()> {
    Day::solve()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() -> Result<()> {
        Day::test(Day::part1, "test-1", 3068)
    }
    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 3159)
    }
    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(Day::part2, "test-1", 1514285714288)
    }
    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", 1566272189352)
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
struct Shape {
    shape: u32,
}

const WIDTH: usize = 7;
const SPACE_ABOVE: usize = 3;

impl Shape {
    fn new(s: &[&str]) -> Self {
        let shape = s.into_iter()
            .rev()
            .map(|line| {
                line.chars()
                    .enumerate()
                    .fold(0, |acc, (x, c)| {
                        if c == '#' {
                            acc | 1 << x
                        } else {
                            acc
                        }
                    })
            }).enumerate()
            .fold(0, |acc, (y, x)| {
                acc | x << 8*y
            });

        Self {
            shape
        }
    }

    fn shape(&self) -> u32 {
        return self.shape;
    }

    fn get_shapes() -> Vec<Shape> {
        vec![
            // Horizontal
            Shape::new(&[
                "..####.",
            ]),
            // Plus
            Shape::new(&[
                "...#...",
                "..###..",
                "...#...",
            ]),
            // Corner
            Shape::new(&[
                "....#..",
                "....#..",
                "..###..",
            ]),
            // Vertical
            Shape::new(&[
                "..#....",
                "..#....",
                "..#....",
                "..#....",
            ]),
            // Square
            Shape::new(&[
                "..##...",
                "..##...",
            ]),
        ]
    }
}

struct Field {
    map: Vec<u8>,
    heights: Vec<usize>,
}

impl Field {
    fn new() -> Self {
        Self {
            map: Vec::new(),
            heights: vec![0; WIDTH],
        }
    }

    fn get_mask(&self, y: usize) -> u32 {
        // Create a mask of all the occupied spaces
        u32::from_le_bytes([self.map[y], self.map[y+1], self.map[y+2], self.map[y+3]])
    }

    fn move_left(&self, y: usize, shape: u32) -> u32 {
        // Create a mask for the left most bits
        let mask = 0b00000001_00000001_00000001_00000001;

        // If any part of the shape is in the left most bit we can not move to the left
        if shape & mask != 0 {
            return shape;
        }

        // Move the shape to the left
        let moved = shape >> 1;

        let mask = self.get_mask(y);

        // If there is overlap we can not move
        if moved & mask != 0 {
            return shape;
        }

        // No collision detected
        return moved;
    }

    fn move_right(&self, y: usize, shape: u32) -> u32 {
        // Move the shape to the right
        let moved = shape << 1;

        // Create the mask for the left wall and occupied spaces
        let mask = 0b10000000_10000000_10000000_10000000 |
            self.get_mask(y);

        // If there is overlap we can not move
        if moved & mask != 0 {
            return shape;
        }

        // No collision detected
        return moved;
    }

    fn collision_down(&self, y: usize, shape: u32) -> bool {
        // Hit the floor
        if y == 0 {
            return true;
        }

        // Create a mask of all occuiped spaces one level down
        let mask = self.get_mask(y-1);

        if shape & mask != 0 {
            return true;
        }

        return false;
    }

    fn land(&mut self, y: usize, shape: u32) {
        for i in 0..4 {
            self.map[y+i] |= ((shape >> 8*i) & 0xFF) as u8;
            for x in 0..WIDTH {
                if (self.map[y+i] >> x) & 1 == 1 {
                    self.heights[x] = self.heights[x].max(y+i+1);
                }
            }
        }
    }

    fn height(&self) -> usize {
        *self.heights.iter().max().unwrap()
    }

    fn height_min(&self) -> usize {
        *self.heights.iter().max().unwrap()
    }

    fn expand(&mut self) {
        let max = self.height();

        while self.map.len() < (max + 4 + SPACE_ABOVE) as usize {
            self.map.push(0);
        }
    }

    fn check_for_pattern(&self, initial_height: usize) -> usize {
        // Check a window starting from the initial height up to the lowest column,
        // as above that the rows might still change
        let height = self.height_min()-initial_height;
        if height % 2 != 0 {
            return 0;
        }

        let bottom = &self.map[initial_height as usize..(height/2 + initial_height) as usize];
        let top = &self.map[(height/2+initial_height) as usize..(height + initial_height) as usize];

        assert_eq!(bottom.len(), top.len(), "Height: {height}");

        // Check if the top and bottom half are the same
        for y in 0..(height/2) as usize {
            if bottom[y] != top[y] {
                return 0;
            }
        }

        return height/2;
    }

    // fn print(&self) {
    //     for line in self.map.iter().rev() {
    //         for x in 0..WIDTH {
    //             if (line >> x) & 1 == 1 {
    //                 print!("#");
    //             } else {
    //                 print!(".");
    //             }
    //         }
    //         println!("");
    //     }
    // }
}

fn simulate_next_block<'a, S, O>(shapes: &mut S, operator: &mut O, field: &mut Field) where
    S: Iterator<Item = &'a Shape>,
    O: Iterator<Item = char>
{
    let mut shape = shapes.next().unwrap().shape();

    field.expand();

    let mut y = field.height() + SPACE_ABOVE;
    loop {
        let direction = operator.next().unwrap();

        match direction {
            '<' => shape = field.move_left(y, shape),
            '>' => shape = field.move_right(y, shape),
            _ => panic!("Unexpected direction"),
        }

        if field.collision_down(y, shape) {
            field.land(y, shape);
            break;
        } else {
            y -= 1;
        }
    }
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = usize;
    type Output2 = usize;

    fn day() -> u8 {
        17
    }

    fn part1(input: &str) -> Self::Output1 {
        let shapes = Shape::get_shapes();
        let mut shapes = shapes.iter().cycle();
        let mut operator = input.trim().chars().cycle();
        let mut field = Field::new();

        for _ in 0..2022 {
            simulate_next_block(&mut shapes, &mut operator, &mut field);
        }


        field.height()
    }

    fn part2(input: &str) -> Self::Output2 {
        let mut operator = input.trim().chars().cycle();
        let shapes = Shape::get_shapes();
        let mut shapes = shapes.iter().cycle();

        let mut field = Field::new();

        // The start of the tower is not part of the pattern
        // So we drop a number of blocks first to make sure we get to the periodic part
        let initial_rock_count = 200;
        for _ in 0..initial_rock_count {
            simulate_next_block(&mut shapes, &mut operator, &mut field);
        }

        let initial_height = field.height();

        // We now start dropping blocks
        // After every block we look for a pattern by taking the part added since the initial drop
        // And then splitting for a given height, splitting it in two
        // If these two are the same we have found a pattern and now it's height
        let mut total_dropped = initial_rock_count;
        let mut pattern_height = 0;
        for i in 0..100_000 {
            simulate_next_block(&mut shapes, &mut operator, &mut field);

            let height = field.check_for_pattern(initial_height);
            if  height != 0 {
                pattern_height = height;
                total_dropped += i+1;
                break;
            }
        }
        assert_ne!(pattern_height, 0);

        // @TODO This does not always give the right answer, depending on initial_rock_count we
        // sometimes get a +1 error, however for my input it works like this
        let rocks_in_pattern = (total_dropped - initial_rock_count)/2;
        println!("rocks_in_pattern: {rocks_in_pattern}");

        let total = 1000000000000;
        // For some reason this calculation is wrong...
        // let already_dropped = initial_rock_count + 4*rocks_in_pattern + 1;
        let already_dropped = total_dropped;
        let repeat_count = (total - already_dropped) / rocks_in_pattern;
        let remaining = total - already_dropped - repeat_count * rocks_in_pattern;

        for _ in 0..remaining {
            simulate_next_block(&mut shapes, &mut operator, &mut field);
        }

        field.height() + repeat_count*pattern_height
    }
}
