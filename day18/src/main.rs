use std::collections::{HashSet, VecDeque};

const INPUT: &str = include_str!("../day18.txt");
const INPUT_2_1: &str = include_str!("../day18-2-ul.txt");
const INPUT_2_2: &str = include_str!("../day18-2-ur.txt");
const INPUT_2_3: &str = include_str!("../day18-2-bl.txt");
const INPUT_2_4: &str = include_str!("../day18-2-br.txt");

fn main() {
    println!("Day 18-1: Steps to all keys: {}", part1(INPUT));
    println!("Day 18-2: Steps to all keys: {}", part2());
}

fn part1(input: &str) -> i32 {
    let maze = Maze::create_from_input(input);
    single_robot_steps_to_find_all_keys(&maze, 0)
}

// This solution doesn't work for the following example:
//
// #############
// #g#f.D#..h#l#
// #F###e#E###.#
// #dCba@#@BcIJ#
// #############
// #nK.L@#@G...#
// #M###N#H###.#
// #o#m..#i#jk.#
// #############
//
// This maze has a shortest path that at one point has a robot reaching a door and backtracking to
// retrieve a key for a robot in another quadrant. The main input doesn't require that. It turns
// out that for this input, we can treat the quadrants as four independent mazes. We assume that
// when a robot 1 in quadrant 1 reaches a door whose key is not in its quadrant, the key will be
// found by another robot X in quadrant X without requiring robot 1 to move to unblock the other
// robot. This is equivalent to giving each robot the keys that exist in other quadrants.
//
// Turns out this gives the right answer, but is certainly an incorrect general solution.
fn part2() -> i32 {
    let mazes = [
        Maze::create_from_input(INPUT_2_1),
        Maze::create_from_input(INPUT_2_2),
        Maze::create_from_input(INPUT_2_3),
        Maze::create_from_input(INPUT_2_4),
    ];

    mazes
        .iter()
        .enumerate()
        .map(|(maze_num, maze)| {
            single_robot_steps_to_find_all_keys(maze, keys_from_others(maze_num, &mazes))
        })
        .sum()
}

fn keys_from_others(maze_num: usize, mazes: &[Maze; 4]) -> u32 {
    mazes
        .iter()
        .enumerate()
        .filter(|(i, _)| maze_num != *i)
        .fold(0, |acc: u32, (_, maze)| acc | maze.encoded_keys)
}

