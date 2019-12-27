extern crate intcode;
extern crate itertools;

use intcode::IntcodeProgram;
use itertools::Itertools;

const PROGRAM: &str = include_str!("../day25.txt");
const PICK_UP_ALL: &str = include_str!("../pick-up-all.txt");
const SAFE_ITEMS: [&str; 8] = [
    "space law space brochure",
    "polygon",
    "astrolabe",
    "hologram",
    "prime number",
    "weather machine",
    "manifold",
    "mouse",
];

fn main() {
    println!(
        "Day 25-1: Successfully bypassed Santa's security. Message:\n\n{}",
        brute_force_santa_password(false).expect("Did not find password")
    );
}

fn brute_force_santa_password(show_output: bool) -> Option<String> {
    let program = &mut IntcodeProgram::init_from(PROGRAM);

    // start the program
    program.run();
    consume_all_output(program, show_output);

    // manually determined instructions to pick up all items and navigate to the room just before
    // the security checkpoint
    let pick_up_all_instrs: Vec<String> = PICK_UP_ALL
        .trim()
        .lines()
        .map(|line| {
            let mut string = line.to_string();
            string.push('\n');
            string
        })
        .collect();
    pick_up_all_instrs.iter().for_each(|instr| {
        do_move(program, instr, show_output);
    });

    for combo in all_item_combos() {
        match try_combo_against_security(program, combo, show_output) {
            Some(result) => return Some(result),
            None => continue,
        }
    }

    None
}

fn all_item_combos() -> Vec<Vec<&'static &'static str>> {
    let mut all_item_combos = Vec::new();
    for k in 1..=SAFE_ITEMS.len() {
        let combos_iter = SAFE_ITEMS.iter().combinations(k);
        combos_iter.for_each(|combo| all_item_combos.push(combo));
    }
    all_item_combos
}

fn try_combo_against_security(
    program: &mut IntcodeProgram,
    item_combo: Vec<&&str>,
    show_output: bool,
) -> Option<String> {
    // drop all items (program will complain that we don't have some of them, but it's easier than
    // checking the inventory or tracking the last combo)
    drop_all_items(program, show_output);

    // pick up the item combo
    item_combo.iter().for_each(|item| {
        let instr = format!("take {}\n", item).to_string();
        if show_output {
            print!("{}", instr);
        }
        do_move(program, &instr, show_output);
    });

    // security checkpoint is east of the penultimate room
    let output = do_move(program, "east\n", show_output);

    if output.contains("Analyzing...")
        && !(output.contains("heavier") || output.contains("lighter"))
    {
        // security complaints seem to either complain that droids on the ship are heavier or
        // lighter than ours...so when we're in the right place and there is neither complaint,
        // that should be the password
        return Some(output);
    }
    None
}

fn do_move(program: &mut IntcodeProgram, input_str: &str, show_output: bool) -> String {
    input_str
        .chars()
        .map(|c| c as i64)
        .for_each(|ascii_val| program.buffer_input(ascii_val));

    program.run();

    consume_all_output(program, show_output)
}

fn drop_all_items(program: &mut IntcodeProgram, show_output: bool) {
    for item in SAFE_ITEMS.iter() {
        do_move(program, format!("drop {}\n", item).as_str(), show_output);
    }
}

fn consume_all_output(program: &mut IntcodeProgram, show_output: bool) -> String {
    let mut buf = String::new();
    while let Some(o) = program.consume_output() {
        buf.push(std::char::from_u32(o as u32).unwrap());
    }

    if show_output {
        print!("{}", buf);
    }
    buf
}

#[cfg(test)]
mod tests {
    use crate::brute_force_santa_password;

    #[test]
    fn test_it() {
        let result = brute_force_santa_password(false).unwrap();
        assert!(result.contains("537165825"));
    }
}
