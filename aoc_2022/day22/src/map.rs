use std::ops::Add;

use crate::instruction::Instruction;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum MapTile {
    Floor,
    Wall,
    Wrap,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Map {
    points: Vec<Vec<MapTile>>,
    width: isize,
    height: isize,
}

impl MapTile {
    fn is_wrap(&self) -> bool {
        matches!(self, MapTile::Wrap)
    }

    fn is_floor(&self) -> bool {
        matches!(self, MapTile::Floor)
    }
}

impl Map {
    pub fn new(points: Vec<Vec<MapTile>>) -> Map {
        Map {
            height: points.len() as isize,
            width: points.iter().map(|line| line.len()).max().unwrap() as isize,
            points,
        }
    }

    fn get_at_point(&self, point: &Point) -> MapTile {
        if (0..self.height).contains(&point.y)
            && (0..(self.points[point.y as usize].len() as isize)).contains(&point.x)
        {
            self.points[point.y as usize][point.x as usize]
        } else {
            MapTile::Wrap
        }
    }

    fn get_min_x(&self, y: isize) -> isize {
        for x in 0..self.width {
            let point = Point { x, y };
            if !self.get_at_point(&point).is_wrap() {
                return x;
            }
        }
        panic!("no min x found");
    }

    fn get_max_x(&self, y: isize) -> isize {
        for x in (0..self.width).rev() {
            let point = Point { x, y };
            if !self.get_at_point(&point).is_wrap() {
                return x;
            }
        }
        panic!("no max x found");
    }

    fn get_min_y(&self, x: isize) -> isize {
        for y in 0..self.height {
            let point = Point { x, y };
            if !self.get_at_point(&point).is_wrap() {
                return y;
            }
        }
        panic!("no min y found");
    }

    fn get_max_y(&self, x: isize) -> isize {
        for y in (0..self.height).rev() {
            let point = Point { x, y };
            if !self.get_at_point(&point).is_wrap() {
                return y;
            }
        }
        panic!("no max y found");
    }

    pub fn get_mapped_point_day_a(&self, point: &Point, direction: &Direction) -> Point {
        match direction {
            Direction::Up => Point {
                x: point.x,
                y: self.get_max_y(point.x),
            },
            Direction::Right => Point {
                x: self.get_min_x(point.y),
                y: point.y,
            },
            Direction::Down => Point {
                x: point.x,
                y: self.get_min_y(point.x),
            },
            Direction::Left => Point {
                x: self.get_max_x(point.y),
                y: point.y,
            },
        }
    }

    pub fn get_starting_position(&self) -> Position {
        Position {
            coord: Point {
                x: self.get_min_x(0),
                y: 0,
            },
            direction: Direction::Right,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn rotate_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn rotate_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Down,
        }
    }

    fn as_coordinate(&self) -> Point {
        match self {
            Direction::Up => Point { x: 0, y: -1 },
            Direction::Left => Point { x: -1, y: 0 },
            Direction::Down => Point { x: 0, y: 1 },
            Direction::Right => Point { x: 1, y: 0 },
        }
    }

    fn get_password(&self) -> usize {
        match self {
            Direction::Up => 3,
            Direction::Left => 2,
            Direction::Down => 1,
            Direction::Right => 0,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Position {
    pub coord: Point,
    pub direction: Direction,
}

impl Position {
    fn can_move_on_map(&self, map: &Map) -> bool {
        let new_position = &self.coord + &self.direction.as_coordinate();
        let mut val = map.get_at_point(&new_position);
        if val.is_wrap() {
            let new_position = map.get_mapped_point_day_a(&new_position, &self.direction);
            val = map.get_at_point(&new_position);
        }
        val.is_floor()
    }

    fn move_forward_one_day_a(&mut self, map: &Map) {
        if self.can_move_on_map(map) {
            self.coord = &self.coord + &self.direction.as_coordinate();
            if map.get_at_point(&self.coord).is_wrap() {
                self.coord = map.get_mapped_point_day_a(&self.coord, &self.direction);
            }
        }
    }

    pub fn run_instruction_day_a(&mut self, map: &Map, instruction: &Instruction) {
        match instruction {
            Instruction::RotateLeft => self.direction = self.direction.rotate_left(),
            Instruction::RotateRight => self.direction = self.direction.rotate_right(),
            Instruction::MoveForward(n) => {
                for _ in 0..*n {
                    self.move_forward_one_day_a(map)
                }
            }
        }
    }

    pub fn get_password(&self) -> usize {
        ((1000 * (self.coord.y + 1) + 4 * (self.coord.x + 1)) as usize)
            + self.direction.get_password()
    }
}

impl Add for &Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
