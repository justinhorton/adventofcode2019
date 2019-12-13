use intcode::{parse_intcode_input, IntcodeProgram};
use std::io;
use std::thread::sleep;
use std::time::Duration;

const GAME_PROGRAM: &str = include_str!("../day13.txt");

const SCREEN_WIDTH: usize = 45;
const SCREEN_HEIGHT: usize = 24;
const TILE_EMPTY: i64 = 0;
const TILE_WALL: i64 = 1; // indestructible
const TILE_BLOCK: i64 = 2; // can be broken by ball
const TILE_HORIZ_PADDLE: i64 = 3; // indestructible
const TILE_BALL: i64 = 4; // moves and bounces off of things

const NUM_QTRS: i64 = 2;

const JOY_NEUT: i64 = 0;
const JOY_LEFT: i64 = -1;
const JOY_RIGHT: i64 = 1;

fn main() {
    part1();
    sleep(Duration::from_secs(2));
    part2();
}

fn part1() {
    let (num_blocks, _) = run_game(false, false);
    println!("Day 13-1: {} blocks on screen", num_blocks);
}

fn part2() {
    let (_, score) = run_game(false, true);
    println!("Day 13-2: Score={}", score);
}

fn run_game(headless: bool, cheat_to_win: bool) -> (i64, i64) {
    // blocks remaining, score
    let mut memory = parse_intcode_input(GAME_PROGRAM);

    if cheat_to_win {
        memory[0] = NUM_QTRS;

        // And since we're modifying memory to play for free, anyway...
        //
        // There's one instruction each loop to check whether the ball's y coordinate is past the
        // bottom of the screen. Hack the program to pretend that it never is:
        //
        // memory[365..=368] = 1007,389,23,381 => LT m[389], 23 -> m[381]
        // >> change to........1107,0,23,381   => LT 0, 23 -> m[381] => 1 -> m[381]
        //
        // If we store 0 in memory[381] each time this comparison executes, the game ignores the
        // paddle position until we reach the win condition.
        memory[365] = 1107;
        memory[366] = 0;
    }

    let mut program = IntcodeProgram::init(&memory, Vec::new());

    let mut player_score: i64 = 0;
    let mut game_grid = vec![vec![TILE_EMPTY; SCREEN_WIDTH]; SCREEN_HEIGHT];

    let mut blocks_remaining = 0;

    while !program.is_halted() {
        program.run();

        // x, y, tile id
        // consume any outputs first, until they're exhausted
        while let (Some(x), Some(y), Some(value)) = (
            program.consume_output(),
            program.consume_output(),
            program.consume_output(),
        ) {
            match (x, y) {
                (-1, 0) => {
                    // segment display
                    player_score = value;
                }
                _ => {
                    // tile
                    let old_tile = &mut game_grid[y as usize][x as usize];
                    if value == TILE_BLOCK {
                        blocks_remaining += 1;
                    } else if value != TILE_BLOCK && *old_tile == TILE_BLOCK {
                        blocks_remaining -= 1;
                    }
                    *old_tile = value;
                }
            }
        }

        if !headless {
            for y in 0..SCREEN_HEIGHT {
                for x in 0..SCREEN_WIDTH {
                    let grid_it = game_grid[y][x];
                    let c = match grid_it {
                        TILE_EMPTY => ' ',
                        TILE_WALL => '|',
                        TILE_BLOCK => '#',
                        TILE_HORIZ_PADDLE => '_',
                        TILE_BALL => '*',
                        _ => panic!("Bad tile"),
                    };
                    print!("{}", c)
                }
                println!();
            }
            println!("BLOCKS REMAINING: {}", blocks_remaining);
            println!("SCORE: {}", player_score);
        }

        // provide updated input when requested
        if program.is_awaiting_input() {
            let input_val = if cheat_to_win {
                JOY_NEUT
            } else {
                let mut buf: String = String::new();
                io::stdin().read_line(&mut buf).unwrap();
                let ch = buf.trim().chars().next();
                as_input_num(ch.unwrap())
            };
            program.buffer_input(input_val);
        }
    }

    (blocks_remaining, player_score)
}

fn as_input_num(ch: char) -> i64 {
    match ch {
        'a' => JOY_LEFT,
        'd' => JOY_RIGHT,
        's' => JOY_NEUT,
        _ => panic!("bad input!!!"),
    }
}

#[cfg(test)]
mod tests {
    use crate::run_game;

    #[test]
    fn test_part1_with_cheat() {
        let (blocks, _) = run_game(true, false);
        assert_eq!(blocks, 462)
    }

    #[test]
    fn test_part2_with_cheat() {
        let (_, score) = run_game(true, true);
        assert_eq!(score, 23981)
    }
}
