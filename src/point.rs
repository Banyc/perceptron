#[derive(PartialEq, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn distance(&self, other: &Point) -> f64 {
        let x_diff = match self.x > other.x {
            true => self.x - other.x,
            false => other.x - self.x,
        };
        let y_diff = match self.y > other.y {
            true => self.y - other.y,
            false => other.y - self.y,
        };
        f64::sqrt((x_diff * x_diff + y_diff * y_diff) as f64)
    }
}
