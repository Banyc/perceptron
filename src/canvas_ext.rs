use ndarray::Array1;
use rand::Rng;

use crate::{
    canvas::Canvas,
    point::Point,
    shapes::{circle::Circle, rectangle::Rectangle},
};

impl Canvas {
    pub fn one_dimensional_array(&self) -> Array1<f64> {
        let mut array = Array1::<f64>::zeros(self.width() * self.height());
        for y in 0..self.height() {
            for x in 0..self.width() {
                let p = Point { x, y };
                array[y * self.height() + x] = match self.pixel(&p) == 0 {
                    true => 1.0,
                    false => 0.0,
                }
            }
        }
        array
    }

    pub fn draw_random_circle(&mut self) {
        let mut rnd = rand::thread_rng();
        let center = Point {
            // [1, (width - 1) - 1]
            x: rnd.gen_range(1..=(self.width() - 1) - 1),
            // [1, (height - 1) - 1]
            y: rnd.gen_range(1..=(self.height() - 1) - 1),
        };
        assert!(center.x >= 1);
        assert!(center.y >= 1);
        assert!(center.x <= (self.width() - 1) - 1);
        assert!(center.y <= (self.height() - 1) - 1);
        let max_radius = usize::max(self.width(), self.height());
        let max_radius = usize::min(max_radius, center.x);
        let max_radius = usize::min(max_radius, (self.width() - 1) - center.x);
        let max_radius = usize::min(max_radius, center.y);
        let max_radius = usize::min(max_radius, (self.height() - 1) - center.y);
        // [1, max_radius]
        let radius = rnd.gen_range(1..=max_radius);
        // println!("{}", max_radius);
        // println!("{}", radius);
        let circle = Circle::new(center, radius as f64);
        self.draw(&circle);
    }

    pub fn draw_random_rectangle(&mut self) {
        let mut rnd = rand::thread_rng();
        let top_left = Point {
            x: rnd.gen_range(0..=(self.width() - 1) - 1),
            y: rnd.gen_range(0..=(self.height() - 1) - 1),
        };
        let bottom_right = Point {
            x: rnd.gen_range(top_left.x + 1..=(self.width() - 1)),
            y: rnd.gen_range(top_left.y + 1..=(self.height() - 1)),
        };
        let rectangle = Rectangle::new(top_left, bottom_right);
        self.draw(&rectangle);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circle() {
        let mut canvas = Canvas::new(10, 9);
        canvas.draw_random_circle();
        println!("{}", canvas);
    }

    #[test]
    fn rectangle() {
        let mut canvas = Canvas::new(10, 9);
        canvas.draw_random_rectangle();
        println!("{}", canvas);
    }
}
