use crate::point::Point;

pub trait Shape {
    fn pixel(&self, point: &Point) -> u8;
}
