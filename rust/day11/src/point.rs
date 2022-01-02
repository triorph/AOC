#[derive(Clone, Debug)]
pub struct Point {
    x: isize,
    y: isize,
}

impl<'a> Point {
    pub fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }

    pub fn out_of_bounds(&self) -> bool {
        self.x < 0 || self.x >= 10 || self.y < 0 || self.y >= 10
    }

    pub fn get_as_index(&self) -> usize {
        (self.y * 10 + self.x) as usize
    }

    pub fn get_neighbours(&'a self) -> Box<dyn Iterator<Item = Point> + 'a> {
        Box::new(
            [
                Point { x: -1, y: -1 },
                Point { x: 0, y: -1 },
                Point { x: 1, y: -1 },
                Point { x: -1, y: 0 },
                Point { x: 1, y: 0 },
                Point { x: -1, y: 1 },
                Point { x: 0, y: 1 },
                Point { x: 1, y: 1 },
            ]
            .iter()
            .map(move |p| self + p),
        )
    }
}

impl std::ops::Add for &Point {
    type Output = Point;
    fn add(self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
