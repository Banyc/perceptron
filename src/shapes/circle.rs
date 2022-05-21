use crate::point::Point;

use super::Shape;

pub struct Circle {
    center: Point,
    radius: f64,
}

impl Circle {
    fn check_rep(&self) {
        // nonzero
        assert!(self.radius > 0.0);

        if self.radius == 0.0 {
            let zero = Point { x: 0, y: 0 };
            assert!(self.center == zero);
        }
    }

    pub fn new(center: Point, radius: f64) -> Self {
        // nonzero
        assert!(radius > 0.0);

        let this = Circle { center, radius };
        this.check_rep();
        this
    }

    pub fn contains(&self, point: &Point) -> bool {
        let dist = self.center.distance(point);
        dist <= self.radius
    }
}

impl Shape for Circle {
    fn pixel(&self, point: &Point) -> u8 {
        if self.contains(point) {
            1
        } else {
            0
        }
    }
}
