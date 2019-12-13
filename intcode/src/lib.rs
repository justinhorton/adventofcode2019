extern crate log;

use log::debug;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct IntcodeProgram {
    memory: Vec<i64>,
    input_buf: VecDeque<i64>,
    output_buf: VecDeque<i64>,
    pc: usize,
    relative_base: i64,
    is_halted: bool,
    is_awaiting_input: bool,
}

const MAX_INTCODE_SIZE: usize = 32 * 1024; // 32KB should be enough for anyone...

impl IntcodeProgram {
    pub fn set_pc(&mut self, new_pc: usize) {
        self.pc = new_pc;
    }

    pub fn init(memory: &Vec<i64>, inputs: Vec<i64>) -> IntcodeProgram {
        // allow MAX_INTCODE_SIZE memory space, initalized to 0
        let mut program_memory = vec![0; MAX_INTCODE_SIZE];
        program_memory[..memory.len()].clone_from_slice(&memory[..]);

        IntcodeProgram {
            memory: program_memory,
            input_buf: VecDeque::from(inputs),
            output_buf: VecDeque::new(),
            pc: 0,
            relative_base: 0,
            is_halted: false,
            is_awaiting_input: false,
        }
    }

    pub fn init_from(intcode_program: &str) -> IntcodeProgram {
        let parsed = parse_intcode_input(intcode_program);
        Self::init(&parsed, Default::default())
    }

    // TODO: Improve running w.r.t. halting, blocking for input, etc.
    pub fn run(&mut self) {
        debug!("Resuming with PC: {}", self.pc);

        // instruction loop: continue until blocking to wait for input or the program halts
        loop {
            let instruction = destructure_inst(self.memory[self.pc]);
            debug!(
                "PC({}), RB({}) :: {:?}",
                self.pc, self.relative_base, instruction
            );
            match instruction {
                Ok(inst) => {
                    // TODO: Clean up the operation/instruction separation (or remove it...) and
                    //   add back verbose debug logging for execution values.
                    let operation = inst.as_operation(self.pc, self.relative_base, &self.memory);
                    let result = self.apply(&operation);
                    match result {
                        IntcodeResult::AwaitingInput | IntcodeResult::Halted => return,
                        IntcodeResult::ExecutedInstruction => {}
                    }
                }
                Err(e) => panic!(format!("Aborting, invalid opcode: {:?}", e)),
            }
        }
    }

