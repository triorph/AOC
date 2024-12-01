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

impl Neighbours for Point2D {
    fn get_neighbours(&self) -> Vec<Point2D> {
        vec![
            self + &Point2D { x: 0, y: -1 },
            self + &Point2D { x: 0, y: 1 },
            self + &Point2D { x: -1, y: 0 },
            self + &Point2D { x: 1, y: 0 },
        ]
    }
}

impl Point2D {
    pub fn from_usize(x: usize, y: usize) -> Point2D {
        Point2D {
            x: x as isize,
            y: y as isize,
        }
    }

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
