#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl std::ops::Add for &Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Point {
    pub fn distance_to(&self, other: &Point) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }
}
