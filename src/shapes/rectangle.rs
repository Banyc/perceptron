use crate::point::Point;

use super::Shape;

pub struct Rectangle {
    top_left: Point,
    bottom_right: Point, // inclusive
}

impl Rectangle {
    fn check_rep(&self) {
        // nonzero
        let zero = Point { x: 0, y: 0 };
        assert!(self.top_left != zero && self.bottom_right != zero);

        if self.top_left != zero && self.bottom_right != zero {
            assert!(self.top_left.x < self.bottom_right.x);
            assert!(self.top_left.y < self.bottom_right.y);
        }
    }

    pub fn new(top_left: Point, bottom_right: Point) -> Self {
        let this = Rectangle {
            top_left,
            bottom_right,
        };
        this.check_rep();
        this
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.bottom_right.x - self.top_left.x
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.bottom_right.y - self.top_left.y
    }

    pub fn contains(&self, point: &Point) -> bool {
        point.x >= self.top_left.x
            && point.x <= self.bottom_right.x
            && point.y >= self.top_left.y
            && point.y <= self.bottom_right.y
    }
}

impl Shape for Rectangle {
    fn pixel(&self, point: &Point) -> u8 {
        if self.contains(point) {
            1
        } else {
            0
        }
    }
}
