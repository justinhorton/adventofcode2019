use std::ops::RangeInclusive;

fn main() {
    day1();
    day2();
    day3();
}

/*~~~ DAY 1 ~~~*/
const INPUT_DAY1: &str = include_str!("../input/day1.txt");

fn day1() {
    println!("day 1-2: {}", get_total_fuel(calc_fuel_mass_only));
    println!("day 1-2: {}", get_total_fuel(calc_fuel_package_and_fuel))
}

fn get_total_fuel(fuel_fn: fn(i32) -> i32) -> i32 {
    INPUT_DAY1
        .lines()
        .map(|line| line.parse::<i32>().expect(&format!("Can't parse int")))
        .map(fuel_fn)
        .sum()
}

// fuel calculation
fn calc_fuel_mass_only(mass: i32) -> i32 {
    mass / 3 - 2
}

fn calc_fuel_package_and_fuel(mass: i32) -> i32 {
    let mut result = 0;
    let mut cur_fuel = calc_fuel_mass_only(mass);

    while cur_fuel > 0 {
        result += cur_fuel;
        cur_fuel = calc_fuel_mass_only(cur_fuel);
    }
    result
}

/*~~~ DAY 2 ~~~*/

const INPUT_DAY2: &str = include_str!("../input/day2.txt");
fn day2() {
    let mem0 = calc_day2(12, 2);
    println!("Day 2-1: {}", mem0);

    for n in 0..99 {
        for v in 0..99 {
            let result = calc_day2(n, v);
            if result == 19690720 {
                let answer = 100 * n + v;
                println!("Day 2-2: {}", answer);
                return;
            }
        }
    }
}

// OPCODE 1: 1,op1,op2,dest; set dest = *op1 + *op2
// OPCODE 2: 2,op1,op2,dest; set dest = *op1 * *op2
// OPCODE 99: halt
// OPCODE anything else: error
fn calc_day2(noun: i32, verb: i32) -> i32 {
    let input: Vec<&str> = INPUT_DAY2.split(',').collect();
    let mut memory: Vec<i32> = input
        .iter()
        .map(|it| it.parse::<i32>().expect("Can't parse int"))
        .collect();

    // restore the state at the time of the elf problem
    memory[1] = noun;
    memory[2] = verb;

    let mut pos = 0;
    while pos < memory.len() {
        let opcode = memory[pos];
        match opcode {
            1..=2 => {
                let op1_pos = memory[pos + 1] as usize;
                let op2_pos = memory[pos + 2] as usize;
                let dest_pos = memory[pos + 3] as usize;
                let op1 = memory[op1_pos];
                let op2 = memory[op2_pos];

                let result = if opcode == 1 { op1 + op2 } else { op1 * op2 };
                memory[dest_pos] = result;
            }
            99 => return memory[0],
            _ => panic!(),
        };
        pos += 4;
    }
    memory[0]
}

/*~~~ DAY 3 ~~~*/
const INPUT_DAY3: &str = include_str!("../input/day3.txt");
fn day3() {
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
    let wires: Vec<&str> = INPUT_DAY3.trim().lines().collect();

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
