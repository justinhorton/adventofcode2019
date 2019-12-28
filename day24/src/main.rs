extern crate itertools;

use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../day24.txt");

const WIDTH: usize = 5;
const HEIGHT: usize = 5;
const CH_BUG: char = '#';
const CH_EMPTY: char = '.';

fn main() {
    println!(
        "Day 24-1: Biodiversity rating of first repeated pattern: {}",
        part1()
    );
    println!("Day 24-2: Bugs after 200 mins: {}", part2())
}

fn part1() -> u64 {
    // "first repeated pattern" is the same as "first repeated biodiversity rating"
    let mut grid = Grid::parse_from(INPUT, false);
    let mut seen_ratings = HashSet::new();
    loop {
        let tick_rating = grid.biodiversity_rating();
        if !seen_ratings.insert(tick_rating) {
            return tick_rating;
        }
        grid.tick();
    }
}

fn part2() -> u64 {
    let mut grid = Grid::parse_from(INPUT, true);
    for _t in 0..200 {
        grid.tick();
    }
    grid.count_all_bugs()
}

struct Grid {
    cells: HashMap<i32, [[char; WIDTH]; HEIGHT]>,
    t: u32,
    with_recursive_grids: bool,
}

impl Grid {
    fn parse_from(input: &str, with_recursion: bool) -> Grid {
        let mut cell_dim = [[' '; WIDTH]; HEIGHT];
        for (y, line) in input.trim().lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                cell_dim[y][x] = c;
            }
        }
        let mut cells = HashMap::new();
        cells.insert(0, cell_dim);
        Grid {
            cells,
            t: 0,
            with_recursive_grids: with_recursion,
        }
    }

    fn tick(&mut self) {
        self.t += 1;

        let mut replacement_cells = HashMap::new();
        if self.with_recursive_grids {
            // overkill with number of layers and copying...oh well
            let t: i32 = self.t as i32;
            self.cells.insert(t * -1, [[CH_EMPTY; WIDTH]; HEIGHT]);
            self.cells.insert(t, [[CH_EMPTY; WIDTH]; HEIGHT]);
        }

        for &dim in self.cells.keys() {
            let mut next_cells = [[CH_EMPTY; WIDTH]; HEIGHT];
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    if self.with_recursive_grids && (x, y) == (2, 2) {
                        // super important: ignore center recursive tile
                        continue;
                    }

                    let adj_bugs = self.num_adj_bugs(dim, x, y);
                    match self.cell_at(dim, x as i32, y as i32) {
                        Some(CH_BUG) => {
                            if adj_bugs == 1 {
                                next_cells[y][x] = CH_BUG;
                            } else {
                                next_cells[y][x] = CH_EMPTY;
                            }
                        }
                        Some(CH_EMPTY) => {
                            if adj_bugs == 1 || adj_bugs == 2 {
                                next_cells[y][x] = CH_BUG;
                            } else {
                                next_cells[y][x] = CH_EMPTY;
                            }
                        }
                        _ => {}
                    }
                }
            }

            replacement_cells.insert(dim, next_cells);
        }

        self.cells = replacement_cells;
    }

    fn count_all_bugs(&self) -> u64 {
        let mut count = 0;
        for dim in self.cells.keys() {
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    if self.is_bug(*dim, x as i32, y as i32) {
                        count += 1
                    }
                }
            }
        }
        count
    }

    fn biodiversity_rating(&self) -> u64 {
        let mut rating = 0;

        let mut pow = 0;
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.is_bug(0, x as i32, y as i32) {
                    rating += 1 << pow;
                }
                pow += 1;
            }
        }
        rating
    }

    fn num_adj_bugs(&self, dim: i32, x: usize, y: usize) -> u8 {
        let x = x as i32;
        let y = y as i32;

        let mut count = 0;
        // bugs in this layer
        for (x, y) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)].iter() {
            if self.is_bug(dim, *x, *y) {
                count += 1;
            }
        }

        // bugs in other layers
        for (other_dim, x2, y2) in Self::dim_adjacencies(dim, x, y).iter() {
            if self.is_bug(*other_dim as i32, *x2 as i32, *y2 as i32) {
                count += 1;
            }
        }

        count
    }

    // adjacencies above and below this layer
    fn dim_adjacencies(dim: i32, x: i32, y: i32) -> Vec<(i32, usize, usize)> {
        // bugs in higher layer
        let mut other: Vec<(i32, usize, usize)> = Vec::new();
        if x == 0 {
            other.push((dim - 1, 1, 2))
        }
        if y == 0 {
            other.push((dim - 1, 2, 1))
        }
        if y == 4 {
            other.push((dim - 1, 2, 3))
        }
        if x == 4 {
            other.push((dim - 1, 3, 2))
        }

        // bugs in lower layer
        match (x, y) {
            (2, 1) => {
                for x2 in 0..WIDTH {
                    other.push((dim + 1, x2, 0))
                }
            }
            (1, 2) => {
                for y2 in 0..HEIGHT {
                    other.push((dim + 1, 0, y2))
                }
            }
            (2, 3) => {
                for x2 in 0..WIDTH {
                    other.push((dim + 1, x2, 4))
                }
            }
            (3, 2) => {
                for y2 in 0..HEIGHT {
                    other.push((dim + 1, 4, y2))
                }
            }
            _ => {}
        }
        other
    }

    fn is_bug(&self, dim: i32, x: i32, y: i32) -> bool {
        if let Some(CH_BUG) = self.cell_at(dim, x, y) {
            true
        } else {
            false
        }
    }

    fn cell_at(&self, dim: i32, x: i32, y: i32) -> Option<char> {
        if !self.cells.contains_key(&dim)
            || x < 0
            || y < 0
            || x >= WIDTH as i32
            || y >= HEIGHT as i32
        {
            None
        } else {
            Some(self.cells.get(&dim).unwrap()[y as usize][x as usize])
        }
    }

    fn print(&self) {
        println!("\nt={}", self.t);

        for dim in self.cells.keys().sorted() {
            println!("d={}", dim);
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    print!("{}", self.cells.get(dim).unwrap()[y][x]);
                }
                println!();
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 2130474);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1923)
    }
}
