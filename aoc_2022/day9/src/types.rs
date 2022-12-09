#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Point(pub isize, pub isize);

impl Point {
    pub fn follow_other(self, other: &Point, distance: usize) -> Point {
        if self.calculate_distance(other) > distance {
            self.move_1_towards(other)
        } else {
            self
        }
    }

    pub fn get_steps(&self) -> Vec<Point> {
        let (quantity, direction) = if self.0 != 0 {
            let quantity = self.0.abs();
            let direction = self.0 / quantity;
            (quantity, Point(direction, 0))
        } else {
            let quantity = self.1.abs();
            let direction = self.1 / quantity;
            (quantity, Point(0, direction))
        };
        (0..quantity).map(|_| direction.clone()).collect()
    }

    fn calculate_distance(&self, other: &Point) -> usize {
        *[self.0.abs_diff(other.0), self.1.abs_diff(other.1)]
            .iter()
            .max()
            .unwrap()
    }

    fn move_1_towards(self, other: &Point) -> Point {
        let x = if other.0 < self.0 {
            self.0 - 1
        } else if other.0 > self.0 {
            self.0 + 1
        } else {
            self.0
        };
        let y = if other.1 < self.1 {
            self.1 - 1
        } else if other.1 > self.1 {
            self.1 + 1
        } else {
            self.1
        };
        Point(x, y)
    }
}

impl std::ops::Add<&Point> for &Point {
    type Output = Point;
    fn add(self, other: &Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
}
