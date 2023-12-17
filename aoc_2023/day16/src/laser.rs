use aoc_helpers::point2d::Point2D;

use crate::room::Room;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Right,
    Left,
    Down,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Laser {
    pub location: Point2D,
    pub direction: Direction,
}

impl Laser {
    pub fn new(location: Point2D, direction: Direction) -> Laser {
        Laser {
            location,
            direction,
        }
    }
    pub fn next_movement(&self) -> Laser {
        Laser::new(
            self.location
                + match self.direction {
                    Direction::Up => Point2D { x: 0, y: -1 },
                    Direction::Right => Point2D { x: 1, y: 0 },
                    Direction::Left => Point2D { x: -1, y: 0 },
                    Direction::Down => Point2D { x: 0, y: 1 },
                },
            self.direction,
        )
    }

    pub fn next_direction(&self, room: &Option<Room>) -> Vec<Laser> {
        match (self.direction, room) {
            (_, None) => vec![],
            (_, Some(Room::Empty)) => vec![*self],
            (Direction::Up, Some(Room::SplitterVertical)) => vec![*self],
            (Direction::Down, Some(Room::SplitterVertical)) => vec![*self],
            (Direction::Left, Some(Room::SplitterHorizontal)) => vec![*self],
            (Direction::Right, Some(Room::SplitterHorizontal)) => vec![*self],
            (Direction::Up, Some(Room::SplitterHorizontal)) => vec![
                Laser::new(self.location, Direction::Left),
                Laser::new(self.location, Direction::Right),
            ],
            (Direction::Down, Some(Room::SplitterHorizontal)) => vec![
                Laser::new(self.location, Direction::Left),
                Laser::new(self.location, Direction::Right),
            ],
            (Direction::Left, Some(Room::SplitterVertical)) => {
                vec![
                    Laser::new(self.location, Direction::Up),
                    Laser::new(self.location, Direction::Down),
                ]
            }
            (Direction::Right, Some(Room::SplitterVertical)) => {
                vec![
                    Laser::new(self.location, Direction::Up),
                    Laser::new(self.location, Direction::Down),
                ]
            }
            (Direction::Up, Some(Room::DiagonalForward)) => {
                vec![Laser::new(self.location, Direction::Right)]
            }
            (Direction::Up, Some(Room::DiagonalBackward)) => {
                vec![Laser::new(self.location, Direction::Left)]
            }
            (Direction::Down, Some(Room::DiagonalForward)) => {
                vec![Laser::new(self.location, Direction::Left)]
            }
            (Direction::Down, Some(Room::DiagonalBackward)) => {
                vec![Laser::new(self.location, Direction::Right)]
            }
            (Direction::Right, Some(Room::DiagonalForward)) => {
                vec![Laser::new(self.location, Direction::Up)]
            }
            (Direction::Right, Some(Room::DiagonalBackward)) => {
                vec![Laser::new(self.location, Direction::Down)]
            }
            (Direction::Left, Some(Room::DiagonalForward)) => {
                vec![Laser::new(self.location, Direction::Down)]
            }
            (Direction::Left, Some(Room::DiagonalBackward)) => {
                vec![Laser::new(self.location, Direction::Up)]
            }
        }
    }
}
