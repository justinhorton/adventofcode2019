extern crate intcode;
extern crate log;
extern crate permutate;

use intcode::{parse_intcode_input, IntcodeProgram};
use log::debug;
use permutate::Permutator;
use std::cmp::max;

const INPUT: &str = include_str!("../day7.txt");
const INPUT_SIGNAL: i64 = 0;
const PT1_PHASES: [&i64; 5] = [&0, &1, &2, &3, &4];
const PT2_PHASES: [&i64; 5] = [&9, &8, &7, &6, &5];

fn main() {
    let memory: Vec<i64> = parse_intcode_input(&INPUT);
    let part1 = maximum_signal(memory.clone(), false, PT1_PHASES);
    let part2 = maximum_signal(memory.clone(), true, PT2_PHASES);

    println!("Day 7-1 amp output: {}", part1);
    println!("Day 7-2 amp output: {}", part2);
}

fn maximum_signal(
    original_memory: Vec<i64>,
    with_feedback: bool,
    possible_phases: [&i64; 5],
) -> i64 {
    // TODO: Well, definitely write something better for permuting w/o repetition...
    let da_phases = possible_phases;
    let possible_phases: &[&i64] = &possible_phases;
    let possible_phases = [possible_phases];
    let mut permutator = Permutator::new(&possible_phases[..]);

    let mut max_signal: Option<i64> = None;
    while let Some(permutation) = next_non_repeating(&mut permutator, &da_phases) {
        let signal = run_all_amplifiers(&mut original_memory.clone(), &permutation, with_feedback);
        max_signal = match max_signal {
            Some(m_s) => Some(max(m_s, signal)),
            None => Some(signal),
        };
    }
    return max_signal.unwrap();
}

fn run_all_amplifiers(start_memory: &Vec<i64>, phases: &Vec<&i64>, with_feedback: bool) -> i64 {
    let mut intcode_amps: Vec<IntcodeProgram> = Vec::new();

    intcode_amps.push(IntcodeProgram::init(
        start_memory,
        vec![*phases[0], INPUT_SIGNAL],
    ));
    for i in 1..5 {
        intcode_amps.push(IntcodeProgram::init(start_memory, vec![*phases[i]]))
    }

    let mut last_output = None;
    return if with_feedback {
        while let Some(output) = run_single_amp_loop(&mut intcode_amps) {
            last_output = Some(output);
        }
        last_output.expect("No output from feedback loop")
    } else {
        run_single_amp_loop(&mut intcode_amps).expect("No output from loop")
    };
}

fn run_single_amp_loop(intcode_amps: &mut Vec<IntcodeProgram>) -> Option<i64> {
    let mut last_output = None;

    for amp_i in 0..5 {
        let amp_program = &mut intcode_amps[amp_i];
        amp_program.run();
        match amp_program.consume_output() {
            Some(amp_output) => {
                let next_amp_i = (amp_i + 1) % 5;

                intcode_amps[next_amp_i].buffer_input(amp_output);
                debug!("Amp {}: {} -> Amp {}", amp_i, amp_output, next_amp_i);
                last_output = Some(amp_output);
            }
            None => {}
        }
    }

    last_output
}

fn next_non_repeating<'a>(
    permutator: &mut Permutator<'a, i64>,
    phases: &[&i64; 5],
) -> Option<Vec<&'a i64>> {
    fn contains_all_phases(permutation: &Vec<&i64>, phases: &[&i64; 5]) -> bool {
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
    use crate::{maximum_signal, INPUT, PT1_PHASES, PT2_PHASES};
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