    fn apply(&mut self, operation: &Operation) -> IntcodeResult {
        // init with default that will throw out of bounds if we access the wrong input
        // TODO: do this more cleanly
        let mut r: [usize; 3] = [MAX_INTCODE_SIZE + 1; 3];

        for i in 0..operation.op.num_parameters() {
            r[i] = operation.slots[i].unwrap() as usize;
        }

        match operation.op {
            Op::Add => {
                self.store(r[2], self.memory[r[0]] + self.memory[r[1]]);
                self.inc_pc(4);
            }
            Op::Mul => {
                let dst = r[2];
                let r1 = self.memory[r[0]];
                let r2 = self.memory[r[1]];

                self.store(dst, r1 * r2);
                self.inc_pc(4);

                debug!("+ MUL {}, {} -> m[{}]", dst, r1, r2);
            }
            Op::Input => match self.consume_input() {
                Some(value) => {
                    let dst = r[0];
                    self.is_awaiting_input = false;

                    self.store(dst, value);
                    self.inc_pc(2);

                    debug!("<< INPUT {} -> m[{}]", value, dst);
                }
                None => {
                    debug!("Waiting for INPUT...");
                    self.is_awaiting_input = true;
                    return IntcodeResult::AwaitingInput;
                }
            },
            Op::Output => {
                let dst = r[0];
                let output = self.memory[dst];

                debug!(">> OUTPUT m[{}] >> {}", dst, output);

                self.buffer_output(output);
                self.inc_pc(2);
            }
            Op::Jit => {
                let pred = self.memory[r[0]];
                let pc = if pred != 0 {
                    let new_pc = self.memory[r[1]] as usize;
                    debug!("+ JIT {}...pass -> pc={}", pred, new_pc);
                    new_pc
                } else {
                    debug!("+ JIT {}...fail", pred);
                    self.pc + 3
                };
                self.pc = pc;
            }
            Op::Jif => {
                let pred = self.memory[r[0]];
                let pc = if pred == 0 {
                    let new_pc = self.memory[r[1]] as usize;
                    debug!("+ JIF {}...pass -> pc={}", pred, new_pc);
                    new_pc
                } else {
                    debug!("+ JIF {}...fail", pred);
                    self.pc + 3
                };
                self.pc = pc;
            }
            Op::Lt => {
                // TODO: Show addressing modes properly for debug output
                debug!("+ LT m[{}], m[{}]", r[0], r[1]);
                let r0 = self.memory[r[0]];
                let r1 = self.memory[r[1]];
                let dst = r[2];
                let result = if r0 < r1 { 1 } else { 0 };

                self.store(dst, result);
                self.inc_pc(4);
                debug!("++ LT {}, {} -> m[{}]", r0, r1, dst);
            }
            Op::Eq => {
                let r0 = self.memory[r[0]];
                let r1 = self.memory[r[1]];
                let dst = r[2];
                let result = if r0 == r1 { 1 } else { 0 };

                self.store(dst, result);
                self.inc_pc(4);
                debug!("+ LT {}, {} -> m[{}]", r0, r1, dst);
            }
            Op::RelBaseOffset => {
                self.relative_base = self.relative_base + self.memory[r[0]];
                self.inc_pc(2);
                debug!("+ SETRB {}", self.relative_base);
            }
            Op::Halt => {
                self.is_halted = true;
                debug!("+ HALT");
                return IntcodeResult::Halted;
            }
        };

        IntcodeResult::ExecutedInstruction
    }

    pub fn mem_value(&self, mem_i: usize) -> i64 {
        self.memory[mem_i]
    }

    pub fn buffer_input(&mut self, input: i64) {
        self.input_buf.push_back(input)
    }

    pub fn is_awaiting_input(&self) -> bool {
        self.is_awaiting_input
    }

    pub fn consume_output(&mut self) -> Option<i64> {
        self.output_buf.pop_front()
    }

    pub fn is_halted(&self) -> bool {
        self.is_halted
    }

    fn inc_pc(&mut self, inc: usize) {
        self.pc += inc;
    }

    fn consume_input(&mut self) -> Option<i64> {
        self.input_buf.pop_front()
    }

    fn buffer_output(&mut self, output: i64) {
        self.output_buf.push_back(output)
    }

    fn store(&mut self, location: usize, value: i64) {
        self.memory[location] = value
    }
}

#[derive(Debug)]
enum IntcodeResult {
    AwaitingInput,
    Halted,
    ExecutedInstruction,
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
            AddressingMode::Immediate => parm_slot as i64,
            AddressingMode::Position => memory[parm_slot],
            AddressingMode::Relative => memory[parm_slot] + relative_base,
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
    use crate::{destructure_inst, AddressingMode, IntcodeProgram, Op};

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
        program.run();
        assert_eq!(
            program.output_buf,
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
        );
    }

    #[test]
    fn test_run_rel_base_ex2() {
        let intcode = "1102,34915192,34915192,7,4,7,99,0";
        let mut program = IntcodeProgram::init_from(intcode);
        program.run();
        assert_eq!(program.output_buf.len(), 1);
        let output = program.consume_output().unwrap();
        assert_eq!(output.to_string().len(), 16)
    }

    #[test]
    fn test_run_rel_base_ex3() {
        let intcode = "104,1125899906842624,99";
        let mut program = IntcodeProgram::init_from(intcode);
        let expected = [program.memory[1]];
        program.run();
        assert_eq!(program.output_buf, expected)
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
        program.run();
        assert_eq!(program.consume_output().unwrap(), 1111);
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
