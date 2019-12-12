use crate::Orientation::{Down, Left, Right, Up};
use intcode::IntcodeProgram;
use log::debug;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Write;

const ROBOT_PROGRAM: &str = include_str!("../day11.txt");

const BLACK: i64 = 0;
const WHITE: i64 = 1;

const DO_TURN_LEFT: i64 = 0;
const DO_TURN_RIGHT: i64 = 1;

const PT2_IMG_PATH: &str = "./day11/part2-output.pbm";

fn main() {
    println!("Day 11-1: Panels painted at least once: {}", day11_part1());
    day11_part2();
}

fn day11_part1() -> usize {
    run_robot(PanelColor::Black).num_panels_painted()
}

fn day11_part2() {
    let robot = run_robot(PanelColor::White);
    match robot.save_image(PT2_IMG_PATH) {
        Ok(_) => println!("Day 11-2: Saved image to '{}'", PT2_IMG_PATH),
        Err(_) => println!("Day 11-2: Failed to save image!"),
    }
}

fn run_robot(first_color: PanelColor) -> Robot {
    let mut robot = Robot::create();
    robot.paint_panel(first_color.as_input());

    let mut program = IntcodeProgram::init_from(ROBOT_PROGRAM);
    while !program.is_halted() {
        program.run();

        // consume any outputs first, until they're exhausted
        while let (Some(c), Some(t)) = (program.consume_output(), program.consume_output()) {
            robot.paint_panel(c);
            robot.turn_advance(t);
        }

        // provide updated input when requested
        if program.is_awaiting_input() {
            let i = robot.get_panel_color().as_input();
            debug!("Providing INPUT {}", i);
            program.buffer_input(i);
        }
    }

    robot
}

struct Robot {
    location: Point,
    orientation: Orientation,
    panels: HashMap<Point, PanelColor>,
}

impl Robot {
    fn create() -> Robot {
        Robot {
            location: Point { x: 0, y: 0 },
            orientation: Up,
            panels: HashMap::new(),
        }
    }

    fn turn_advance(&mut self, input: i64) {
        self.orientation = self.orientation.change(input);
        debug!("{}: Turning {:?}", input, self.orientation);
        match self.orientation {
            // (0, 0) is upper left corner
            Up => self.location.y -= 1,
            Down => self.location.y += 1,
            Right => self.location.x += 1,
            Left => self.location.x -= 1,
        };
    }

    fn get_panel_color(&self) -> &PanelColor {
        self.panels
            .get(&self.location)
            .unwrap_or(&PanelColor::Black)
    }

    fn paint_panel(&mut self, input: i64) {
        let color = match input {
            BLACK => PanelColor::Black,
            WHITE => PanelColor::White,
            _ => panic!("Bad color"),
        };

        debug!("Paint panel: {:?} {:?}", self.location, color);
        self.panels.insert(self.location, color);
    }

    fn num_panels_painted(&self) -> usize {
        self.panels.keys().len()
    }

    fn save_image(&self, output_pbm: &str) -> io::Result<()> {
        let min_x = self.panels.keys().map(|p| p.x).min().unwrap();
        let max_x = self.panels.keys().map(|p| p.x).max().unwrap();
        let min_y = self.panels.keys().map(|p| p.y).min().unwrap();
        let max_y = self.panels.keys().map(|p| p.y).max().unwrap();

        let height = (max_y - min_y + 1) as usize;
        let width = (max_x - min_x + 1) as usize;

        let mut out: Vec<Vec<char>> = vec![vec!['0'; width]; height];
        for (point, color) in &self.panels {
            if let PanelColor::White = color {
                let pix_x = shift_origin(point.x, min_x);
                let pix_y = shift_origin(point.y, min_y);
                out[pix_y][pix_x] = '1'
            }
        }

        let mut f = File::create(output_pbm)?;

        let mut buf = String::new();
        buf.push_str(format!("P1\n{} {}\n", width, height).as_str());
        let mut p = 0;
        out.iter().flatten().for_each(|it| {
            buf.push(*it);
            p += 1;
            buf.push(if p % width == 0 { '\n' } else { ' ' });
        });
        f.write(buf.as_bytes())?;

        Ok(())
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

impl Orientation {
    fn change(&self, input: i64) -> Orientation {
        match (self, input) {
            (Up, DO_TURN_LEFT) | (Down, DO_TURN_RIGHT) => Left,
            (Up, DO_TURN_RIGHT) | (Down, DO_TURN_LEFT) => Right,
            (Right, DO_TURN_RIGHT) | (Left, DO_TURN_LEFT) => Down,
            (Right, DO_TURN_LEFT) | (Left, DO_TURN_RIGHT) => Up,
            _ => panic!("Bad input for direction change"),
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn shift_origin(c: i32, min_c: i32) -> usize {
    (if min_c < 0 { c + min_c * -1 } else { c }) as usize
}

#[derive(Debug)]
enum PanelColor {
    Black,
    White,
}

impl PanelColor {
    fn as_input(&self) -> i64 {
        match self {
            PanelColor::Black => BLACK,
            PanelColor::White => WHITE,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day11_part1;

    #[test]
    fn test_part1() {
        assert_eq!(day11_part1(), 1894)
    }
}
