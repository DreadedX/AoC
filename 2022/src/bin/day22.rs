#![feature(test)]
use core::fmt;

use anyhow::Result;
use aoc::Solver;
use regex::Regex;

// -- Runners --
fn main() -> Result<()> {
    Day::solve()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() -> Result<()> {
        Day::test(Day::part1, "test-1", 6032)
    }
    #[test]
    fn part1_solution() -> Result<()> {
        Day::test(Day::part1, "input", 93226)
    }
    #[test]
    fn part2_test1() -> Result<()> {
        Day::test(Day::part2, "test-1", 5031)
    }
    #[test]
    fn part2_solution() -> Result<()> {
        Day::test(Day::part2, "input", 37415)
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

const PLANES: usize = 4;

type Transform = Box<dyn 'static + Fn(Player) -> Player>;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Tile {
    Void,
    Open,
    Wall,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn rotate(&mut self, clockwise: bool) {
        *self = match self {
            Direction::Right => if clockwise { Direction::Down }  else { Direction::Up },
            Direction::Down => if clockwise { Direction::Left }  else { Direction::Right },
            Direction::Left => if clockwise { Direction::Up }  else { Direction::Down },
            Direction::Up => if clockwise { Direction::Right }  else { Direction::Left },
        }
    }
}

impl From<&Direction> for usize {
    fn from(value: &Direction) -> Self {
        match value {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            ' ' => Tile::Void,
            '.' => Tile::Open,
            '#' => Tile::Wall,
            _ => panic!("Invalid tile input: {value}"),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Tile::Void => '~',
            Tile::Open => '.',
            Tile::Wall => '#',
        })
    }
}

#[derive(Debug, Copy, Clone)]
struct Vec2 {
    x: usize,
    y: usize,
}

impl Vec2 {
    fn new(x: usize, y: usize) -> Self { 
        Self { x, y }
    }
}

#[derive(Copy, Clone)]
struct Player {
    plane: Vec2,
    position: Vec2,
    direction: Direction,
}

struct Plane {
    grid: Vec<Vec<Tile>>,
    neighbours: Vec<Option<Transform>>,
}

impl Plane {
    fn new(size: usize) -> Plane {
        let grid = vec![vec![Tile::Void; size]; size];

        let mut neighbours = Vec::with_capacity(4);
        for _ in 0..4 {
            neighbours.push(None);
        }

        Plane { grid, neighbours }
    }
}

struct Map {
    planes: Vec<Vec<Option<Plane>>>,
    player: Player,
    size: usize,
}

impl Map {
    fn new(input: &str, size: usize) -> Self {
        let mut planes = Vec::with_capacity(PLANES);
        for _ in 0..PLANES {
            let mut temp = Vec::with_capacity(PLANES);
            for _ in 0..PLANES {
                temp.push(None);
            }
            planes.push(temp);
        }

        input
            .lines()
            .enumerate()
            .take_while(|(_, line)| !line.is_empty())
            .for_each(|(y, line)| {
                line
                    .chars()
                    .enumerate()
                    .for_each(|(x, c)| {
                        let tile = Tile::from(c);
                        if tile == Tile::Void {
                            return;
                        }

                        let plane = &mut planes[y/size][x/size];
                        if plane.is_none() {
                            *plane = Some(Plane::new(size));
                        }

                        if let Some(plane) = plane {
                            plane.grid[y % size][x % size] = tile;
                        }
                    });
            });

        let x = planes[0].iter().position(|plane| plane.is_some()).unwrap();

        let player = Player {
            plane: Vec2::new(x, 0),
            position: Vec2::new(0, 0),
            direction: Direction::Right,
        };

        Self { planes, player, size }
    }

