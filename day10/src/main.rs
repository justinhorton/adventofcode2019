extern crate itertools;

use itertools::Itertools;
use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../day10.txt");
const ASTEROID: char = '#';
const EMPTY: char = '.';

fn main() {
    let (max_visible, (x, y)) = day10_part1(INPUT);
    println!("Max visible: {} at ({}, {})", max_visible, x, y);
}

fn day10_part1(input: &str) -> (usize, (usize, usize)) {
    let trimmed = input.trim();
    let width = trimmed
        .split_whitespace()
        .nth(0)
        .map(|it| it.len())
        .unwrap();
    let height = trimmed.split_whitespace().count();

    let mut data = vec![vec![EMPTY; width]; height];
    let mut y = 0;
    for row in trimmed.split_whitespace() {
        let mut x = 0;
        for ch in row.chars() {
            data[y][x] = ch;
            x += 1;
        }
        y += 1;
    }

    let mut counts = vec![vec![0; width]; height];
    // utilize peak efficiency of cartesian product * cartesian product * k
    for (source_x, source_y) in (0..width).cartesian_product(0..height) {
        let source = Asteroid {
            x: source_x,
            y: source_y,
        };
        if let ASTEROID = data[source_y][source_x] {
            for (target_x, target_y) in (0..width).cartesian_product(0..height) {
                if let ASTEROID = data[target_y][target_x] {
                    let target = Asteroid {
                        x: target_x,
                        y: target_y,
                    };
                    if source.is_target_visible(&target, &data) {
                        counts[source_y][source_x] += 1;
                    }
                }
            }
        }
    }

    let mut best_coords = (width, height);
    let mut max_visible = 0;
    for (source_x, source_y) in (0..width).cartesian_product(0..height) {
        if let ASTEROID = data[source_y][source_x] {
            let m = counts[source_y][source_x];
            if m > max_visible {
                max_visible = m;
                best_coords = (source_x, source_y);
            }
        }
    }

    (max_visible, best_coords)
}

#[derive(PartialEq)]
struct Asteroid {
    x: usize,
    y: usize,
}

impl Asteroid {
    fn slope_with(&self, other: &Asteroid) -> f32 {
        let divisor = other.x as f32 - self.x as f32;
        if divisor == 0.0 {
            0.0
        } else {
            (other.y as f32 - self.y as f32) / divisor
        }
    }

    fn is_target_visible(&self, target_point: &Asteroid, data: &Vec<Vec<char>>) -> bool {
        if self == target_point {
            return false;
        }

        let s_t_slope = self.slope_with(target_point);
        let range = range(self.x, target_point.x).cartesian_product(range(self.y, target_point.y));
        for (x2, y2) in range {
            if let ASTEROID = data[y2][x2] {
                let blocker_point = &Asteroid { x: x2, y: y2 };
                if blocker_point != self && blocker_point != target_point {
                    if s_t_slope == self.slope_with(&blocker_point) {
                        return false;
                    }
                }
            }
        }
        true
    }
}

fn range(v1: usize, v2: usize) -> RangeInclusive<usize> {
    // order the range for iteration
    if v1 <= v2 {
        v1..=v2
    } else {
        v2..=v1
    }
}

#[cfg(test)]
mod tests {
    use crate::{day10_part1, INPUT};

    #[test]
    fn test_part1() {
        let result = day10_part1(INPUT);
        assert_eq!(result, (288, (17, 22)));
    }

    const INPUT_EX1: &str = "
    .#..#
    .....
    #####
    ....#
    ...##
    ";

    #[test]
    fn test_part1_ex1() {
        let result = day10_part1(INPUT_EX1);
        assert_eq!(result, (8, (3, 4)))
    }

    const INPUT_EX2: &str = "
    ......#.#.
    #..#.#....
    ..#######.
    .#.#.###..
    .#..#.....
    ..#....#.#
    #..#....#.
    .##.#..###
    ##...#..#.
    .#....####
    ";

    #[test]
    fn test_part1_ex2() {
        let result = day10_part1(INPUT_EX2);
        assert_eq!(result, (33, (5, 8)))
    }

    const INPUT_EX3: &str = "
    #.#...#.#.
    .###....#.
    .#....#...
    ##.#.#.#.#
    ....#.#.#.
    .##..###.#
    ..#...##..
    ..##....##
    ......#...
    .####.###.
    ";

    #[test]
    fn test_part1_ex3() {
        let result = day10_part1(INPUT_EX3);
        assert_eq!(result, (35, (1, 2)))
    }

    const INPUT_EX4: &str = "
    .#..#..###
    ####.###.#
    ....###.#.
    ..###.##.#
    ##.##.#.#.
    ....###..#
    ..#.#..#.#
    #..#.#.###
    .##...##.#
    .....#.#..
    ";

    #[test]
    fn test_part1_ex4() {
        let result = day10_part1(INPUT_EX4);
        assert_eq!(result, (41, (6, 3)))
    }

    const INPUT_EX5: &str = "
    .#..##.###...#######
    ##.############..##.
    .#.######.########.#
    .###.#######.####.#.
    #####.##.#.##.###.##
    ..#####..#.#########
    ####################
    #.####....###.#.#.##
    ##.#################
    #####.##.###..####..
    ..######..##.#######
    ####.##.####...##..#
    .#####..#.######.###
    ##...#.##########...
    #.##########.#######
    .####.#.###.###.#.##
    ....##.##.###..#####
    .#.#.###########.###
    #.#.#.#####.####.###
    ###.##.####.##.#..##
    ";

    #[test]
    fn test_part1_ex5() {
        let result = day10_part1(INPUT_EX5);
        assert_eq!(result, (210, (11, 13)))
    }
}
