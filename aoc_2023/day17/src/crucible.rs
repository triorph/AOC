use aoc_helpers::point2d::Point2D;
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub struct Crucible {
    dir: Direction,
    pub length: usize,
    pub min: usize,
    max: usize,
}

impl Direction {
    fn as_point_delta(&self) -> Point2D {
        match self {
            Direction::Up => Point2D { x: 0, y: -1 },
            Direction::Left => Point2D { x: -1, y: 0 },
            Direction::Right => Point2D { x: 1, y: 0 },
            Direction::Down => Point2D { x: 0, y: 1 },
        }
    }
}

impl Crucible {
    fn init(&self, dir: Direction) -> Crucible {
        Crucible {
            dir,
            length: 1,
            min: self.min,
            max: self.max,
        }
    }

    fn one_larger(&self) -> Crucible {
        Crucible {
            dir: self.dir,
            length: self.length + 1,
            min: self.min,
            max: self.max,
        }
    }

    fn get_neighbours(&self) -> Vec<Crucible> {
        let mut ret = Vec::new();
        if self.length < self.max {
            ret.push(self.one_larger())
        }
        if self.length >= self.min {
            ret.extend(match self.dir {
                Direction::Up => vec![self.init(Direction::Left), self.init(Direction::Right)],
                Direction::Left => vec![self.init(Direction::Up), self.init(Direction::Down)],
                Direction::Right => vec![self.init(Direction::Up), self.init(Direction::Down)],
                Direction::Down => vec![self.init(Direction::Left), self.init(Direction::Right)],
            });
        }
        ret
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CrucibleLocation {
    pub crucible: Crucible,
    pub location: Point2D,
}

impl CrucibleLocation {
    pub fn get_starting_crucible_location(min: usize, max: usize) -> CrucibleLocation {
        CrucibleLocation {
            crucible: Crucible {
                dir: Direction::Right,
                length: 0,
                min,
                max,
            },
            location: Point2D { x: 0, y: 0 },
        }
    }

    pub fn get_neighbours(&self) -> Vec<CrucibleLocation> {
        self.crucible
            .get_neighbours()
            .into_iter()
            .map(|neighbour_crucible| CrucibleLocation {
                crucible: neighbour_crucible,
                location: self.location + neighbour_crucible.dir.as_point_delta(),
            })
            .collect()
    }

    pub fn is_at_end(&self, end: &Point2D) -> bool {
        self.crucible.length >= self.crucible.min && &self.location == end
    }
}
