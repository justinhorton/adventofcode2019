extern crate itertools;
extern crate log;

use itertools::Itertools;
use log::debug;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::f32::consts::{FRAC_PI_2, PI};
use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../day10.txt");
const ASTEROID: char = '#';
const EMPTY: char = '.';

fn main() {
    let (max_visible, (x, y)) = day10_part1(INPUT);
    println!("Day 10-1: Max visible: {} at ({}, {})", max_visible, x, y);

    let two_hundredth = day10_part2();
    println!(
        "Day 10-2: 200th vaporized is {:?}. Answer = 100 * x + y = {}",
        two_hundredth,
        100 * two_hundredth.x + two_hundredth.y
    )
}

fn day10_part2() -> Asteroid {
    calc_day10_part2(INPUT, (17, 22))
}

fn calc_day10_part2(input: &str, monitoring_location: (usize, usize)) -> Asteroid {
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

    let (source_x, source_y) = monitoring_location;
    let mut monitoring = MonitoringAsteroid {
        asteroid: Asteroid {
            x: source_x,
            y: source_y,
            atan: 0.0,
        },
        visible_asteroids: Vec::new(),
    };

    let mut num_vaporized = 0;
    loop {
        // find visible asteroids
        for (target_x, target_y) in (0..width).cartesian_product(0..height) {
            if let ASTEROID = data[target_y][target_x] {
                let target = Asteroid {
                    x: target_x,
                    y: target_y,
                    atan: 0.0,
                };

                if let Some(atan) = monitoring.asteroid.is_target_visible(&target, &data) {
                    monitoring.visible_asteroids.push(Asteroid {
                        x: target_x,
                        y: target_y,
                        atan,
                    });
                }
            }
        }

        // minimize the customized atan2 calc, so first asteroids are the closest to "up",
        // proceeding clockwise
        monitoring.visible_asteroids.sort_by(|a1, a2| {
            if a1.atan < a2.atan {
                Less
            } else if a1.atan == a2.atan {
                Equal
            } else {
                Greater
            }
        });

        // goodbye
        for a in monitoring.visible_asteroids.iter() {
            num_vaporized += 1;
            if num_vaporized == 200 {
                return *a;
            }
            debug!(
                "i={}: Vaporizing {:?}, Relative target: {}, {}",
                num_vaporized,
                a,
                a.x() - monitoring.asteroid.x(),
                a.y() - monitoring.asteroid.y()
            );
            data[a.y][a.x] = EMPTY;
        }
        monitoring.visible_asteroids.clear();
    }
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
            atan: 0.0,
        };
        if let ASTEROID = data[source_y][source_x] {
            for (target_x, target_y) in (0..width).cartesian_product(0..height) {
                if let ASTEROID = data[target_y][target_x] {
                    let target = Asteroid {
                        x: target_x,
                        y: target_y,
                        atan: 0.0,
                    };
                    if let Some(_) = source.is_target_visible(&target, &data) {
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

#[derive(PartialEq, Debug, Copy, Clone)]
struct Asteroid {
    x: usize,
    y: usize,
    atan: f32,
}

impl Asteroid {
    fn x(&self) -> f32 {
        self.x as f32
    }

    fn y(&self) -> f32 {
        self.y as f32
    }
}

struct MonitoringAsteroid {
    asteroid: Asteroid,
    visible_asteroids: Vec<Asteroid>,
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

    fn is_target_visible(&self, target_point: &Asteroid, data: &Vec<Vec<char>>) -> Option<f32> {
        if self == target_point {
            return None;
        }

        let s_t_slope = self.slope_with(target_point);
        let range = range(self.x, target_point.x).cartesian_product(range(self.y, target_point.y));
        for (x2, y2) in range {
            if let ASTEROID = data[y2][x2] {
                let blocker_point = &Asteroid {
                    x: x2,
                    y: y2,
                    atan: 0.0,
                };
                if blocker_point != self && blocker_point != target_point {
                    if s_t_slope == self.slope_with(&blocker_point) {
                        return None;
                    }
                }
            }
        }

        // say new origin is (3, 4), target is (2, 1)
        // then adjusted coords are (-1, -3)
        let (rel_x, rel_y) = (target_point.x() - self.x(), target_point.y() - self.y());
        let atan = shifted_atan2(rel_x, rel_y);
        Some(atan)
    }
}

fn shifted_atan2(x: f32, y: f32) -> f32 {
    // given a relativized x, y of the target, adjust the atan2 calc so that instead:
    //   a) it is always positive, [0, 2pi) instead of [-pi, pi]
    //   b) it is adjusted s.t. the "up" on the given examples (-y direction) is 0 rad with rads
    //      increasing clockwise
    let it = y.atan2(x);
    if it >= -PI && it < -FRAC_PI_2 {
        it + 5.0 * FRAC_PI_2
    } else {
        FRAC_PI_2 + it
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
    use crate::{calc_day10_part2, day10_part1, day10_part2, shifted_atan2, Asteroid, INPUT};
    use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};

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

    #[test]
    fn test_shifted_atan2() {
        assert_eq!(shifted_atan2(0f32, 3f32), PI);
        assert_eq!(shifted_atan2(3f32, 3f32), FRAC_PI_2 + FRAC_PI_4);
        assert_eq!(shifted_atan2(0f32, -3f32), 0.0);
        assert_eq!(shifted_atan2(-3f32, -3f32), 2.0 * PI - FRAC_PI_4);
        assert_eq!(shifted_atan2(3f32, 0f32), FRAC_PI_2);
        assert_eq!(shifted_atan2(-3f32, 0f32), PI + FRAC_PI_2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            day10_part2(),
            Asteroid {
                x: 6,
                y: 16,
                atan: 5.2117357f32
            }
        )
    }

    #[test]
    fn test_part2_ex5() {
        assert_eq!(
            calc_day10_part2(INPUT_EX5, (11, 13)),
            Asteroid {
                x: 8,
                y: 2,
                atan: 6.0169334
            }
        )
    }
}