fn single_robot_steps_to_find_all_keys(maze: &Maze, start_keys: u32) -> i32 {
    let mut visited: HashSet<PositionWithKeys> = HashSet::new();
    let mut visit_queue: VecDeque<PositionWithKeys> = VecDeque::new();
    visit_queue.push_back(PositionWithKeys::create_with_keys(
        maze.start_positions[0].x,
        maze.start_positions[0].y,
        start_keys,
        None,
    ));

    // goal is the keys we started with plus what's available in this maze
    let goal_keys = start_keys | maze.encoded_keys;

    let mut num_steps = 0;
    while !visit_queue.is_empty() {
        let size = visit_queue.len();
        num_steps += 1;

        // every alternative in the queue when the inner loop begins is the same depth into the
        // maze, 1 step deeper than the prior iteration of the outer loop
        for _i in 0..size {
            let cur_pos_and_keys = visit_queue.pop_front().unwrap();

            for (candidate_pos, candidate_tile) in
                maze.adjacent_positions(cur_pos_and_keys.x as i32, cur_pos_and_keys.y as i32)
            {
                let mut new_key = None;
                match candidate_tile {
                    Tile::Door { key_id } => {
                        if !cur_pos_and_keys.has_key(key_id) {
                            // we don't have this key, move to the next alternative
                            continue;
                        }
                    }
                    Tile::Key { key_id } => {
                        // found a new key, we'll add it to the candidate PosWithKeys's keys
                        new_key = Some(key_id);
                    }
                    Tile::Empty => {}
                    Tile::Wall => panic!("Maze should not return a wall as adjacent"),
                }

                // next position + keys to check, possibly with a new key just found
                let next_pos_with_keys = PositionWithKeys::create_with_keys(
                    candidate_pos.x,
                    candidate_pos.y,
                    cur_pos_and_keys.encoded_keys,
                    new_key,
                );
                if next_pos_with_keys.is_goal(goal_keys) {
                    return num_steps;
                }

                if !visited.contains(&next_pos_with_keys) {
                    visited.insert(next_pos_with_keys);
                    visit_queue.push_back(next_pos_with_keys);
                }
            }
        }
    }

    // no path that finds all keys
    return -1;
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct PositionWithKeys {
    x: usize,
    y: usize,
    encoded_keys: u32,
}

impl PositionWithKeys {
    fn create_with_keys(
        x: usize,
        y: usize,
        encoded_keys: u32,
        new_key: Option<u32>,
    ) -> PositionWithKeys {
        let mut pk = PositionWithKeys { x, y, encoded_keys };
        if let Some(new_key) = new_key {
            pk.add_key(new_key);
        }
        pk
    }

    fn add_key(&mut self, key_id: u32) {
        self.encoded_keys |= 1 << key_id;
    }

    fn has_key(&self, key_id: u32) -> bool {
        (self.encoded_keys >> key_id) & 1 == 1
    }

    fn is_goal(&self, goal: u32) -> bool {
        self.encoded_keys == goal
    }
}

struct Maze {
    cells: Vec<Vec<Tile>>,
    start_positions: Vec<Position>,
    encoded_keys: u32,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl Maze {
    fn create_from_input(input: &str) -> Maze {
        let mut cells: Vec<Vec<Tile>> = Vec::new();
        let mut encoded_keys: u32 = 0;

        let mut start_positions = Vec::new();
        for (y, row) in input.trim().lines().map(|r| r.trim()).enumerate() {
            let mut row_vec = Vec::new();
            for (x, row_c) in row.chars().enumerate() {
                if row_c == Tile::CH_ROBOT {
                    start_positions.push(Position { x, y });
                    row_vec.push(Tile::Empty);
                } else {
                    row_vec.push(Tile::from_char(row_c));
                }

                if let Tile::Key { key_id: n } = row_vec.last().unwrap() {
                    // store accessible keys as integer with every key_id bit set
                    // (where key_id is 0-indexed, starting with 'a')
                    encoded_keys |= (1 << n) as u32;
                }
            }
            cells.push(row_vec);
        }

        Maze {
            cells,
            start_positions,
            encoded_keys,
        }
    }

    fn adjacent_positions(&self, x: i32, y: i32) -> Vec<(Position, Tile)> {
        let mut result = Vec::new();
        self.add_tile(&mut result, x - 1, y);
        self.add_tile(&mut result, x + 1, y);
        self.add_tile(&mut result, x, y - 1);
        self.add_tile(&mut result, x, y + 1);
        result
    }

    fn add_tile(&self, result: &mut Vec<(Position, Tile)>, x: i32, y: i32) {
        match self.tile_at(x, y) {
            // only include in-bounds non-walls
            Some(Tile::Wall) | None => {}
            t => result.push((
                Position {
                    x: x as usize,
                    y: y as usize,
                },
                *t.unwrap(),
            )),
        };
    }

    fn tile_at(&self, x: i32, y: i32) -> Option<&Tile> {
        if x < 0 || y < 0 {
            return None;
        }
        match self.cells.get(y as usize) {
            Some(row) => row.get(x as usize),
            None => None,
        }
    }

    // return a string of key chars where each key's door is in this maze
    fn doors_with_keys_here(&self) -> String {
        let mut doors: u32 = 0;
        let mut keys: u32 = 0;
        for cell in self.cells.iter().flat_map(|r| r.iter()).into_iter() {
            match cell {
                Tile::Key { key_id: n } => {
                    keys |= 1 << *n;
                }
                Tile::Door { key_id: n } => {
                    doors |= 1 << *n;
                }
                _ => {}
            }
        }

        let keys_with_doors = keys & doors;
        let mut keys_with_doors_here = String::new();
        for i in 0..26 {
            if (keys_with_doors >> i) & 1 == 1 {
                keys_with_doors_here.push(std::char::from_u32('a' as u32 + i).unwrap())
            }
        }

        keys_with_doors_here
    }
}

#[derive(Clone, Copy)]
enum Tile {
    // key_id for Key and corresponding Door match and are adjusted s.t. key a is 0, key b is 1, etc.
    Key { key_id: u32 },
    Door { key_id: u32 },
    Wall,
    Empty,
}

impl Tile {
    const CH_ROBOT: char = '@';
    const CH_WALL: char = '#';
    const CH_EMPTY: char = '.';

    fn from_char(c: char) -> Tile {
        if c == Tile::CH_WALL {
            Tile::Wall
        } else if c == Tile::CH_EMPTY {
            Tile::Empty
        } else if c.is_ascii_uppercase() {
            Tile::Door {
                key_id: c as u32 - 'A' as u32,
            }
        } else if c.is_ascii_lowercase() {
            Tile::Key {
                key_id: c.to_ascii_uppercase() as u32 - 'A' as u32,
            }
        } else {
            panic!(format!("Unknown maze character '{}'", c))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TINY: &str = "#########
    #b.A.@.a#
    #########";

    const INPUT_MED: &str = "########################
    #f.D.E.e.C.b.A.@.a.B.c.#
    ######################.#
    #d.....................#
    ########################";

    const INPUT_MED2: &str = "########################
    #...............b.C.D.f#
    #.######################
    #.....@.a.B.c.d.A.e.F.g#
    ########################";

    const INPUT_MED3: &str = "#################
    #i.G..c...e..H.p#
    ########.########
    #j.A..b...f..D.o#
    ########@########
    #k.E..a...g..B.n#
    ########.########
    #l.F..d...h..C.m#
    #################";

    const INPUT_MED4: &str = "########################
    #@..............ac.GI.b#
    ###d#e#f################
    ###A#B#C################
    ###g#h#i################
    ########################";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 3832)
    }

    #[test]
    fn test_part1_tiny() {
        assert_eq!(part1(INPUT_TINY), 8)
    }

    #[test]
    fn test_part1_med() {
        assert_eq!(part1(INPUT_MED), 86)
    }

    #[test]
    fn test_part1_med2() {
        assert_eq!(part1(INPUT_MED2), 132)
    }

    #[test]
    fn test_part1_med3() {
        assert_eq!(part1(INPUT_MED3), 136)
    }

    #[test]
    fn test_part1_med4() {
        assert_eq!(part1(INPUT_MED4), 81)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1724)
    }
}
