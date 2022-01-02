#[derive(Clone, Debug)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl std::ops::Add for Point {
    type Output = Point;
    fn add(self: Point, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
