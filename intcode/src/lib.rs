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
        let mut r: [usize; 3] = [MAX_INTCODE_SIZE + 1; 3]; // init with default that will throw out
                                                           // of bounds if we access the wrong input...can do this more cleanly
        for i in 0..operation.op.num_parameters() {
            r[i] = operation.slots[i].unwrap() as usize;
        }

        match operation.op {
            Op::Add => {
                let result = self.memory[r[0]] + self.memory[r[1]];
                self.store(r[2] as usize, result);

                //                debug!("ADD {} {} = {} -> {}", r0, r1, result, r2);
                self.inc_pc(4);
            }
            Op::Mul => {
                let result = self.memory[r[0]] * self.memory[r[1]];
                self.store(r[2] as usize, result);
                //                debug!("MUL {} {} = {}  -> {}", r0, r1, result, store);
                self.inc_pc(4);
            }
            Op::Input => match self.consume_input() {
                Some(value) => {
                    self.store(r[0] as usize, value);
                    //                    debug!("INPUT: {} -> {}", value, store);
                    self.inc_pc(2);
                }
                None => return IntcodeResult::AwaitingInput { pc: self.pc },
            },
            Op::Output => {
                //                debug!("Resolved output: {}", output);
                //                debug!("Parameter: {}", self.pc + 1);
                //                debug!("OUTPUT: {}", output);
                let output = self.memory[r[0]];
                self.inc_pc(2);
                return IntcodeResult::Output { output };
            }
            Op::Jit => {
                let pc = if self.memory[r[0]] != 0 {
                    let new_pc = self.memory[r[1]] as usize;
                    //                    debug!("JIT {} (pass): PC <-- {}", r0, new_pc);
                    new_pc
                } else {
                    //                    debug!("JIT {} (fail): PC <-- {}", r0, self.pc + 3);
                    self.pc + 3
                };
                self.set_pc(pc);
            }
            Op::Jif => {
                //                let r0 = operation.r0();
                let pc = if self.memory[r[0]] == 0 {
                    let new_pc = self.memory[r[1]] as usize;
                    //                    debug!("JIF {} (pass): PC <-- {}", r0, new_pc);
                    new_pc
                } else {
                    //                    debug!("JIF {} (fail): PC <-- {}", r0, self.pc + 3);
                    self.pc + 3
                };
                self.set_pc(pc);
            }
            Op::Lt => {
                let r0 = self.memory[r[0]];
                let r1 = self.memory[r[1]];
                let store = r[2] as usize;

                let result = if r0 < r1 { 1 } else { 0 };

                self.store(store, result);
                //                debug!("LT {}, {} = {} -> {}", r0, r1, result, store);
                self.inc_pc(4);
            }
            Op::Eq => {
                let r0 = self.memory[r[0]];
                let r1 = self.memory[r[1]];
                let store = r[2] as usize;
                let result = if r0 == r1 { 1 } else { 0 };

                self.store(store, result);
                //                debug!("LT {}, {} = {} -> {}", r0, r1, result, store);
                self.inc_pc(4);
            }
            Op::RelBaseOffset => {
                let r0 = self.memory[r[0]];
                self.relative_base = self.relative_base + r0;
                //                debug!("RBO {} (from {}) = {}", r0, self.pc + 1, self.relative_base);
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
    for _i in 0..op.num_parameters() {
        addr_modes.push(AddressingMode::from(digits.pop_back())?);
    }

    Ok(Instruction { op, addr_modes })
}

fn get_parameter_mem_slot(
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
            AddressingMode::Immediate => {
                //                let p = memory[parm_slot];
                //                debug!("p: {} from m[{}]", p, parm_slot);
                parm_slot as i64
            }
            AddressingMode::Position => {
                let pos = memory[parm_slot];
                pos
                //                let p = memory[pos as usize];
                //                debug!("p: {} from m[m[{}]] = m[{}]", p, parm_slot, pos);
                //                p
            }
            AddressingMode::Relative => {
                let i = memory[parm_slot];
                let pos = i + relative_base;
                pos
                //                let p = memory[pos as usize];
                //                debug!("p: {} from m[m[{}]] = m[{} + {}] = m[{}]", p, parm_slot, i, relative_base, pos);
                //                p
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
    slots: Vec<Option<i64>>,
}

impl Instruction {
    fn as_operation(&self, pc: usize, relative_base: i64, memory: &Vec<i64>) -> Operation {
        let mut slots = Vec::new();
        for i in 0..3 {
            slots.push(get_parameter_mem_slot(pc, relative_base, memory, i, &self));
        }
        Operation {
            op: &self.op,
            slots,
        }
    }
}

#[derive(Debug, PartialEq)]
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
    use crate::{destructure_inst, AddressingMode, IntcodeProgram, IntcodeResult, Op};

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

    #[test]
    fn test_another_relative_base_ex() {
        /*
        For example, if the relative base is 2000, then after the instruction 109,19, the relative
        base would be 2019. If the next instruction were 204,-34, then the value at address 1985 would be output.
        */
        let rel_base = 2000;
        let mut program = IntcodeProgram::init_from("109,19,204,-34,99");
        program.relative_base = rel_base;
        program.memory[1985] = 1111;
        let result = program.run();
        let out = match result {
            IntcodeResult::Output { output: o } => o,
            _ => panic!(),
        };
        assert_eq!(out, 1111);
    }

    #[test]
    fn test_decode_relative_address_mode_for_input_store() {
        let instr = 203;
        let x = destructure_inst(instr);
        assert!(x.is_ok());
        let result = x.unwrap();
        assert_eq!(result.op, Op::Input);
        let addr_mode = result.addr_modes.get(0);
        assert!(addr_mode.is_some());
        assert_eq!(*addr_mode.unwrap(), AddressingMode::Relative);
    }
}
