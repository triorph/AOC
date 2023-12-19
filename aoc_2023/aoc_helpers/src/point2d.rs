use std::{
    fmt::Debug,
    ops::{Add, Mul},
};

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Point2D {
    pub x: isize,
    pub y: isize,
}

impl Debug for Point2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
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

impl Mul<isize> for &Point2D {
    type Output = Point2D;

    fn mul(self, scalar: isize) -> Self::Output {
        Point2D {
            x: self.x * scalar,
            y: self.y * scalar,
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
