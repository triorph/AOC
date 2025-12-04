use std::{
    fmt::Debug,
    ops::{Add, Mul, Range, Sub},
};

use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
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
    fn get_8_neighbours(&self) -> Vec<Point2D>;
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

    fn get_8_neighbours(&self) -> Vec<Point2D> {
        vec![
            self + &Point2D { x: -1, y: -1 },
            self + &Point2D { x: 0, y: -1 },
            self + &Point2D { x: 1, y: -1 },
            self + &Point2D { x: -1, y: 0 },
            self + &Point2D { x: 1, y: 0 },
            self + &Point2D { x: -1, y: 1 },
            self + &Point2D { x: 0, y: 1 },
            self + &Point2D { x: 1, y: 1 },
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

    pub fn iterate_x_y(x_range: Range<isize>, y_range: Range<isize>) -> Vec<Point2D> {
        x_range
            .cartesian_product(y_range)
            .map(|(x, y)| Point2D { x, y })
            .collect()
    }

    pub fn iterate_x_y_usize(x_range: Range<usize>, y_range: Range<usize>) -> Vec<Point2D> {
        x_range
            .cartesian_product(y_range)
            .map(|(x, y)| Point2D::from_usize(x, y))
            .collect()
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

impl Sub for Point2D {
    type Output = Point2D;

    fn sub(self, other: Point2D) -> Point2D {
        Point2D {
            x: self.x - other.x,
            y: self.y - other.y,
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

impl Sub for &Point2D {
    type Output = Point2D;

    fn sub(self, other: &Point2D) -> Point2D {
        Point2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
