use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../day3.txt");

fn main() {
    println!("Day 3-1: {}", calc_day3(|it| it.point.manhattan_distance()));
    println!("Day 3-2: {}", calc_day3(|it| it.total_distance));
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

const START_POINT: Point = Point { x: 0, y: 0 };
impl Point {
    fn dist_from(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn manhattan_distance(&self) -> i32 {
        self.dist_from(&START_POINT)
    }
}

struct Delta {
    dx: i32,
    dy: i32,
}

impl Delta {
    fn apply_to(&self, point: &Point) -> Point {
        Point {
            x: point.x + self.dx,
            y: point.y + self.dy,
        }
    }

    fn from(direction: &str, distance: i32) -> Delta {
        let (dx, dy) = match direction {
            "L" => (-distance, 0),
            "R" => (distance, 0),
            "U" => (0, distance),
            "D" => (0, -distance),
            _ => panic!("Unknown direction: {}", direction),
        };
        Delta { dx, dy }
    }
}

struct Intersection {
    point: Point,
    total_distance: i32,
}

#[derive(Debug)]
struct Segment {
    p1: Point,
    p2: Point,
}

impl Segment {
    fn intersection_with(&self, other: &Segment) -> Option<Point> {
        let orientation = (self.dx() != 0, other.dx() != 0);
        match orientation {
            (false, true) => Segment::do_find_intersection(self, other),
            (true, false) => Segment::do_find_intersection(other, self),
            _ => None,
        }
    }

    fn do_find_intersection(
        vertical_segment: &Segment,
        horizontal_segment: &Segment,
    ) -> Option<Point> {
        let x_range = Segment::ordered_range(horizontal_segment.p1.x, horizontal_segment.p2.x);
        let y_range = Segment::ordered_range(vertical_segment.p1.y, vertical_segment.p2.y);
        let candidate_x = vertical_segment.p1.x;
        let candidate_y = horizontal_segment.p1.y;

        if x_range.contains(&candidate_x) && y_range.contains(&candidate_y) {
            Some(Point {
                x: candidate_x,
                y: candidate_y,
            })
        } else {
            None
        }
    }

    fn dx(&self) -> i32 {
        self.p1.x - self.p2.x
    }

    fn dy(&self) -> i32 {
        self.p1.y - self.p2.y
    }

    fn length(&self) -> i32 {
        if self.dx() != 0 {
            self.dx().abs()
        } else {
            self.dy().abs()
        }
    }

    fn ordered_range(coord1: i32, coord2: i32) -> RangeInclusive<i32> {
        // a range such as 6..=2 does not "contain" values (2, 6], so need to invert it
        if coord1 <= coord2 {
            coord1..=coord2
        } else {
            coord2..=coord1
        }
    }
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

trait SegmentCalculator {
    fn segment_ending_at(&self, index: usize) -> Segment;
}

impl SegmentCalculator for Vec<Point> {
    fn segment_ending_at(&self, index: usize) -> Segment {
        Segment {
            p1: *(self.get(index - 1).unwrap()),
            p2: *(self.get(index).unwrap()),
        }
    }
}
