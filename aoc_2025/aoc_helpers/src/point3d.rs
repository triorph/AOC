use std::{
    fmt::Debug,
    ops::{Add, Mul, Sub},
};

#[derive(PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
pub struct Point3D {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Debug for Point3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

pub trait Neighbours {
    fn get_neighbours(&self) -> Vec<Point3D>;
}

impl Neighbours for Point3D {
    fn get_neighbours(&self) -> Vec<Point3D> {
        vec![
            self + &Point3D { x: 0, y: -1, z: 0 },
            self + &Point3D { x: 0, y: 1, z: 0 },
            self + &Point3D { x: -1, y: 0, z: 0 },
            self + &Point3D { x: 1, y: 0, z: 0 },
            self + &Point3D { x: 0, y: 0, z: -1 },
            self + &Point3D { x: 0, y: 0, z: 1 },
        ]
    }
}

impl Point3D {
    pub fn from_usize(x: usize, y: usize, z: usize) -> Point3D {
        Point3D {
            x: x as isize,
            y: y as isize,
            z: z as isize,
        }
    }

    pub fn get_manhattan_distance(&self, other: &Point3D) -> usize {
        (self.x - other.x).unsigned_abs()
            + (self.y - other.y).unsigned_abs()
            + (self.z - other.z).unsigned_abs()
    }

    pub fn get_euclidean_distance(&self, other: &Point3D) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)) as f64)
            .powf(0.5)
    }
}

impl Add for Point3D {
    type Output = Point3D;

    fn add(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Mul<isize> for &Point3D {
    type Output = Point3D;

    fn mul(self, scalar: isize) -> Self::Output {
        Point3D {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Add for &Point3D {
    type Output = Point3D;

    fn add(self, rhs: &Point3D) -> Point3D {
        Point3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for &Point3D {
    type Output = Point3D;

    fn sub(self, rhs: Self) -> Self::Output {
        Point3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
