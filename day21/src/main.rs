use intcode::{parse_intcode_input, IntcodeProgram};

const PROGRAM: &str = include_str!("../day21.txt");

fn main() {
    println!(
        "Day 21-1: Hull damage: {}",
        run_robot_part1().expect("Robot tragically died")
    );
}

const PT1_ROUTINE: &str = "NOT A J
AND B J
AND C J
NOT C T
AND A T
AND B T
OR T J
NOT B T
OR T J
NOT D T
NOT T T
AND T J
WALK\n";

fn run_robot_part1() -> Option<i64> {
    let mut inputs: Vec<i64> = Vec::new();
    add_ascii_inputs(&mut inputs, PT1_ROUTINE);

    let program_memory = parse_intcode_input(PROGRAM);

    // init program with all inputs already buffered
    let mut program = IntcodeProgram::init(&program_memory, inputs);
    while !program.is_halted() {
        program.run();

        while let Some(output) = program.consume_output() {
            let ch = std::char::from_u32(output as u32);
            if let Some(c) = ch {
                print!("{}", c); // robot died
            } else {
                return Some(output); // hull damage
            }
        }
    }
    None
}

fn add_ascii_inputs(input_vec: &mut Vec<i64>, input_str: &str) {
    input_str
        .chars()
        .map(|c| c as i64)
        .for_each(|ascii_val| input_vec.push(ascii_val));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(run_robot_part1(), Some(19355364))
    }
}
