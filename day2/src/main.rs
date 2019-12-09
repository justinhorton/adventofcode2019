extern crate intcode;

use intcode::{parse_intcode_input, IntcodeProgram, IntcodeResult};

const INPUT: &str = include_str!("../day2.txt");
const NOUN: i32 = 12;
const VERB: i32 = 2;

fn main() {
    println!("Day 2-1: {}", calc_day2(NOUN, VERB));
    println!("Day 2-2: {}", day2_pt2().expect("No answer found!"))
}

fn day2_pt2() -> Option<i32> {
    for n in 0..99 {
        for v in 0..99 {
            let result = calc_day2(n, v);
            if result == 19690720 {
                return Some(100 * n + v);
            }
        }
    }
    None
}

// OPCODE 1: 1,op1,op2,dest; set dest = *op1 + *op2
// OPCODE 2: 2,op1,op2,dest; set dest = *op1 * *op2
// OPCODE 99: halt
// OPCODE anything else: error
fn calc_day2(noun: i32, verb: i32) -> i32 {
    let mut memory: Vec<i32> = parse_intcode_input(&INPUT);

    // restore the state at the time of the elf problem
    memory[1] = noun;
    memory[2] = verb;

    let mut intcode = IntcodeProgram::init(&memory, Default::default());
    match intcode.run() {
        IntcodeResult::Halted => {
            return intcode.mem_value(0);
        },
        x => panic!("Unexpected intcode result: {:?}", x),
    }
}


#[cfg(test)]
mod tests {
    use crate::{calc_day2, NOUN, VERB, day2_pt2};
    use intcode::{IntcodeProgram, parse_intcode_input};

    #[test]
    fn test_simple1() {
        let memory = parse_intcode_input("1,0,0,0,99");
        let mut intcode = IntcodeProgram::init(&memory, Default::default());
        intcode.run();
        assert_eq!(intcode.mem_value(0), 2);
    }

    #[test]
    fn test_part1() {
        let result = calc_day2(NOUN, VERB);
        assert_eq!(result, 3101878);
    }

    #[test]
    fn test_part2() {
        let result = day2_pt2();
        assert_eq!(result, Some(8444));
    }
}
