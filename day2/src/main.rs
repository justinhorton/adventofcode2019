const INPUT: &str = include_str!("../day2.txt");

fn main() {
    let mem0 = calc_day2(12, 2);
    println!("Day 2-1: {}", mem0);
    println!("Day 2-2: {}", day2_pt2().expect("No answer found!"))
}

fn day2_pt2() -> Option<i32> {
    for n in 0..99 {
        for v in 0..99 {
            let result = calc_day2(n, v);
            if result == 19690720 {
                return Some(100 * n + v);
            }
        }
    }
    None
}

// OPCODE 1: 1,op1,op2,dest; set dest = *op1 + *op2
// OPCODE 2: 2,op1,op2,dest; set dest = *op1 * *op2
// OPCODE 99: halt
// OPCODE anything else: error
fn calc_day2(noun: i32, verb: i32) -> i32 {
    let input: Vec<&str> = INPUT.trim().split(',').collect();
    let mut memory: Vec<i32> = input
        .iter()
        .map(|it| it.parse::<i32>().expect("Can't parse int"))
        .collect();

    // restore the state at the time of the elf problem
    memory[1] = noun;
    memory[2] = verb;

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
            }
            99 => return memory[0],
            _ => panic!(),
        };
        pos += 4;
    }
    memory[0]
}
