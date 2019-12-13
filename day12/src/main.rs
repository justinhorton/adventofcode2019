extern crate num;

use regex::Regex;
use std::fmt::{Display, Error, Formatter};

const INPUT: &str = include_str!("../day12.txt");

fn main() {
    println!("Day 12-1: {:?}", day12_part1());
    println!("{:?}", day12_part2());
}

fn day12_part1() -> i32 {
    calc_day12_part1(INPUT, 1000)
}

fn calc_day12_part1(input: &str, t_val: u64) -> i32 {
    let mut moons = parse_input(input);
    pass_time(&mut moons, t_val);
    moons.iter().map(|it| it.total_energy()).sum()
}

fn day12_part2() -> u64 {
    calc_day12_part2(INPUT)
}

fn calc_day12_part2(input: &str) -> u64 {
    use num::Integer;

    let mut moons = parse_input(input);
    let (x_per, y_per, z_per) = find_periods(&mut moons);

    // the period across all 3 axes is the least common multiple of the system's x, y, and z periods
    x_per.lcm(&y_per).lcm(&z_per)
}

fn find_periods(moons: &mut Vec<Moon>) -> (u64, u64, u64) {
    let initial_moons: Vec<Moon> = moons.clone();

    let mut per_x: Option<u64> = None;
    let mut per_y: Option<u64> = None;
    let mut per_z: Option<u64> = None;

    // For (x, y, z) individually, find the t value where the system reaches its initial state
    // (original coordinates and 0 velocity). That is the period for the system on that axis.
    // Continue until we have the periods for each axis.
    let mut t = 0;
    loop {
        pass_time(moons, 1);
        t += 1;

        if let None = per_x {
            let is_initial_state: bool = initial_moons
                .iter()
                .map(|it| it.pos.x)
                .eq(moons.iter().map(|it| it.pos.x));
            if is_initial_state && moons.iter().all(|it| it.vel.x == 0) {
                per_x = Some(t);
            }
        }

        if let None = per_y {
            let is_initial_state: bool = initial_moons
                .iter()
                .map(|it| it.pos.y)
                .eq(moons.iter().map(|it| it.pos.y));
            if is_initial_state && moons.iter().all(|it| it.vel.y == 0) {
                per_y = Some(t);
            }
        }

        if let None = per_z {
            let is_initial_state: bool = initial_moons
                .iter()
                .map(|it| it.pos.z)
                .eq(moons.iter().map(|it| it.pos.z));
            if is_initial_state && moons.iter().all(|it| it.vel.z == 0) {
                per_z = Some(t);
            }
        }

        if let (Some(x), Some(y), Some(z)) = (per_x, per_y, per_z) {
            return (x, y, z);
        }
    }
}

fn pass_time(moons: &mut Vec<Moon>, t_val: u64) {
    let moon_pair_indices: Vec<(usize, usize)> = index_pairs(moons.len());

    let mut t: u64 = 0;
    loop {
        for (m1_i, m2_i) in &moon_pair_indices {
            let (a, b) = moons.split_at_mut(m1_i + 1);
            let moon1 = &mut a[*m1_i];
            let moon2 = &mut b[*m2_i - *m1_i - 1];
            moon1.apply_gravity(moon2);
        }

        moons.iter_mut().for_each(|m| m.apply_velocity());

        t += 1;
        if t == t_val {
            break;
        }
    }
}

fn parse_input(input: &str) -> Vec<Moon> {
    let moon_data_regex = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();

    let mut moons: Vec<Moon> = Vec::new();
    for line in input.lines().into_iter() {
        for cap in moon_data_regex.captures_iter(line) {
            let (a, b, c) = (
                &cap[1].parse::<i32>(),
                &cap[2].parse::<i32>(),
                &cap[3].parse::<i32>(),
            );

            let pos = Position {
                x: *a.as_ref().unwrap(),
                y: *b.as_ref().unwrap(),
                z: *c.as_ref().unwrap(),
            };
            let vel = Velocity { x: 0, y: 0, z: 0 };
            moons.push(Moon { pos, vel });
        }
    }
    moons
}

