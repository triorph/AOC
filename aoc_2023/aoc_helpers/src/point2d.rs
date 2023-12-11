use std::ops::Add;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Point2D {
    pub x: isize,
    pub y: isize,
}

pub trait Neighbours {
    fn get_neighbours(&self) -> Vec<Point2D>;
}

impl Point2D {
    pub fn get_manhattan_distance(&self, other: &Point2D) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }
}

impl Add for Point2D {
    type Output = Point2D;

    fn add(self, other: Point2D) -> Point2D {
        Point2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add for &Point2D {
    type Output = Point2D;

    fn add(self, other: &Point2D) -> Point2D {
        Point2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
