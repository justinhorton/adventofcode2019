use intcode::IntcodeProgram;

const DRONE_PROGRAM: &str = include_str!("../day19.txt");

const PT1_GRID_SIZE: usize = 50;

fn main() {
    println!("Day 19-1: {}", part1());
    println!("Day 19-2: {}", part2());
}

fn part1() -> usize {
    let mut count = 0;
    for x in 0..PT1_GRID_SIZE {
        for y in 0..PT1_GRID_SIZE {
            if run_program(x, y) {
                count += 1;
            }
        }
    }
    count
}

const PT2_DIAG_COUNT_GOAL: usize = 100;

fn part2() -> usize {
    let mut start_x = 200;

    loop {
        let (count, result) = get_santa_units_at_fixed_x(start_x);
        if count == PT2_DIAG_COUNT_GOAL {
            return result;
        }

        // made up heuristic for stepping more quickly through the options, but not great since it
        // requires that I know how the beam looks...but I do, so ¯\_(ツ)_/¯
        if PT2_DIAG_COUNT_GOAL - count >= 5 {
            start_x += 100;
        } else {
            start_x += 1;
        }
    }
}

fn get_santa_units_at_fixed_x(start_x: usize) -> (usize, usize) {
    let mut y_top = start_x;
    while !run_program(start_x, y_top) {
        // First, for fixed x, find the y coord where the beam first has an effect. This is our
        // potential top right corner of the area containing Santa's ship.
        y_top += 1;
    }

    let mut x = start_x;
    let mut y = y_top;
    let mut count = 0;
    loop {
        let in_beam = run_program(x, y);
        if in_beam {
            count += 1;
        }

        if count == PT2_DIAG_COUNT_GOAL {
            break;
        }

        if !in_beam {
            // out of the beam before reaching 100, this isn't the answer
            return (count, 0);
        } else {
            // Decrease x and increase y, looking for a diagonal count of 100 to signal that Santa's
            // ship can fit. This is the simplest definition of "fitting" that I could think of,
            // given that the ship is square.
            x -= 1;
            y += 1;
        }
    }

    (count, x * 10000 + y_top)
}

fn run_program(x: usize, y: usize) -> bool {
    let mut program = IntcodeProgram::init_from(DRONE_PROGRAM);

    program.buffer_input(x as i64);
    program.buffer_input(y as i64);
    program.run();

    if let Some(output) = program.consume_output() {
        if output == 1 {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 150)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 12201460)
    }
}