fn index_pairs(num_moons: usize) -> Vec<(usize, usize)> {
    // e.g. (0,1), (0,2), (0,3), (1,2), (1,3), (2,3)
    let mut vec = Vec::new();
    for i in 0..num_moons {
        for j in i..num_moons {
            if i != j {
                vec.push((i, j))
            }
        }
    }
    vec
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Moon {
    pos: Position,
    vel: Velocity,
}

impl Moon {
    fn total_energy(&self) -> i32 {
        self.pot() * self.kin()
    }

    fn pot(&self) -> i32 {
        i32::abs(self.pos.x) + i32::abs(self.pos.y) + i32::abs(self.pos.z)
    }

    fn kin(&self) -> i32 {
        i32::abs(self.vel.x) + i32::abs(self.vel.y) + i32::abs(self.vel.z)
    }

    fn apply_gravity(&mut self, other: &mut Moon) {
        Self::recalc_velocity(self.pos.x, other.pos.x, &mut self.vel.x, &mut other.vel.x);
        Self::recalc_velocity(self.pos.y, other.pos.y, &mut self.vel.y, &mut other.vel.y);
        Self::recalc_velocity(self.pos.z, other.pos.z, &mut self.vel.z, &mut other.vel.z);
    }

    fn recalc_velocity(c1: i32, c2: i32, this_v: &mut i32, that_v: &mut i32) {
        let diff = if c1 < c2 {
            1
        } else if c1 == c2 {
            0
        } else {
            -1
        };
        *this_v += diff;
        *that_v += -diff;
    }

    fn apply_velocity(&mut self) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
        self.pos.z += self.vel.z;
    }
}

impl Display for Moon {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        // pos=<x=-1, y=  0, z= 2>, vel=<x= 0, y= 0, z= 0>
        f.write_fmt(format_args!(
            "pos=<x={}, y={}, z={}>, vel=<x={}, y={}, z={}>",
            self.pos.x, self.pos.y, self.pos.z, self.vel.x, self.vel.y, self.vel.z
        ))
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Velocity {
    x: i32,
    y: i32,
    z: i32,
}

#[cfg(test)]
mod tests {
    use crate::{calc_day12_part1, calc_day12_part2, day12_part1, day12_part2, Moon};

    const INPUT_EX1: &str = "<x=-1, y=0, z=2>
    <x=2, y=-10, z=-7>
    <x=4, y=-8, z=8>
    <x=3, y=5, z=-1>
    ";
    const INPUT_EX2: &str = "<x=-8, y=-10, z=0>
    <x=5, y=5, z=10>
    <x=2, y=-7, z=3>
    <x=9, y=-8, z=-3>
    ";

    #[test]
    fn test_recalc_velocity() {
        let mut cur_v1 = 32;
        let mut cur_v2 = -5;

        Moon::recalc_velocity(3, 5, &mut cur_v1, &mut cur_v2);
        assert_eq!(cur_v1, 33);
        assert_eq!(cur_v2, -6);
    }

    #[test]
    fn test_part1() {
        assert_eq!(day12_part1(), 10845);
    }

    #[test]
    fn test_part1_ex1() {
        assert_eq!(calc_day12_part1(INPUT_EX1, 10), 179);
    }

    #[test]
    fn test_part1_ex2() {
        assert_eq!(calc_day12_part1(INPUT_EX2, 100), 1940);
    }

    #[test]
    fn test_part2() {
        assert_eq!(day12_part2(), 551272644867044);
    }

    #[test]
    fn test_part2_ex1() {
        assert_eq!(calc_day12_part2(INPUT_EX1), 2772);
    }

    #[test]
    fn test_part2_ex2() {
        assert_eq!(calc_day12_part2(INPUT_EX2), 4686774924);
    }
}
