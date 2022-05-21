use crate::point::Point;

use super::Shape;

pub struct Dot {
    location: Point,
}

impl Dot {
    pub fn new(location: Point) -> Self {
        Dot { location }
    }
}

impl Shape for Dot {
    fn pixel(&self, point: &Point) -> u8 {
        if *point == self.location {
            1
        } else {
            0
        }
    }
}
