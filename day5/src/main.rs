use std::io::stdin;
use std::collections::VecDeque;
use std::ops::{Add, AddAssign};
use std::error::Error;
use std::fmt::{Display, Formatter};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    run_diagnostic();
//    println!("DSI {:?}", destructure_inst(1002))
}

// OPCODE 1: 1,p1,p2,dest; mem[dest] = mem[p1] + mem[p2]
// OPCODE 2: 2,p1,p2,dest; mem[dest] = mem[p1] * mem[p2]
// OPCODE 3: 3,p1; mem[p1] = <input i32>
// OPCODE 4: 4,p1; <output mem[p1]>
// OPCODE 99: halt
// OPCODE anything else: error

// want instrs:
//   ADD((p1, mode), (p2, mode), (dst, mode = POSITION))

fn get_parameter(pc: usize, memory: &Vec<i32>, p_index: usize, inst: &Instruction) -> i32 {
    let mode = inst.addr_modes.get(p_index);
    match mode {
        Some(AddressingMode::IMMEDIATE) => memory[pc + 1 + p_index],
        Some(AddressingMode::POSITION) => memory[memory[pc + 1 + p_index] as usize],
        _ => panic!("panic!!!")
    }
}

fn run_diagnostic() -> Result<i32, String> {
    let input: Vec<&str> = INPUT.trim().split(',').collect();
    let mut memory: Vec<i32> = input
        .iter()
        .map(|it| it.parse::<i32>().expect("Can't parse int"))
        .collect();

    let mut pc = 0;
    while pc < memory.len() {
        let instruction = destructure_inst(memory[pc]);
        match instruction {
            Ok(inst) => {
                match inst.opcode {
                    OP_ADD => {
                        let r0 = get_parameter(pc, &memory, 0, &inst);
                        let r1 = get_parameter(pc, &memory, 1, &inst);
                        let dest_pos = memory[pc + 3] as usize;

                        memory[dest_pos] = r0 + r1;
                        pc += 4;
                    }
                    OP_MUL => {
                        let r0 = get_parameter(pc, &memory, 0, &inst);
                        let r1 = get_parameter(pc, &memory, 1, &inst);
                        let dest_pos = memory[pc + 3] as usize;

                        memory[dest_pos] = r0 * r1;
                        pc += 4;
                    }
                    OP_INPUT => {
                        let mut s = String::new();
                        println!("Input value: ");
                        stdin().read_line(&mut s).expect("Failed to read line");
                        let value = s.trim().parse::<i32>().expect("Bad input integer");

                        let dest_pos = memory[pc + 1] as usize;
                        memory[dest_pos] = value;
                        pc += 2;
                    }
                    OP_OUTPUT => {
                        let output = match inst.addr_modes.get(0).unwrap() {
                            AddressingMode::IMMEDIATE => memory[pc + 1],
                            AddressingMode::POSITION => memory[memory[pc + 1] as usize]
                        };
                        println!("{}", output);
                        pc += 2;
                    }
                    OP_HALT => return Ok(memory[0]),
                    x => {
                        return Err(format!("Unknown opcode: {}", x))
                    }
                };
//                println!("opcode: {}", inst.opcode);
            }
            Err(e) => panic!(e)
        }
    }
    Ok(memory[0])
}

#[derive(Debug)]
struct Instruction {
    opcode: i8,
    addr_modes: Vec<AddressingMode>
}

struct Parameter {
    mode: AddressingMode,
    value: i32 // interpret based on mode
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

#[derive(Debug)]
struct InvalidOpcode {

}

impl Error for InvalidOpcode {
    fn description(&self) -> &str {
        "Invalid Opcode"
    }
}

impl Display for InvalidOpcode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Invalid opcode")
    }
}

const OP_ADD: i8 = 1;
const OP_MUL: i8 = 2;
const OP_INPUT: i8 = 3;
const OP_OUTPUT: i8 = 4;
const OP_HALT: i8 = 99;

fn destructure_inst(inst: i32) -> Result<Instruction, InvalidOpcode> {
    let mut digits = digits(inst);
//    println!("digits: {:?}", digits);

    let mut addr_modes: Vec<AddressingMode> = Vec::new();

    let opcode_0 = digits.pop_back().unwrap_or_default();
    let opcode_1 = digits.pop_back().unwrap_or_default();
    let opcode: i8 = opcode_1 * 10 + opcode_0;

    for _i in 0..3 {
        addr_modes.push(AddressingMode::from(digits.pop_back()));
    }

    Ok(Instruction { opcode, addr_modes })
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
