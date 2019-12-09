extern crate env_logger;
extern crate intcode;
extern crate log;

use intcode::IntcodeProgram;
use intcode::IntcodeResult;

const INPUT: &str = include_str!("../day9.txt");

fn main() {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Debug)
        .init();
    day9_part1()
}

fn day9_part1() {
    let mut program = IntcodeProgram::init_from(INPUT);
    program.buffer_input(1);

    loop {
        let result = program.run();
        match result {
            IntcodeResult::Halted => {
                break;
            }
            IntcodeResult::Output { output: o } => {
                println!("output: {}", o);
            }
            _ => {}
        }
    }
}