    fn _print(&self) {
        for y in 0..self.size*PLANES {
            for x in 0..self.size*PLANES {
                if let Some(plane) = &self.planes[y/self.size][x/self.size] {
                    if self.player.plane.x == x/self.size && self.player.plane.y == y/self.size && self.player.position.x == x%self.size && self.player.position.y == y%self.size {
                        print!("$");
                    } else {
                        let tile = &plane.grid[y%self.size][x%self.size];
                        print!("{tile}");
                    }
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
    }
}

impl Map {
    fn movement(&mut self, moves: Vec<(usize, Option<bool>)>) {
        for m in moves {
            self.step(m.0);
            if let Some(clockwise) = m.1 {
                self.player.direction.rotate(clockwise);
            }
        }
    }

    fn get_neighbour(&self, plane: &Vec2, direction: &Direction) -> &Transform {
        // Get the position of the neighbour
        let plane = self.planes[plane.y][plane.x].as_ref().unwrap();
        plane.neighbours[usize::from(direction)].as_ref().unwrap()
    }

    fn step(&mut self, steps: usize) {
        for _ in 0..steps {
            // Make a copy of the player
            let mut np = self.player;

            match self.player.direction {
                Direction::Right => {
                    // Move the new player
                    np.position.x += 1;

                    // We walk out of the current plane
                    if np.position.x >= self.size {
                        // Apply the correct transform to the move the player to the right location
                        let transform = self.get_neighbour(&np.plane, &np.direction);
                        np = transform(np);
                    }
                },
                Direction::Left => {
                    // Move the new player
                    let temp = np.position.x as isize - 1;

                    // We walk out of the current plane
                    if temp < 0 {
                        // Apply the correct transform to the move the player to the right location
                        let transform = self.get_neighbour(&np.plane, &np.direction);
                        np = transform(np);
                    } else {
                        np.position.x -= 1;
                    }

                },
                Direction::Down => {
                    // Move the new player
                    np.position.y += 1;

                    // We walk out of the current plane
                    if np.position.y >= self.size {
                        // Apply the correct transform to the move the player to the right location
                        let transform = self.get_neighbour(&np.plane, &np.direction);
                        np = transform(np);
                    }
                },
                Direction::Up => {
                    // Move the new player
                    let temp = np.position.y as isize - 1;

                    // We walk out of the current plane
                    if temp < 0 {
                        // Apply the correct transform to the move the player to the right location
                        let transform = self.get_neighbour(&np.plane, &np.direction);
                        np = transform(np);
                    } else {
                        np.position.y -= 1;
                    }

                },
            }

            // Get the current plane
            let plane = self.planes[np.plane.y][np.plane.x].as_ref().unwrap();

            // Make sure the new location is free
            if plane.grid[np.position.y][np.position.x] == Tile::Wall {
                // We are done moving
                break;
            }

            // Update the player location
            self.player = np;
        }
    }

    fn score(&self) -> usize {
        1000 * (self.player.plane.y*self.size + self.player.position.y + 1) + 4 * (self.player.plane.x*self.size + self.player.position.x + 1) + (usize::from(&self.player.direction))
    }

    // Connect the planes together according to the rules in part 1
    fn fill_neighbours_part1(&mut self) {
        let size = self.size;
        for y in 0..PLANES {
            for x in 0..PLANES {
                if self.planes[y][x].is_some() {
                    // Check up neighbour
                    {
                        let y_neighbour = (0..PLANES).rev().cycle().skip(PLANES-y).take(PLANES).find(|y| self.planes[*y][x].is_some()).unwrap();
                        self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Up)] = Some(Box::new(move |mut p: Player| {
                            p.plane = Vec2::new(x, y_neighbour);
                            p.position.y = size-1;
                            return p;
                        }));
                    }

                    // Check down neighbour
                    {
                        let y_neighbour = (0..PLANES).cycle().skip(y+1).take(PLANES).find(|y| self.planes[*y][x].is_some()).unwrap();
                        self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Down)] = Some(Box::new(move |mut p: Player| {
                            p.plane = Vec2::new(x, y_neighbour);
                            p.position.y = 0;
                            return p;
                        }));
                    }

                    // Check left neighbour
                    {
                        let x_neighbour = (0..PLANES).rev().cycle().skip(PLANES-x).take(PLANES).find(|x| self.planes[y][*x].is_some()).unwrap();
                        self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Left)] = Some(Box::new(move |mut p: Player| {
                            p.plane = Vec2::new(x_neighbour, y);
                            p.position.x = size-1;
                            return p;
                        }));
                    }

                    // Check right neighbour
                    {
                        let x_neighbour = (0..PLANES).cycle().skip(x+1).take(PLANES).find(|x| self.planes[y][*x].is_some()).unwrap();
                        self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Right)] = Some(Box::new(move |mut p: Player| {
                            p.plane = Vec2::new(x_neighbour, y);
                            p.position.x = 0;
                            return p;
                        }));
                    }
                }
            }
        }
    }

    // Setup the connection to the neighbours for the test case
    // @TODO Automatically construct this instead of manually figuring out where every edge leads
    fn fill_neighbours_part2_test(&mut self) {
        let size = self.size;
        {
            let (x, y) = (2, 0);
            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Up)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(0, 1);
                p.position.x = size - p.position.x - 1;
                p.position.y = 0;
                p.direction = Direction::Down;

                return p;
            }));

            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Left)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(1, 1);
                p.position.x = p.position.y;
                p.position.y = 0;
                p.direction = Direction::Down;

                return p;
            }));

            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Down)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(2, 1);
                p.position.y = 0;
                p.direction = Direction::Down;

                return p;
            }));

            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Right)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(3, 2);
                p.position.y = size - p.position.y - 1;
                p.position.y = size-1;
                p.direction = Direction::Left;

                return p;
            }));
        }

        {
            let (x, y) = (0, 1);
            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Up)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(2, 0);
                p.position.x = size - p.position.x - 1;
                p.position.y = 0;
                p.direction = Direction::Down;

                return p;
            }));

            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Left)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(3, 2);
                p.position.x = size - p.position.y - 1;
                p.position.y = size-1;
                p.direction = Direction::Down;

                return p;
            }));

            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Down)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(2, 2);
                p.position.x = size - p.position.x - 1;
                p.position.y = size-1;
                p.direction = Direction::Up;

                return p;
            }));

            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Right)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(1, 1);
                p.position.x = 0;

                return p;
            }));
        }

        {
            let (x, y) = (1, 1);
            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Up)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(2, 0);
                p.position.y = p.position.x;
                p.position.x = 0;
                p.direction = Direction::Right;

                return p;
            }));

            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Left)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(0, 1);
                p.position.x = size-1;

                return p;
            }));

            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Down)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(2, 2);
                p.position.y = size - p.position.x - 1;
                p.position.x = 0;
                p.direction = Direction::Right;

                return p;
            }));

            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Right)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(2, 1);
                p.position.x = 0;

                return p;
            }));
        }

        {
            let (x, y) = (2, 1);
            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Up)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(2, 0);
                p.position.y = size-1;

                return p;
            }));

            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Left)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(1, 1);
                p.position.x = size-1;

                return p;
            }));

            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Down)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(2, 2);
                p.position.y = 0;

                return p;
            }));

            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Right)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(3, 2);
                p.position.x = size - p.position.y - 1;
                p.position.y = 0;
                p.direction = Direction::Down;

                return p;
            }));
        }

        {
            let (x, y) = (2, 2);
            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Up)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(2, 1);
                p.position.y = size-1;

                return p;
            }));

            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Left)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(1, 1);
                p.position.x = size - p.position.y - 1;
                p.position.y = size-1;
                p.direction = Direction::Up;

                return p;
            }));

            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Down)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(0, 1);
                p.position.x = size - p.position.x - 1;
                p.position.y = size-1;
                p.direction = Direction::Up;

                return p;
            }));

            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Right)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(3, 2);
                p.position.x = 0;

                return p;
            }));
        }

        {
            let (x, y) = (3, 2);
            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Up)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(2, 1);
                p.position.y = size - p.position.x - 1;
                p.position.x = size-1;
                p.direction = Direction::Left;

                return p;
            }));

            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Left)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(2, 2);
                p.position.x = size-1;

                return p;
            }));

            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Down)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(0, 1);
                p.position.y = size - p.position.x - 1;
                p.position.x = 0;
                p.direction = Direction::Left;

                return p;
            }));

            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Right)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(2, 0);
                p.position.y = size - p.position.y - 1;
                p.position.x = size-1;
                p.direction = Direction::Left;

                return p;
            }));
        }
    }

    // Setup the connection to the neighbours for the actual input
    // @TODO Automatically construct this instead of manually figuring out where every edge leads
    // For now this is fine as appearently all inputs have the same shape, so this should be good
    // enough to solve every input, but it is not pretty at all...
    fn fill_neighbours_part2_solution(&mut self) {
        let size = self.size;
        {
            let (x, y) = (1, 0);
            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Up)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(0, 3);
                p.position.y = p.position.x;
                p.position.x = 0;
                p.direction = Direction::Right;

                return p;
            }));

            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Left)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(0, 2);
                p.position.y = size - p.position.y - 1;
                p.position.x = 0;
                p.direction = Direction::Right;

                return p;
            }));

            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Down)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(1, 1);
                p.position.y = 0;

                return p;
            }));

            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Right)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(2, 0);
                p.position.x = 0;

                return p;
            }));
        }

        {
            let (x, y) = (2, 0);
            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Up)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(0, 3);
                p.position.y = size-1;

                return p;
            }));

            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Left)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(1, 0);
                p.position.x = size-1;

                return p;
            }));

            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Down)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(1, 1);
                p.position.y = p.position.x;
                p.position.x = size-1;
                p.direction = Direction::Left;

                return p;
            }));

            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Right)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(1, 2);
                p.position.y = size - p.position.y - 1;
                p.position.x = size-1;
                p.direction = Direction::Left;

                return p;
            }));
        }

        {
            let (x, y) = (1, 1);
            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Up)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(1, 0);
                p.position.y = size-1;

                return p;
            }));

            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Left)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(0, 2);
                p.position.x = p.position.y;
                p.position.y = 0;
                p.direction = Direction::Down;

                return p;
            }));

            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Down)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(1, 2);
                p.position.y = 0;

                return p;
            }));

            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Right)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(2, 0);
                p.position.x = p.position.y;
                p.position.y = size-1;
                p.direction = Direction::Up;

                return p;
            }));
        }

        {
            let (x, y) = (0, 2);
            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Up)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(1, 1);
                p.position.y = p.position.x;
                p.position.x = 0;
                p.direction = Direction::Right;

                return p;
            }));

            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Left)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(1, 0);
                p.position.y = size - p.position.x - 1;
                p.position.x = 0;
                p.direction = Direction::Right;

                return p;
            }));

            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Down)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(0, 3);
                p.position.y = 0;

                return p;
            }));

            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Right)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(1, 2);
                p.position.x = 0;

                return p;
            }));
        }

        {
            let (x, y) = (1, 2);
            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Up)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(1, 1);
                p.position.y = size-1;

                return p;
            }));

            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Left)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(0, 2);
                p.position.x = size-1;

                return p;
            }));

            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Down)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(0, 3);
                p.position.y = p.position.x;
                p.position.x = size-1;
                p.direction = Direction::Left;

                return p;
            }));

            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Right)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(2, 0);
                p.position.y = size - p.position.y - 1;
                p.position.x = size-1;
                p.direction = Direction::Left;

                return p;
            }));
        }

        {
            let (x, y) = (0, 3);
            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Up)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(0, 2);
                p.position.y = size-1;

                return p;
            }));

            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Left)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(1, 0);
                p.position.x = p.position.y;
                p.position.y = 0;
                p.direction = Direction::Down;

                return p;
            }));

            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Down)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(2, 0);
                p.position.y = 0;

                return p;
            }));

            self.planes[y][x].as_mut().unwrap().neighbours[usize::from(&Direction::Right)] = Some(Box::new(move |mut p: Player| {
                p.plane = Vec2::new(1, 2);
                p.position.x = p.position.y;
                p.position.y = size-1;
                p.direction = Direction::Up;

                return p;
            }));
        }
    }
}

