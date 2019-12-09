extern crate permutate;
extern crate intcode;

use permutate::Permutator;
use std::cmp::max;
use intcode::{IntcodeProgram, IntcodeResult, parse_intcode_input};

const INPUT: &str = include_str!("../day7.txt");
const INPUT_SIGNAL: i32 = 0;
const PT1_PHASES: [&i32; 5] = [&0, &1, &2, &3, &4];
const PT2_PHASES: [&i32; 5] = [&9, &8, &7, &6, &5];

fn main() {
    let memory: Vec<i32> = parse_intcode_input(&INPUT);
    let part1 = maximum_signal(memory.clone(), false, PT1_PHASES);
    let part2 = maximum_signal(memory.clone(), true, PT2_PHASES);

    println!("Day 7-1 amp output: {}", part1);
    println!("Day 7-2 amp output: {}", part2);
}

fn maximum_signal(
    original_memory: Vec<i32>,
    with_feedback: bool,
    possible_phases: [&i32; 5],
) -> i32 {
    // TODO: Well, definitely write something better for permuting w/o repetition...
    let da_phases = possible_phases;
    let possible_phases: &[&i32] = &possible_phases;
    let possible_phases = [possible_phases];
    let mut permutator = Permutator::new(&possible_phases[..]);

    let mut max_signal: Option<i32> = None;
    while let Some(permutation) = next_non_repeating(&mut permutator, &da_phases) {
        let signal = run_all_amplifiers(&mut original_memory.clone(), &permutation, with_feedback);
        max_signal = match max_signal {
            Some(m_s) => Some(max(m_s, signal)),
            None => Some(signal),
        };
    }
    return max_signal.unwrap();
}

fn run_all_amplifiers(start_memory: &Vec<i32>, phases: &Vec<&i32>, with_feedback: bool) -> i32 {
    let mut intcode_amps: Vec<IntcodeProgram> = Vec::new();

    intcode_amps.push(IntcodeProgram::init(
        start_memory,
        vec![*phases[0], INPUT_SIGNAL],
    ));
    for i in 1..5 {
        intcode_amps.push(IntcodeProgram::init(start_memory, vec![*phases[i]]))
    }

    let mut last_output = None;
    loop {
        for amp_i in 0..5 {
            let amp_process = &mut intcode_amps[amp_i];

            match amp_process.run() {
                IntcodeResult::AwaitingInput { pc: paused_pc } => {
                    println!("Amp {} awaiting input at {}", amp_i, paused_pc);
                }
                IntcodeResult::Output { output: o } => {
                    last_output = Some(o);
                    intcode_amps[(amp_i + 1) % 5].buffer_input(o);
                }
                IntcodeResult::Halted => {
                    println!("Amp {} halted", amp_i);
                    if amp_i == 4 {
                        return last_output.expect("Something went wrong; no output to return");
                    }
                }
                _ => {}
            };
        }

        if !with_feedback {
            return last_output.expect("Something went wrong; no output to return");
        }
    }
}

fn next_non_repeating<'a>(
    permutator: &mut Permutator<'a, i32>,
    phases: &[&i32; 5],
) -> Option<Vec<&'a i32>> {
    fn contains_all_phases(permutation: &Vec<&i32>, phases: &[&i32; 5]) -> bool {
        for phase in phases {
            if !permutation.contains(phase) {
                return false;
            }
        }
        true
    }

    while let Some(permutation) = permutator.next() {
        if contains_all_phases(&permutation, phases) {
            return Some(permutation);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::{maximum_signal, PT1_PHASES, INPUT, PT2_PHASES};
    use intcode::parse_intcode_input;

    #[test]
    fn test_part1() {
        let max = maximum_signal(parse_intcode_input(INPUT), false, PT1_PHASES);
        assert_eq!(14902, max);
    }

    #[test]
    fn test_part2() {
        let max = maximum_signal(parse_intcode_input(INPUT), true, PT2_PHASES);
        assert_eq!(6489132, max);
    }
}
