pub(crate) const START_POINT: Point = Point { x: 0, y: 0 };

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) struct Point {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl Point {
    pub(crate) fn dist_from(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub(crate) fn manhattan_distance(&self) -> i32 {
        self.dist_from(&START_POINT)
    }
}

pub(crate) struct Delta {
    dx: i32,
    dy: i32,
}

impl Delta {
    pub(crate) fn apply_to(&self, point: &Point) -> Point {
        Point {
            x: point.x + self.dx,
            y: point.y + self.dy,
        }
    }

    pub(crate) fn from(direction: &str, distance: i32) -> Delta {
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
