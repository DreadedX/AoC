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

#[derive(PartialEq, Eq)]
enum ShapeName {
    Horizontal,
    Plus,
    Corner,
    Vertical,
    Square,
}

struct Position {
    x: i64,
    y: i64,
}

trait Shape {
    fn height(&self) -> i64;
    fn can_move_left(&self, origin: &Position, field: &Field) -> bool;
    fn can_move_right(&self, origin: &Position, field: &Field) -> bool;
    fn can_move_down(&self, origin: &Position, field: &Field) -> bool;

    fn land(&self, origin: &Position, field: &mut Field);

    fn shape(&self) -> ShapeName;
}

// ####
struct Horizontal;
impl Shape for Horizontal {
    fn height(&self) -> i64 {
        1
    }

    fn can_move_left(&self, origin: &Position, field: &Field) -> bool {
        field.is_open(origin.x-1, origin.y)
    }

    fn can_move_right(&self, origin: &Position, field: &Field) -> bool {
        field.is_open(origin.x+4, origin.y)
    }

    fn can_move_down(&self, origin: &Position, field: &Field) -> bool {
        field.is_open(origin.x, origin.y-1) &&
            field.is_open(origin.x+1, origin.y-1) &&
            field.is_open(origin.x+2, origin.y-1) &&
            field.is_open(origin.x+3, origin.y-1)
    }

    fn land(&self, origin: &Position, field: &mut Field) {
        field.set(origin.x+3, origin.y);
        field.set(origin.x+2, origin.y);
        field.set(origin.x+1, origin.y);
        field.set(origin.x, origin.y);
    }

    fn shape(&self) -> ShapeName {
        ShapeName::Horizontal
    }
}

// .#.
// ###
// .#.
struct Plus;
impl Shape for Plus {
    fn height(&self) -> i64 {
        3
    }

    fn can_move_left(&self, origin: &Position, field: &Field) -> bool {
        field.is_open(origin.x, origin.y+2) &&
            field.is_open(origin.x-1, origin.y+1) &&
            field.is_open(origin.x, origin.y)

    }

    fn can_move_right(&self, origin: &Position, field: &Field) -> bool {
        field.is_open(origin.x+2, origin.y+2) &&
            field.is_open(origin.x+3, origin.y+1) &&
            field.is_open(origin.x+2, origin.y)

    }

    fn can_move_down(&self, origin: &Position, field: &Field) -> bool {
        field.is_open(origin.x, origin.y) &&
            field.is_open(origin.x+1, origin.y-1) &&
            field.is_open(origin.x+2, origin.y)
    }

    fn land(&self, origin: &Position, field: &mut Field) {
        field.set(origin.x+1, origin.y+2);
        field.set(origin.x, origin.y+1);
        field.set(origin.x+1, origin.y+1);
        field.set(origin.x+2, origin.y+1);
        field.set(origin.x+1, origin.y);
    }

    fn shape(&self) -> ShapeName {
        ShapeName::Plus
    }
}

// ..#
// ..#
// ###
struct Corner;
impl Shape for Corner {
    fn height(&self) -> i64 {
        3
    }

    fn can_move_left(&self, origin: &Position, field: &Field) -> bool {
        field.is_open(origin.x+1, origin.y+2) &&
            field.is_open(origin.x+1, origin.y+1) &&
            field.is_open(origin.x-1, origin.y)
    }

    fn can_move_right(&self, origin: &Position, field: &Field) -> bool {
        field.is_open(origin.x+3, origin.y+2) &&
            field.is_open(origin.x+3, origin.y+1) &&
            field.is_open(origin.x+3, origin.y)
    }

    fn can_move_down(&self, origin: &Position, field: &Field) -> bool {
        field.is_open(origin.x, origin.y-1) &&
            field.is_open(origin.x+1, origin.y-1) &&
            field.is_open(origin.x+2, origin.y-1)
    }

    fn land(&self, origin: &Position, field: &mut Field) {
        field.set(origin.x+2, origin.y+2);
        field.set(origin.x+2, origin.y+1);
        field.set(origin.x, origin.y);
        field.set(origin.x+1, origin.y);
        field.set(origin.x+2, origin.y);
    }

    fn shape(&self) -> ShapeName {
        ShapeName::Corner
    }
}

// #
// #
// #
// #
struct Vertical;
impl Shape for Vertical {
    fn height(&self) -> i64 {
        4
    }

    fn can_move_left(&self, origin: &Position, field: &Field) -> bool {
        field.is_open(origin.x-1, origin.y+3) &&
            field.is_open(origin.x-1, origin.y+2) &&
            field.is_open(origin.x-1, origin.y+1) &&
            field.is_open(origin.x-1, origin.y)
    }

    fn can_move_right(&self, origin: &Position, field: &Field) -> bool {
        field.is_open(origin.x+1, origin.y+3) &&
            field.is_open(origin.x+1, origin.y+2) &&
            field.is_open(origin.x+1, origin.y+1) &&
            field.is_open(origin.x+1, origin.y)
    }

    fn can_move_down(&self, origin: &Position, field: &Field) -> bool {
        field.is_open(origin.x, origin.y-1)
    }

    fn land(&self, origin: &Position, field: &mut Field) {
        field.set(origin.x, origin.y+3);
        field.set(origin.x, origin.y+2);
        field.set(origin.x, origin.y+1);
        field.set(origin.x, origin.y);
    }

    fn shape(&self) -> ShapeName {
        ShapeName::Vertical
    }
}

// ##
// ##
struct Square;
impl Shape for Square {
    fn height(&self) -> i64 {
        2
    }

