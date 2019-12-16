extern crate intcode;

use intcode::IntcodeProgram;
use std::collections::{HashSet, VecDeque};

const ROBOT_PROGRAM: &str = include_str!("../day15.txt");

const STATUS_WALL: i64 = 0;
const STATUS_MOVED: i64 = 1;
const STATUS_FOUND_OXYGEN: i64 = 2;

fn main() {
    let (steps_to_oxygen, grid) = run_droid_part1();
    // Position { x: -18, y: 16 }
    println!("Day 15-1: {} movements to reach oxygen", steps_to_oxygen);
    println!(
        "Day 15-2: {} mins to fill space",
        disperse_oxygen_part2(&grid)
    );
}

type TimeToDisperse = u32;
type Depth = u32;
fn disperse_oxygen_part2(grid: &Grid) -> TimeToDisperse {
    let oxygen = Position { x: -18, y: 16 };

    let mut visited = HashSet::new();
    let mut visit_queue: VecDeque<(Position, Depth)> = VecDeque::new();
    visit_queue.push_back((oxygen, 0));

    let mut max_depth = 0;
    while !visit_queue.is_empty() {
        let (cur_pos, depth) = visit_queue.pop_front().unwrap();
        visited.insert(cur_pos);

        for direction in DIRECTIONS.iter() {
            let pos = direction.apply_to(cur_pos);
            if !visited.contains(&pos) && grid.get(pos.x, pos.y) != Grid::WALL {
                visited.insert(pos);
                visit_queue.push_back((pos, depth + 1));
            }
        }

        if depth > max_depth {
            max_depth = depth;
        }
    }

    max_depth
}

type StepsToOxygen = i32;
fn run_droid_part1() -> (StepsToOxygen, Grid) {
    let mut program = IntcodeProgram::init_from(ROBOT_PROGRAM);

    // start the program, bringing us to the movement I/O loop
    program.run();

    let mut grid = Grid::create();
    grid.set(0, 0, Grid::DROID);

    let mut min_steps = -1;
    let start_pos = Position { x: 0, y: 0 };
    let mut visited: HashSet<Position> = HashSet::new();
    dfs_part1(
        &mut grid,
        &mut program,
        &mut visited,
        start_pos,
        0,
        &mut min_steps,
    );

    (min_steps, grid)
}

fn dfs_part1(
    grid: &mut Grid,
    program: &mut IntcodeProgram,
    visited: &mut HashSet<Position>,
    cur_pos: Position,
    steps: i32,
    min_steps: &mut i32,
) {
    for direction in DIRECTIONS.iter() {
        let new_pos = direction.apply_to(cur_pos);

        if !visited.contains(&new_pos) {
            visited.insert(new_pos);

            match try_move(program, direction) {
                STATUS_WALL => {
                    grid.set(new_pos.x, new_pos.y, Grid::WALL);
                }
                STATUS_MOVED => {
                    grid.set(new_pos.x, new_pos.y, Grid::CLEAR);

                    dfs_part1(grid, program, visited, new_pos, steps + 1, min_steps);
                    try_move(program, &direction.reverse());
                }
                STATUS_FOUND_OXYGEN => {
                    grid.set(new_pos.x, new_pos.y, Grid::OXYGEN);

                    if *min_steps == -1 || steps + 1 < *min_steps {
                        *min_steps = steps + 1;
                    }
                    try_move(program, &direction.reverse());
                }
                _ => panic!("Bad output"),
            };
        }
    }
}

fn try_move(program: &mut IntcodeProgram, movement: &Direction) -> i64 {
    program.buffer_input(movement.as_input());
    program.run();
    program.consume_output().unwrap()
}

struct Grid {
    arr: Vec<Vec<char>>,
}

impl Grid {
    const ORIGIN_X: i32 = 25;
    const ORIGIN_Y: i32 = 25;
    const WIDTH: usize = 50;
    const HEIGHT: usize = 50;
    const WALL: char = '#';
    const OXYGEN: char = 'O';
    const DROID: char = 'D';
    const CLEAR: char = ' ';
    const UNKNOWN: char = '?';

    fn create() -> Grid {
        Grid {
            arr: vec![vec![Grid::UNKNOWN; Grid::WIDTH]; Grid::HEIGHT],
        }
    }

    fn set(&mut self, x: i32, y: i32, c: char) {
        let adj_y = (y + Grid::ORIGIN_Y) as usize;
        let adj_x = (x + Grid::ORIGIN_X) as usize;
        self.arr[adj_y][adj_x] = c;
    }

    fn get(&self, x: i32, y: i32) -> char {
        let adj_y = (y + Grid::ORIGIN_Y) as usize;
        let adj_x = (x + Grid::ORIGIN_X) as usize;
        self.arr[adj_y][adj_x]
    }

    fn print(&self) {
        for i in 0..Grid::HEIGHT {
            for j in 0..Grid::WIDTH {
                print!("{}", self.arr[i][j]);
            }
            println!();
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Position {
    x: i32,
    y: i32,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::East,
    Direction::West,
];

enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    const INPUT_NORTH: i64 = 1;
    const INPUT_SOUTH: i64 = 2;
    const INPUT_WEST: i64 = 3;
    const INPUT_EAST: i64 = 4;

    fn reverse(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        }
    }

    fn apply_to(&self, pos: Position) -> Position {
        let pos_x = pos.x;
        let pos_y = pos.y;
        match self {
            Direction::North => Position {
                x: pos_x,
                y: pos_y + 1,
            },
            Direction::South => Position {
                x: pos_x,
                y: pos_y - 1,
            },
            Direction::West => Position {
                x: pos_x - 1,
                y: pos_y,
            },
            Direction::East => Position {
                x: pos_x + 1,
                y: pos_y,
            },
        }
    }

    fn as_input(&self) -> i64 {
        match self {
            Direction::North => Direction::INPUT_NORTH,
            Direction::South => Direction::INPUT_SOUTH,
            Direction::West => Direction::INPUT_WEST,
            Direction::East => Direction::INPUT_EAST,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{disperse_oxygen_part2, run_droid_part1};

    #[test]
    fn test_part1() {
        let (steps, _) = run_droid_part1();
        assert_eq!(steps, 234);
    }

    #[test]
    fn test_part2() {
        let (_, grid) = run_droid_part1();
        assert_eq!(disperse_oxygen_part2(&grid), 292);
    }
}
