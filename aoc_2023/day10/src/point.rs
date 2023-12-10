use crate::pipe::Pipe;
use std::ops::Add;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn neighbours(&self, pipe: &Pipe) -> Vec<Point> {
        pipe.neighbours().into_iter().map(|x| x + *self).collect()
    }

    pub fn all_neighbours(&self) -> Vec<Point> {
        self.neighbours(&Pipe::Start)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
