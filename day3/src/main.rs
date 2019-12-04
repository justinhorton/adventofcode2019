use crate::point::{Delta, Point, START_POINT};
use crate::segment::SegmentCalculator;

mod point;
mod segment;

const INPUT: &str = include_str!("../day3.txt");

fn main() {
    println!("Day 3-1: {}", calc_day3(|it| it.point.manhattan_distance()));
    println!("Day 3-2: {}", calc_day3(|it| it.total_distance));
}

struct Intersection {
    point: Point,
    total_distance: i32,
}

fn calc_day3(distance_fn: fn(&Intersection) -> i32) -> i32 {
    let wires: Vec<&str> = INPUT.trim().lines().collect();

    let points_by_wire = points_by_wire(wires);
    // just stick to 2 wires here
    let wire1_points = points_by_wire.first().unwrap();
    let wire2_points = points_by_wire.last().unwrap();

    let mut intersections: Vec<Intersection> = Vec::new();

    let mut w1_len = 0;
    let mut i_1 = 1;
    loop {
        let w1_seg = wire1_points.segment_ending_at(i_1);

        let mut w2_len = 0;
        let mut i_2 = 1;
        loop {
            let w2_seg = wire2_points.segment_ending_at(i_2);

            let maybe_intersection = w2_seg.intersection_with(&w1_seg);
            if maybe_intersection.is_some() && maybe_intersection.unwrap() != START_POINT {
                // discard the START_POINT as an intersection
                let intersection = maybe_intersection.unwrap();
                // keep track of segment distance up to the intersection
                let total_distance = w1_len
                    + w2_len
                    + intersection.dist_from(&w2_seg.p1)
                    + intersection.dist_from(&w1_seg.p1);
                intersections.push(Intersection {
                    point: intersection,
                    total_distance,
                });
            }

            i_2 += 1;
            w2_len += w2_seg.length();
            if i_2 >= wire2_points.len() {
                break;
            }
        }

        i_1 += 1;
        w1_len += w1_seg.length();
        if i_1 >= wire1_points.len() {
            break;
        }
    }

    // minimize based on the supplied distance function
    intersections.iter().map(distance_fn).min().unwrap()
}

fn points_by_wire(wires: Vec<&str>) -> Vec<Vec<Point>> {
    let mut points_by_wire: Vec<Vec<Point>> = Vec::new();
    for wire in wires {
        let wire_dirs: Vec<&str> = wire.split(',').collect();

        points_by_wire.push(Vec::new());
        let wire_vec = points_by_wire.last_mut().unwrap();
        wire_vec.push(START_POINT);

        for wire_dir in wire_dirs {
            let (dir_str, distance_str) = wire_dir.split_at(1);
            let distance = distance_str.parse::<i32>().expect("Parse error");

            let delta = Delta::from(dir_str, distance);
            let next_point = delta.apply_to(wire_vec.last().unwrap());
            wire_vec.push(next_point)
        }
    }
    points_by_wire
}
