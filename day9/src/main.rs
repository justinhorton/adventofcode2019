extern crate env_logger;
extern crate intcode;
extern crate log;

use intcode::IntcodeProgram;

const INPUT: &str = include_str!("../day9.txt");

fn main() {
    //    env_logger::Builder::new()
    //        .filter_level(log::LevelFilter::Debug)
    //        .init();
    println!("Day 9-1: {}", run_day9(1));
    println!("Day 9-2: {}", run_day9(2));
}

fn run_day9(input_code: i64) -> i64 {
    let mut program = IntcodeProgram::init_from(INPUT);
    program.buffer_input(input_code);
    program.run();
    program.consume_output().expect("No output")
}

#[cfg(test)]
mod tests {
    use crate::run_day9;

    #[test]
    fn test_part1() {
        assert_eq!(run_day9(1), 4234906522)
    }

    #[test]
    fn test_part2() {
        assert_eq!(run_day9(2), 60962)
    }
}