fn parse_movement(input: &str) -> Vec<(usize, Option<bool>)> {
    let re = Regex::new(r"(?P<steps>[0-9]+)(?P<direction>L|R)?").unwrap();

    re.captures_iter(input).map(|capture| {
        let steps: usize = capture.name("steps").unwrap().as_str().parse().unwrap();

        let clockwise = capture.name("direction").map(|direction| {
            match direction.as_str() {
                "L" => false,
                "R" => true,
                _ => panic!("Invalid rotation: {}", direction.as_str()),
            }
        });

        (steps, clockwise)
    }).collect()
}

fn is_test(input: &str) -> bool {
    input.len() == 189
}

// -- Solution --
pub struct Day;
impl aoc::Solver for Day {
    type Output1 = usize;
    type Output2 = usize;

    fn day() -> u8 {
        22
    }

    fn part1(input: &str) -> Self::Output1 {
        // Split the input into the two different parts
        let (map, movement) = input.split_once("\n\n").unwrap();

        // Create the map
        let size = if is_test(input) { 4 } else { 50 };
        let mut map = Map::new(map, size);

        // Connect the planes together
        map.fill_neighbours_part1();

        // Create the movement instructions and execute them
        let moves = parse_movement(movement);
        map.movement(moves);

        map.score()
    }

    fn part2(input: &str) -> Self::Output2 {
        // Split the input into the two different parts
        let (map, movement) = input.split_once("\n\n").unwrap();

        // Create the map
        let size = if is_test(input) { 4 } else { 50 };
        let mut map = Map::new(map, size);

        if is_test(input) {
            map.fill_neighbours_part2_test();
        } else {
            map.fill_neighbours_part2_solution();
        }

        // Create the movement instructions and execute them
        let moves = parse_movement(movement);
        map.movement(moves);

        map.score()
    }
}
