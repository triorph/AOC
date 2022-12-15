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
        let mut ret = Vec::with_capacity(distance as usize * 4);
        for i in 0..(distance - 1) {
            ret.push(Point {
                x: self.x - distance + i,
                y: self.y - i,
            });
            ret.push(Point {
                x: self.x + i,
                y: self.y - distance + i,
            });
            ret.push(Point {
                x: self.x + distance - i,
                y: self.y + i,
            });
            ret.push(Point {
                x: self.x - i,
                y: self.y + distance - i,
            });
        }
        ret
    }

    pub fn outside_max(&self, max: isize) -> bool {
        !(0..=max).contains(&self.x) || !(0..=max).contains(&self.y)
    }

    pub fn calculate_tuning_frequency(&self) -> isize {
        self.x * 4000000 + self.y
    }
}
