#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn manhattan_distance(&self, other: &Point) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn get_all_points_at_manhattan_distance(&self, distance: isize) -> Vec<Point> {
        let mut ret = Vec::new();
        for i in 0..distance {
            // The way i've written this probably causes dupes at the corners, but that's fine.
            ret.push(Point {
                x: self.x - distance + i,
                y: self.y - i,
            });
            ret.push(Point {
                x: self.x + distance - i,
                y: self.y - i,
            });
            ret.push(Point {
                x: self.x - distance + i,
                y: self.y + i,
            });
            ret.push(Point {
                x: self.x + distance - i,
                y: self.y + i,
            });
        }
        ret
    }
}
