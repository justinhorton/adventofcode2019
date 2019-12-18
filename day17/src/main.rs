extern crate intcode;

use intcode::{IntcodeProgram, parse_intcode_input};

const PROGRAM: &str = include_str!("../day17.txt");

fn main() {
    run_robot_part1();
    run_robot_part2();
}

const ROUTINE_A: &str = "L,12,L,10,R,8,L,12\n";
const ROUTINE_B: &str = "R,8,R,10,R,12\n";
const ROUTINE_C: &str = "L,10,R,12,R,8\n";
const MAIN_ROUTINE: &str = "A,B,A,B,C,C,B,A,B,C\n";
const CONTINUOUS_VIDEO: &str = "n\n";

fn run_robot_part2() {
    let mut inputs: Vec<i64> = Vec::new();
    add_ascii_inputs(&mut inputs, MAIN_ROUTINE);
    add_ascii_inputs(&mut inputs, ROUTINE_A);
    add_ascii_inputs(&mut inputs, ROUTINE_B);
    add_ascii_inputs(&mut inputs, ROUTINE_C);
    add_ascii_inputs(&mut inputs, CONTINUOUS_VIDEO);

    let mut program_memory = parse_intcode_input(PROGRAM);
    program_memory[0] = 2; // set the robot to wake up

    // init program with all inputs already buffered
    let mut program = IntcodeProgram::init(&program_memory, inputs);

    let mut last_output: Option<i64> = None;
    while !program.is_halted() {
        program.run();

        while let Some(output) = program.consume_output() {
            // the robot still spits out some ASCII, but the last output is the dust amount
            last_output = Some(output);
        }
    }

    println!("Day 17-2: Dust collected: {:?} ", last_output.unwrap());
}

fn add_ascii_inputs(input_vec: &mut Vec<i64>, input_str: &str) {
    input_str.chars()
        .map(|c| c as i64)
        .for_each(|ascii_val| input_vec.push(ascii_val));
}

fn run_robot_part1() {
    let mut program = IntcodeProgram::init_from(PROGRAM);
    let mut grid = Grid::create();

    let mut x: usize = 0;
    let mut y: usize = 0;
    while !program.is_halted() {
        program.run();

        while let Some(output) = program.consume_output() {
            let c = std::char::from_u32(output as u32).unwrap();
            match c {
                '\n' => {
                    y += 1;
                    x = 0;
                },
                '.' => {
                    grid.set(x, y, ' '); // print empty as a space, easier on the eyes
                    x += 1;
                }
                _ => {
                    grid.set(x, y, c);
                    x += 1;
                }
            }
        }
    }

    grid.mark_intersections();
    grid.print();

    println!("Day 17-1: Alignment: {}", grid.sum_alignment());
}

struct Grid {
    arr: Vec<Vec<char>>,
}

impl Grid {
    const HEIGHT: usize = 65;
    const WIDTH: usize = 44;

    const SCAFFOLD: char = '#';
    const INTERSECTION: char = 'O';

    fn create() -> Grid {
        Grid {
            arr: vec![vec![' '; Grid::WIDTH]; Grid::HEIGHT],
        }
    }

    fn set(&mut self, x: usize, y: usize, c: char) {
        self.arr[y][x] = c;
    }

    fn print(&self) {
        for i in 0..Grid::HEIGHT {
            for j in 0..Grid::WIDTH {
                print!("{}", self.arr[i][j]);
            }
            println!();
        }
    }

    fn mark_intersections(&mut self) {
        for y in 0..Grid::HEIGHT {
            for x in 0..Grid::WIDTH {
                if self.is_intersection(x, y) {
                    self.arr[y][x] = Grid::INTERSECTION;
                }
            }
        }
    }

    fn is_intersection(&self, x: usize, y: usize) -> bool {
        self.is_scaffold(x, y)
            && y > 0 && self.is_scaffold(x, y - 1)
            && y < Grid::HEIGHT - 1 && self.is_scaffold(x, y + 1)
            && x > 0 && self.is_scaffold(x - 1, y)
            && x < Grid::WIDTH - 1 && self.is_scaffold(x + 1, y)
    }

    fn is_scaffold(&self, x: usize, y: usize) -> bool {
        self.arr[y][x] == Grid::SCAFFOLD
    }

    fn sum_alignment(&self) -> usize {
        let mut sum = 0;
        for i in 0..Grid::HEIGHT {
            for j in 0..Grid::WIDTH {
                if self.arr[i][j] == Grid::INTERSECTION {
                    sum += i * j;
                }
            }
        }
        sum
    }
}
