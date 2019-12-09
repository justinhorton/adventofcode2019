extern crate log;

use log::debug;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};

pub struct IntcodeProgram {
    memory: Vec<i64>,
    input_buf: VecDeque<i64>,
    pc: usize,
    relative_base: i64,
}

const MAX_INTCODE_SIZE: usize = 32 * 1024; // 32KB should be enough for anyone...

impl IntcodeProgram {
    pub fn init(memory: &Vec<i64>, inputs: Vec<i64>) -> IntcodeProgram {
        let mut program_memory = vec![0; MAX_INTCODE_SIZE];
        let mut i = 0;
        for b in memory {
            program_memory[i] = *b;
            i += 1;
        }
        IntcodeProgram {
            memory: program_memory,
            input_buf: VecDeque::from(inputs),
            pc: 0,
            relative_base: 0,
        }
    }

    pub fn init_from(intcode_program: &str) -> IntcodeProgram {
        let parsed = parse_intcode_input(intcode_program);
        Self::init(&parsed, Default::default())
    }

    pub fn run(&mut self) -> IntcodeResult {
        debug!("Resuming with PC: {}", self.pc);

        loop {
            let instruction = destructure_inst(self.memory[self.pc]);
            debug!(
                "PC({}), RB({}) :: {:?}",
                self.pc, self.relative_base, instruction
            );
            match instruction {
                Ok(inst) => {
                    let operation = inst.as_operation(self.pc, self.relative_base, &self.memory);
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
                let result = r0 + r1;

                self.store(store, result);
                debug!("ADD {} {} = {} -> {}", r0, r1, result, store);
                self.inc_pc(4);
            }
            Op::Mul => {
                let store = operation.r2() as usize;
                let r0 = operation.r0();
                let r1 = operation.r1();
                let result = r0 * r1;

                self.store(store, result);
                debug!("MUL {} {} = {}  -> {}", r0, r1, result, store);
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

                let result = if r0 < r1 { 1 } else { 0 };

                self.store(store, result);
                debug!("LT {}, {} = {} -> {}", r0, r1, result, store);
                self.inc_pc(4);
            }
            Op::Eq => {
                let r0 = operation.r0();
                let r1 = operation.r1();
                let store = operation.r2() as usize;
                let result = if r0 == r1 { 1 } else { 0 };

                self.store(store, result);
                debug!("LT {}, {} = {} -> {}", r0, r1, result, store);
                self.inc_pc(4);
            }
            Op::RelBaseOffset => {
                let r0 = operation.r0();
                self.relative_base = self.relative_base + r0;
                debug!("SETRB {}", self.relative_base);
                self.inc_pc(2);
            }
            Op::Halt => return IntcodeResult::Halted,
        };
        return IntcodeResult::AdvanceInstruction;
    }

    pub fn mem_value(&self, mem_i: usize) -> i64 {
        self.memory[mem_i]
    }

    fn set_pc(&mut self, new_pc: usize) {
        self.pc = new_pc
    }

    fn inc_pc(&mut self, inc: usize) {
        self.pc += inc;
    }

    pub fn buffer_input(&mut self, input: i64) {
        self.input_buf.push_back(input)
    }

    fn consume_input(&mut self) -> Option<i64> {
        self.input_buf.pop_front()
    }

    fn store(&mut self, location: usize, value: i64) {
        self.memory[location] = value
    }
}

#[derive(Debug)]
pub enum IntcodeResult {
    Output { output: i64 },
    AwaitingInput { pc: usize },
    AdvanceInstruction,
    Halted,
}

fn destructure_inst(inst: i64) -> std::result::Result<Instruction, InvalidOpCodeError> {
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
    relative_base: i64,
    memory: &Vec<i64>,
    parm_index: usize, // 0-indexed
    inst: &Instruction,
) -> Option<i64> {
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
            AddressingMode::Relative => {
                let pos = memory[parm_slot] + relative_base;
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
    r0: Option<i64>,
    r1: Option<i64>,
    r2: Option<i64>,
}

impl Operation<'_> {
    fn r0(&self) -> i64 {
        self.r0.unwrap()
    }

    fn r1(&self) -> i64 {
        self.r1.unwrap()
    }

    fn r2(&self) -> i64 {
        self.r2.unwrap()
    }
}

impl Instruction {
    fn as_operation(&self, pc: usize, relative_base: i64, memory: &Vec<i64>) -> Operation {
        let r0 = get_parameter(pc, relative_base, memory, 0, &self);
        let r1 = get_parameter(pc, relative_base, memory, 1, &self);
        let r2 = get_parameter(pc, relative_base, memory, 2, &self);
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
    RelBaseOffset,
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
            9 => Some(Op::RelBaseOffset),
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
            Op::Input | Op::Output | Op::RelBaseOffset => 1,
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

#[derive(Debug, PartialEq)]
enum AddressingMode {
    Position,
    Immediate,
    Relative,
}

impl AddressingMode {
    fn from(i: Option<i8>) -> Result<AddressingMode, InvalidOpCodeError> {
        match i {
            Some(0) | None => Ok(AddressingMode::Position),
            Some(1) => Ok(AddressingMode::Immediate),
            Some(2) => Ok(AddressingMode::Relative),
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

pub fn parse_intcode_input(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|it| it.parse::<i64>().expect("Can't parse int"))
        .collect()
}

fn digits(num: i64) -> VecDeque<i8> {
    let mut digits: VecDeque<i8> = VecDeque::new();
    let mut value = num;
    while value > 0 {
        digits.push_front((value % 10) as i8);
        value = value / 10;
    }
    digits
}

#[cfg(test)]
mod tests {
    use crate::{destructure_inst, AddressingMode, IntcodeProgram, IntcodeResult};

    #[test]
    fn test_parse_relative_mode() {
        let mode = AddressingMode::from(Some(2));
        assert!(mode.is_ok());
        assert_eq!(mode.unwrap(), AddressingMode::Relative);
    }

    #[test]
    fn test_initial_relative_base() {
        let program = IntcodeProgram::init(&Vec::new(), Vec::new());
        assert_eq!(program.relative_base, 0)
    }

    #[test]
    fn test_run_relative_base() {
        let mut program = IntcodeProgram::init_from("109,19,99");
        program.relative_base = 2000;
        program.run();
        assert_eq!(program.relative_base, 2019);
    }

    #[test]
    fn test_run_negative_relative_base() {
        let mut program = IntcodeProgram::init_from("109,19,99");
        program.relative_base = -20;
        program.run();
        assert_eq!(program.relative_base, -1);
    }

    #[test]
    fn test_run_rel_base_ex1() {
        let intcode = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let mut program = IntcodeProgram::init_from(intcode);
        let mut out_buf = String::new();
        while let IntcodeResult::Output { output: o } = program.run() {
            out_buf.push_str(format!("{},", o).as_str());
        }
        out_buf.remove(out_buf.len() - 1);
        assert_eq!(out_buf, intcode);
    }

    #[test]
    fn test_run_rel_base_ex2() {
        let intcode = "1102,34915192,34915192,7,4,7,99,0";
        let mut program = IntcodeProgram::init_from(intcode);
        let mut out_buf = String::new();
        while let IntcodeResult::Output { output: o } = program.run() {
            out_buf.push_str(format!("{}", o).as_str());
        }
        assert_eq!(out_buf.len(), 16)
    }

    #[test]
    fn test_run_rel_base_ex3() {
        let intcode = "104,1125899906842624,99";
        let mut program = IntcodeProgram::init_from(intcode);
        let mut out_buf = String::new();
        while let IntcodeResult::Output { output: o } = program.run() {
            out_buf.push_str(format!("{}", o).as_str());
        }
        assert_eq!(out_buf, "1125899906842624")
    }

    #[test]
    fn test_destructure_inst() {
        let inst = 203;
        let x = destructure_inst(inst);
        assert!(x.is_ok());
        println!("{:?}", x.unwrap());
    }
}
