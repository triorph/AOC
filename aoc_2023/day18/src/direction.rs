use aoc_helpers::point2d::Point2D;

#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn as_point_delta(&self) -> Point2D {
        match self {
            Direction::Up => Point2D { x: 0, y: -1 },
            Direction::Down => Point2D { x: 0, y: 1 },
            Direction::Left => Point2D { x: -1, y: 0 },
            Direction::Right => Point2D { x: 1, y: 0 },
        }
    }
}
