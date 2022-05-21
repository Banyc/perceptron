use std::fmt;

use crate::{point::Point, shapes::Shape};

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Vec<u8>>,
}

impl Canvas {
    fn check_rep(&self) {
        assert_eq!(self.width, self.pixels.len());
        for y_strip in &self.pixels {
            assert_eq!(self.height, y_strip.len());
        }
    }

    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![vec![0; height]; width];
        let this = Canvas {
            width,
            height,
            pixels,
        };
        this.check_rep();
        this
    }

    pub fn set_pixel(&mut self, pixel: u8, point: &Point) {
        assert!(point.x < self.width);
        assert!(point.y < self.height);
        self.pixels[point.x][point.y] = pixel;
        self.check_rep()
    }

    #[inline]
    pub fn pixel(&self, point: &Point) -> u8 {
        assert!(point.x < self.width);
        assert!(point.y < self.height);
        self.pixels[point.x][point.y]
    }

    pub fn draw<S>(&mut self, shape: &S)
    where
        S: Shape,
    {
        for x in 0..self.width {
            for y in 0..self.height {
                let point = Point { x, y };
                self.pixels[x][y] = shape.pixel(&point);
            }
        }
        self.check_rep()
    }

    pub fn clear(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.pixels[x][y] = 0;
            }
        }
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }
}

impl fmt::Display for Canvas {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{:02X}  ", self.pixels[x][y])?;
            }
            writeln!(f, "")?;
            writeln!(f, "")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::shapes::circle::Circle;

    use super::*;

    #[test]
    fn new() {
        Canvas::new(1, 1);
        Canvas::new(0, 0);
        Canvas::new(0, 1);
    }

    #[test]
    fn set_get_pixel() {
        let mut c = Canvas::new(1, 2);
        let p = Point { x: 0, y: 1 };
        c.set_pixel(3, &p);
        assert_eq!(c.pixel(&p), 3);
    }

    #[test]
    fn draw() {
        let mut canvas = Canvas::new(11, 11);
        let center = Point { x: 5, y: 5 };
        let circle = Circle::new(center, 5.0);
        canvas.draw(&circle);
        assert_eq!(canvas.pixel(&Point { x: 0, y: 0 }), 0);
        assert_eq!(canvas.pixel(&Point { x: 5, y: 5 }), 1);

        println!("{}", canvas);
    }
}
