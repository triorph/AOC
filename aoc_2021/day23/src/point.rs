#[derive(Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }

    pub fn manhattan_distance(&self, other: &Point) -> usize {
        // Not technically manhattan distance, as we always sum the y values.
        (self.x as isize - other.x as isize).abs() as usize
            + (self.y as isize + other.y as isize).abs() as usize
    }
}
