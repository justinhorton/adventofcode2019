use crate::point::Point;
use std::ops::RangeInclusive;

#[derive(Debug)]
pub(crate) struct Segment {
    pub(crate) p1: Point,
    pub(crate) p2: Point,
}

impl Segment {
    pub(crate) fn intersection_with(&self, other: &Segment) -> Option<Point> {
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

    pub(crate) fn length(&self) -> i32 {
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

pub(crate) trait SegmentCalculator {
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
