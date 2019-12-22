use std::collections::{HashSet, VecDeque};

const INPUT: &str = include_str!("../day18.txt");

fn main() {
    println!("Day 18-1: Steps to all keys: {}", part1());
}

fn part1() -> i32 {
    steps_to_find_all_keys(INPUT, 26)
}

fn steps_to_find_all_keys(input: &str, num_keys: usize) -> i32 {
    let maze = Maze::create_from_input(input);

    let mut visited: HashSet<PositionWithKeys> = HashSet::new();
    let mut visit_queue: VecDeque<PositionWithKeys> = VecDeque::new();
    visit_queue.push_back(PositionWithKeys::create_no_keys(
        maze.start_pos.x,
        maze.start_pos.y,
    ));

    // goal has all key_id bits set (where key_id is 0-indexed based on the key's char in the maze)
    let goal_encoded_keys = {
        let mut k: u32 = 0;
        for i in 0..num_keys {
            k |= 1 << i as u32;
        }
        k
    };

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
                if next_pos_with_keys.is_goal(goal_encoded_keys) {
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
    fn create_no_keys(x: usize, y: usize) -> PositionWithKeys {
        PositionWithKeys {
            x,
            y,
            encoded_keys: 0,
        }
    }

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
    start_pos: Position,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl Maze {
    fn create_from_input(input: &str) -> Maze {
        let mut cells: Vec<Vec<Tile>> = Vec::new();
        let mut x = 0;
        let mut y = 0;

        let mut start_pos = None;
        for row in input.trim().lines().map(|r| r.trim()) {
            let mut row_vec = Vec::new();
            for row_c in row.chars() {
                if row_c == Tile::CH_ROBOT {
                    start_pos = Some(Position { x, y });
                    row_vec.push(Tile::Empty);
                } else {
                    row_vec.push(Tile::from_char(row_c));
                }
                x += 1;
            }
            cells.push(row_vec);
            x = 0;
            y += 1;
        }

        Maze {
            cells,
            start_pos: start_pos.unwrap(),
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
        assert_eq!(part1(), 3832)
    }

    #[test]
    fn test_part1_tiny() {
        assert_eq!(steps_to_find_all_keys(INPUT_TINY, 2), 8)
    }

    #[test]
    fn test_part1_med() {
        assert_eq!(steps_to_find_all_keys(INPUT_MED, 6), 86)
    }

    #[test]
    fn test_part1_med2() {
        assert_eq!(steps_to_find_all_keys(INPUT_MED2, 7), 132)
    }

    #[test]
    fn test_part1_med3() {
        assert_eq!(steps_to_find_all_keys(INPUT_MED3, 16), 136)
    }

    #[test]
    fn test_part1_med4() {
        assert_eq!(steps_to_find_all_keys(INPUT_MED4, 9), 81)
    }
}
