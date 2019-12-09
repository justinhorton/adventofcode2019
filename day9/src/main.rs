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
    run_day9(2)
}

fn run_day9(input_code: i64) {
    let mut program = IntcodeProgram::init_from(INPUT);
    program.buffer_input(input_code);

    // TODO: Move this output loop into the program and read from output buffer at the end.
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
