use std::ops::Add;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

impl Direction {
    pub fn get_as_displacement(&self) -> Point {
        match self {
            Direction::Left => Point { x: -1, y: 0 },
            Direction::Right => Point { x: 1, y: 0 },
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Add for &Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