    fn can_move_left(&self, origin: &Position, field: &Field) -> bool {
        field.is_open(origin.x-1, origin.y+1) &&
            field.is_open(origin.x-1, origin.y)
    }

    fn can_move_right(&self, origin: &Position, field: &Field) -> bool {
        field.is_open(origin.x+2, origin.y+1) &&
            field.is_open(origin.x+2, origin.y)
    }

    fn can_move_down(&self, origin: &Position, field: &Field) -> bool {
        field.is_open(origin.x, origin.y-1) &&
            field.is_open(origin.x+1, origin.y-1)
    }

    fn land(&self, origin: &Position, field: &mut Field) {
        field.set(origin.x, origin.y+1);
        field.set(origin.x+1, origin.y+1);
        field.set(origin.x, origin.y);
        field.set(origin.x+1, origin.y);
    }

    fn shape(&self) -> ShapeName {
        ShapeName::Square
    }
}

struct Field {
    map: Vec<Vec<bool>>,
    heights: Vec<i64>,
}

const WIDTH: i64 = 7;
const SPACE_ABOVE: i64 = 3;

impl Field {
    fn new() -> Self {
        Self {
            map: Vec::new(),
            heights: vec![0; WIDTH as usize],
        }
    }

    fn is_open(&self, x: i64, y: i64) -> bool {
        // Out of bounds check
        // Note that we do not check the upper bound for y as we are only going to move down
        if x < 0 || x >= WIDTH || y < 0 {
            return false;
        }

        // The spot is open if the value in the map is false
        !self.map[y as usize][x as usize]
    }

    fn set(&mut self, x: i64, y: i64) {
        // Update the max height for each column
        self.heights[x as usize] = self.heights[x as usize].max(y+1);
        self.map[y as usize][x as usize] = true;
    }

    fn expand(&mut self, height: i64) {
        let max = self.height();

        while self.map.len() < (max + height + SPACE_ABOVE) as usize {
            self.map.push(vec![false; WIDTH as usize]);
        }
    }

    fn height(&self) -> i64 {
        *self.heights.iter().max().unwrap()
    }

    fn check_for_pattern(&self, initial_height: i64) -> i64 {
        // Skip the top lines as they might still might change
        'outer: for height in 10..(self.height()-initial_height) {
            if height % 2 != 0 {
                continue 'outer;
            }

            let bottom = &self.map[initial_height as usize..(height/2 + initial_height) as usize];
            let top = &self.map[(height/2+initial_height) as usize..(height + initial_height) as usize];

            assert_eq!(bottom.len(), top.len(), "Height: {height}");

            for y in 0..(height/2) as usize {
                for x in 0..WIDTH as usize {
                    if bottom[y][x] != top[y][x] {
                        continue 'outer;
                    }
                }
            }

            return height/2;
        }

        return 0;
    }
}

fn simulate_next_block<'a, S, O>(shapes: &mut S, operator: &mut O, field: &mut Field) where
    S: Iterator<Item = &'a Box<dyn Shape>>,
    O: Iterator<Item = char>
{
    // Get the next shape
    let shape = shapes.next().unwrap();
 
    // Expand the field upwards
    field.expand(shape.height());

    // Set the origin to two from the side and three above the highest rock
    let mut origin = Position{ x: 2, y: field.height() as i64 + SPACE_ABOVE as i64 };
    loop {
        // First check left/right movement
        let direction = operator.next().unwrap();

        if direction == '<' && shape.can_move_left(&origin, &field) {
            origin.x -= 1;
        } else if direction == '>' && shape.can_move_right(&origin, &field) {
            origin.x += 1;
        } else {
            // Unable to move left or right
        }

        if shape.can_move_down(&origin, &field) {
            origin.y -= 1;
        } else {
            // We are done and can land
            break;
        }
    }

    shape.land(&origin, field);
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = i64;
    type Output2 = i64;

    fn day() -> u8 {
        17
    }

    fn part1(input: &str) -> Self::Output1 {
        let mut operator = input.trim().chars().cycle();
        let shapes: Vec<Box<dyn Shape>> = vec![Box::new(Horizontal{}), Box::new(Plus{}), Box::new(Corner{}), Box::new(Vertical{}), Box::new(Square{})];
        let mut shapes = shapes.iter().cycle();

        let mut field = Field::new();

        for _rock in 0..2022 {
            simulate_next_block(&mut shapes, &mut operator, &mut field);
        }

        field.height()
    }

    fn part2(input: &str) -> Self::Output2 {
        let mut operator = input.trim().chars().cycle();
        let shapes: Vec<Box<dyn Shape>> = vec![Box::new(Horizontal{}), Box::new(Plus{}), Box::new(Corner{}), Box::new(Vertical{}), Box::new(Square{})];
        let mut shapes = shapes.iter().cycle().peekable();

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
        let mut pattern_height: i64 = 0;
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

        // Because of how the pattern finding is implemented, it might require blocks of the next
        // pattern iteration to fall before it is able to find the patterm.
        // In order to get to a known state we keep droppping until we have added the hight of
        // another pattern iteration
        while field.height() <= initial_height + 3*pattern_height {
            simulate_next_block(&mut shapes, &mut operator, &mut field);
            total_dropped += 1;
        }
        // After this we should be in the situation where we have dropped the first block of a
        // pattern, so how we need to drop another pattern until we have once again added the
        // height of a pattern iteration
        let mut rocks_in_pattern = 0;
        while field.height() <= initial_height + 4*pattern_height {
            simulate_next_block(&mut shapes, &mut operator, &mut field);
            rocks_in_pattern += 1;
        }
        total_dropped += rocks_in_pattern;
        // We are once again in the situation where we have dropped the first block in the next
        // pattern

        assert_ne!(rocks_in_pattern, 0);

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
