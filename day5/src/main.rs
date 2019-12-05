use std::io::stdin;
use std::collections::VecDeque;
use std::ops::{Add, AddAssign};

const INPUT: &str = "3,0,4,0,99";

fn main() {
    println!("Day 5-1: {}", calc_day5());
    println!("DSI {:?}", destructure_inst(1002))
}

// OPCODE 1: 1,p1,p2,dest; mem[dest] = mem[p1] + mem[p2]
// OPCODE 2: 2,p1,p2,dest; mem[dest] = mem[p1] * mem[p2]
// OPCODE 3: 3,p1; mem[p1] = <input i32>
// OPCODE 4: 4,p1; <output mem[p1]>
// OPCODE 99: halt
// OPCODE anything else: error
fn calc_day5() -> i32 {
    let input: Vec<&str> = INPUT.trim().split(',').collect();
    let mut memory: Vec<i32> = input
        .iter()
        .map(|it| it.parse::<i32>().expect("Can't parse int"))
        .collect();

    let mut pos = 0;
    while pos < memory.len() {
        let opcode = memory[pos];
        match opcode {
            1..=2 => {
                let op1_pos = memory[pos + 1] as usize;
                let op2_pos = memory[pos + 2] as usize;
                let dest_pos = memory[pos + 3] as usize;
                let op1 = memory[op1_pos];
                let op2 = memory[op2_pos];

                let result = if opcode == 1 { op1 + op2 } else { op1 * op2 };
                memory[dest_pos] = result;

                pos += 4;
            }
            3 => {
                let mut s = String::new();
                println!("Input value: ");
                stdin().read_line(&mut s);
                let value = s.trim().parse::<i32>().expect("Bad input value");

                let dest_pos = memory[pos + 1] as usize;
                memory[dest_pos] = value;
                pos += 2;
            }
            4 => {
                let output_pos = memory[pos + 1] as usize;

                println!("{}", memory[output_pos]);
                pos += 2;
            }
            99 => return memory[0],
            x => panic!("Unknown opcode {}!", x),
        };
    }
    memory[0]
}

#[derive(Debug)]
enum AddressingMode {
    POSITION,
    IMMEDIATE,
}

impl AddressingMode {
    fn from(i: Option<i8>) -> AddressingMode {
        match i {
            None | Some(0) => AddressingMode::POSITION,
            Some(1) => AddressingMode::IMMEDIATE,
            Some(x) => panic!("Unknown addressing mode: {}", x)
        }
    }
}

fn destructure_inst(inst: i32) -> (i8, AddressingMode, AddressingMode, AddressingMode) {
    let mut digits = digits(inst);
    let opcode_0 = digits.pop_back().unwrap();
    let opcode_1 = digits.pop_back().unwrap();
    let opcode: i8 = opcode_1 * 10 + opcode_0;

    let p1_addressing = AddressingMode::from(digits.pop_back());
    let p2_addressing = AddressingMode::from(digits.pop_back());
    let p3_addressing = AddressingMode::from(digits.pop_back());

    return (opcode, p1_addressing, p2_addressing, p3_addressing)
}

fn digits(num: i32) -> VecDeque<i8> {
    let mut digits: VecDeque<i8> = VecDeque::new();
    let mut value = num;
    while value > 0 {
        digits.push_front((value % 10) as i8);
        value = value / 10;
    }
    digits
}
