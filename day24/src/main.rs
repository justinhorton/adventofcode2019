use std::collections::HashSet;

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
}

fn part1() -> u64 {
    // "first repeated pattern" is the same as "first repeated biodiversity rating"
    let mut grid = Grid::parse_from(INPUT);
    let mut seen_ratings = HashSet::new();
    loop {
        let tick_rating = grid.biodiversity_rating();
        if !seen_ratings.insert(tick_rating) {
            return tick_rating;
        }
        grid.tick();
    }
}

struct Grid {
    cells: [[char; WIDTH]; HEIGHT],
    t: u32,
}

impl Grid {
    fn parse_from(input: &str) -> Grid {
        let mut cells = [[' '; WIDTH]; HEIGHT];
        for (y, line) in input.trim().lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                cells[y][x] = c;
            }
        }
        Grid { cells, t: 0 }
    }

    fn tick(&mut self) {
        let mut next_cells = [[' '; WIDTH]; HEIGHT];
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let adj_bugs = self.num_adj_bugs(x, y);
                match self.cell_at(x as i32, y as i32) {
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

        self.cells = next_cells;
        self.t += 1;
    }

    fn biodiversity_rating(&self) -> u64 {
        let mut rating = 0;

        let mut pow = 0;
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if let Some(CH_BUG) = self.cell_at(x as i32, y as i32) {
                    rating += 1 << pow;
                }
                pow += 1;
            }
        }
        rating
    }

    fn num_adj_bugs(&self, x: usize, y: usize) -> u8 {
        let x = x as i32;
        let y = y as i32;

        let mut count = 0;
        for (x, y) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)].iter() {
            if let Some(CH_BUG) = self.cell_at(*x, *y) {
                count += 1;
            }
        }
        count
    }

    fn cell_at(&self, x: i32, y: i32) -> Option<char> {
        if x < 0 || y < 0 || x >= WIDTH as i32 || y >= HEIGHT as i32 {
            None
        } else {
            Some(self.cells[y as usize][x as usize])
        }
    }

    fn print(&self) {
        println!("\nt={}", self.t);
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                print!("{}", self.cells[y][x]);
            }
            println!()
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
}
