use std::collections::VecDeque;
use std::convert::TryInto;
use std::fmt::{Display, Formatter};
use std::fs::ReadDir;
use std::io::stdin;

const INPUT: &str = include_str!("../day5.txt");

fn main() {
    match run_diagnostic() {
        Ok(()) => println!("Diagnostic completed"),
        Err(e) => println!("Diagnostic failed\n{}", e),
    }
}

fn run_diagnostic() -> Result<(), String> {
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
                println!("{:?}", inst);

                let r0 = get_parameter(pc, &memory, 0, &inst);
                let r1 = get_parameter(pc, &memory, 1, &inst);
                let r2 = get_parameter(pc, &memory, 2, &inst);
                match inst.op {
                    Op::Add => {
                        memory[r2.unwrap() as usize] = r0.unwrap() + r1.unwrap();
                        pc += 4;
                    }
                    Op::Mul => {
                        memory[r2.unwrap() as usize] = r0.unwrap() * r1.unwrap();
                        pc += 4;
                    }
                    Op::Input => {
                        println!("Input an integer: ");

                        let mut s = String::new();
                        let read_line = stdin().read_line(&mut s);
                        if read_line.is_err() {
                            return Err("Error parsing input line".to_string());
                        }

                        let int_val = match s.trim().parse::<i32>() {
                            Ok(x) => x,
                            Err(e) => return Err(e.to_string()),
                        };

                        let dst = memory[pc + 1] as usize;
                        memory[dst] = int_val;
                        pc += 2;
                    }
                    Op::Output => {
                        println!("{}", r0.unwrap());
                        pc += 2;
                    }
                    Op::Jit => {
                        let new_pc = r1.unwrap() as usize;
                        pc = if r0.unwrap() != 0 { new_pc } else { pc + 3 };
                    }
                    Op::Jif => {
                        let new_pc = r1.unwrap() as usize;
                        pc = if r0.unwrap() == 0 { new_pc } else { pc + 3 };
                    }
                    Op::Lt => {
                        memory[r2.unwrap() as usize] =
                            if r0.unwrap() < r1.unwrap() { 1 } else { 0 };
                        pc += 4;
                    }
                    Op::Eq => {
                        memory[r2.unwrap() as usize] =
                            if r0.unwrap() == r1.unwrap() { 1 } else { 0 };
                        pc += 4;
                    }
                    Op::Halt => return Ok(()),
                };
            }
            Err(e) => return Err(format!("Invalid opcode: {:?}", e)),
        }
    }

    Err(format!("Program halted unexpectedly"))
}

fn destructure_inst(inst: i32) -> Result<Instruction, InvalidOpCodeError> {
    let mut digits = digits(inst);
    let mut addr_modes: Vec<AddressingMode> = Vec::new();

    let opcode_0 = digits.pop_back().unwrap_or_default();
    let opcode_1 = digits.pop_back().unwrap_or_default();
    let opcode: i8 = opcode_1 * 10 + opcode_0;

    let op = Op::from_opcode(opcode)?;
    for _i in 1..op.num_parameters() {
        addr_modes.push(AddressingMode::from(digits.pop_back())?);
    }

    if op.does_store() {
        addr_modes.push(AddressingMode::DoStore);
    } else if op.num_parameters() != 0 {
        addr_modes.push(AddressingMode::from(digits.pop_back())?)
    }

    Ok(Instruction { op, addr_modes })
}

fn get_parameter(
    pc: usize,
    memory: &Vec<i32>,
    parm_index: usize, // 0-indexed
    inst: &Instruction,
) -> Option<i32> {
    if parm_index >= inst.op.num_parameters() {
        return None;
    } else {
        let parm_slot = pc + parm_index + 1;
        let mode = inst.addr_modes.get(parm_index);
        mode.map(|m| match m {
            AddressingMode::Immediate => memory[parm_slot],
            AddressingMode::Position => {
                let pos = memory[parm_slot];
                memory[pos as usize]
            }
            AddressingMode::DoStore => memory[parm_slot],
        })
    }
}

#[derive(Debug)]
struct Instruction {
    op: Op,
    addr_modes: Vec<AddressingMode>,
}

#[derive(Debug)]
enum Op {
    Add,
    Mul,
    Input,
    Output,
    Jit,
    Jif,
    Lt,
    Eq,
    Halt,
}

/*
    Opcode 1 adds together numbers read from two positions and stores the result in a third position.
    Opcode 2 works exactly like opcode 1, except it multiplies the two inputs instead of adding them.
    Opcode 3 takes a single integer as input and saves it to the position given by its only parameter.
    Opcode 4 outputs the value of its only parameter. For example, the instruction 4,50 would output the value at address 50.
    Opcode 5 is jump-if-true: if the first parameter is non-zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
    Opcode 6 is jump-if-false: if the first parameter is zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
    Opcode 7 is less than: if the first parameter is less than the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
    Opcode 8 is equals: if the first parameter is equal to the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
*/
impl Op {
    fn from_opcode(opcode: i8) -> Result<Op, InvalidOpCodeError> {
        let op = match opcode {
            1 => Some(Op::Add),
            2 => Some(Op::Mul),
            3 => Some(Op::Input),
            4 => Some(Op::Output),
            5 => Some(Op::Jit),
            6 => Some(Op::Jif),
            7 => Some(Op::Lt),
            8 => Some(Op::Eq),
            99 => Some(Op::Halt),
            _ => None,
        };
        op.ok_or_else(|| InvalidOpCodeError {
            message: format!("Invalid opcode: {}", opcode),
        })
    }

    fn num_parameters(&self) -> usize {
        match self {
            Op::Add | Op::Mul | Op::Eq | Op::Lt => 3,
            Op::Jit | Op::Jif => 2,
            Op::Input | Op::Output => 1,
            Op::Halt => 0,
        }
    }

    fn does_store(&self) -> bool {
        match self {
            Op::Add | Op::Mul | Op::Eq | Op::Input | Op::Lt => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
enum AddressingMode {
    Position,
    Immediate,
    DoStore,
}

impl AddressingMode {
    fn from(i: Option<i8>) -> Result<AddressingMode, InvalidOpCodeError> {
        match i {
            Some(0) | None => Ok(AddressingMode::Position),
            Some(1) => Ok(AddressingMode::Immediate),
            Some(x) => Err(InvalidOpCodeError {
                message: format!("Unknown addressing mode: {}", x),
            }),
        }
    }
}

#[derive(Debug)]
struct InvalidOpCodeError {
    message: String,
}

impl Display for InvalidOpCodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Invalid opcode {}", self.message)
    }
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
