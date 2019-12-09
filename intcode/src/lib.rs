extern crate log;

use log::debug;
use std::fmt::{Formatter, Display};
use std::collections::VecDeque;

pub struct IntcodeProgram {
    memory: Vec<i32>,
    input_buf: VecDeque<i32>,
    pc: usize,
}

impl IntcodeProgram {
    pub fn init(memory: &Vec<i32>, inputs: Vec<i32>) -> IntcodeProgram {
        IntcodeProgram {
            memory: memory.clone(),
            input_buf: VecDeque::from(inputs),
            pc: 0,
        }
    }

    pub fn run(&mut self) -> IntcodeResult {
        debug!("Resuming with PC: {}", self.pc);

        loop {
            let instruction = destructure_inst(self.memory[self.pc]);
            match instruction {
                Ok(inst) => {
                    let operation = inst.as_operation(self.pc, &self.memory);
                    let result = self.apply(&operation);
                    match result {
                        IntcodeResult::AdvanceInstruction => {}
                        _ => return result,
                    }
                }
                Err(e) => panic!(format!("Aborting, invalid opcode: {:?}", e)),
            }
        }
    }

    fn apply(&mut self, operation: &Operation) -> IntcodeResult {
        match operation.op {
            Op::Add => {
                let store = operation.r2() as usize;
                let r0 = operation.r0();
                let r1 = operation.r1();

                self.store(store, r0 + r1);
                debug!("ADD {} {} -> {}", r0, r1, store);
                self.inc_pc(4);
            }
            Op::Mul => {
                let store = operation.r2() as usize;
                let r0 = operation.r0();
                let r1 = operation.r1();

                self.store(store, r0 * r1);
                debug!("MUL {} {} -> {}", r0, r1, store);
                self.inc_pc(4);
            }
            Op::Input => match self.consume_input() {
                Some(value) => {
                    let store = operation.r0() as usize;

                    self.store(store, value);
                    debug!("INPUT: {} -> {}", value, store);
                    self.inc_pc(2);
                }
                None => return IntcodeResult::AwaitingInput { pc: self.pc },
            },
            Op::Output => {
                let output = operation.r0();

                debug!("OUTPUT: {}", output);
                self.inc_pc(2);
                return IntcodeResult::Output { output };
            }
            Op::Jit => {
                let r0 = operation.r0();
                let pc = if r0 != 0 {
                    let new_pc = operation.r1() as usize;
                    debug!("JIT {} (pass): PC <-- {}", r0, new_pc);
                    new_pc
                } else {
                    debug!("JIT {} (fail): PC <-- {}", r0, self.pc + 3);
                    self.pc + 3
                };
                self.set_pc(pc);
            }
            Op::Jif => {
                let r0 = operation.r0();
                let pc = if r0 == 0 {
                    let new_pc = operation.r1() as usize;
                    debug!("JIF {} (pass): PC <-- {}", r0, new_pc);
                    new_pc
                } else {
                    debug!("JIF {} (fail): PC <-- {}", r0, self.pc + 3);
                    self.pc + 3
                };
                self.set_pc(pc);
            }
            Op::Lt => {
                let r0 = operation.r0();
                let r1 = operation.r1();
                let store = operation.r2() as usize;

                self.store(store, if r0 < r1 { 1 } else { 0 });
                debug!("LT {}, {} -> {}", r0, r1, store);
                self.inc_pc(4);
            }
            Op::Eq => {
                let r0 = operation.r0();
                let r1 = operation.r1();
                let store = operation.r2() as usize;

                self.store(store, if r0 == r1 { 1 } else { 0 });
                debug!("LT {}, {} -> {}", r0, r1, store);
                self.inc_pc(4);
            }
            Op::Halt => return IntcodeResult::Halted,
        };
        return IntcodeResult::AdvanceInstruction;
    }

    pub fn mem_value(&self, mem_i: usize) -> i32 {
        self.memory[mem_i]
    }

    fn set_pc(&mut self, new_pc: usize) {
        self.pc = new_pc
    }

    fn inc_pc(&mut self, inc: usize) {
        self.pc += inc;
    }

    pub fn buffer_input(&mut self, input: i32) {
        self.input_buf.push_back(input)
    }

    fn consume_input(&mut self) -> Option<i32> {
        self.input_buf.pop_front()
    }

    fn store(&mut self, location: usize, value: i32) {
        self.memory[location] = value
    }
}

#[derive(Debug)]
pub enum IntcodeResult {
    Output { output: i32 },
    AwaitingInput { pc: usize },
    AdvanceInstruction,
    Halted,
}

fn destructure_inst(inst: i32) -> std::result::Result<Instruction, InvalidOpCodeError> {
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
        addr_modes.push(AddressingMode::Immediate);
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
        })
    }
}

#[derive(Debug)]
pub struct Instruction {
    op: Op,
    addr_modes: Vec<AddressingMode>,
}

struct Operation<'a> {
    op: &'a Op,
    r0: Option<i32>,
    r1: Option<i32>,
    r2: Option<i32>,
}

impl Operation<'_> {
    fn r0(&self) -> i32 {
        self.r0.unwrap()
    }

    fn r1(&self) -> i32 {
        self.r1.unwrap()
    }

    fn r2(&self) -> i32 {
        self.r2.unwrap()
    }
}

impl Instruction {
    fn as_operation(&self, pc: usize, memory: &Vec<i32>) -> Operation {
        let r0 = get_parameter(pc, memory, 0, &self);
        let r1 = get_parameter(pc, memory, 1, &self);
        let r2 = get_parameter(pc, memory, 2, &self);
        Operation {
            op: &self.op,
            r0,
            r1,
            r2,
        }
    }
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
    Immediate
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
pub struct InvalidOpCodeError {
    message: String,
}

impl Display for InvalidOpCodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Invalid opcode {}", self.message)
    }
}

pub fn parse_intcode_input(input: &str) -> Vec<i32> {
    input.trim()
        .split(',')
        .map(|it| it.parse::<i32>().expect("Can't parse int"))
        .collect()
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
