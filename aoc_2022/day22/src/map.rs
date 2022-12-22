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

    fn move_forward_one_day_b(&mut self, map: &Map) {
        let (next_point, next_direction) = self.get_next_point_day_b();
        let map_value = map.get_at_point(&next_point);
        if map_value.is_wrap() {
            // sanity check
            panic!(
                "next_point {:?} wrapped incorrectly from {:?} {:?}",
                next_point, self.coord, self.direction
            );
        }
        if map_value.is_floor() {
            self.coord = next_point;
            self.direction = next_direction;
        }
    }

    fn get_next_point_day_b(&mut self) -> (Point, Direction) {
        match (self.coord, self.direction) {
            (coord, Direction::Up) if coord.is_edge_a() => {
                let delta = coord.x - 50;
                (
                    Point {
                        x: 0,
                        y: 150 + delta,
                    },
                    Direction::Right,
                )
            }
            (coord, Direction::Up) if coord.is_edge_b() => {
                let delta = coord.x - 100;
                // Pretty sure we don't flip x here
                (Point { x: delta, y: 199 }, Direction::Up)
            }
            (coord, Direction::Right) if coord.is_edge_c() => {
                let delta = coord.y;
                (
                    Point {
                        x: 99,
                        y: 149 - delta,
                    },
                    Direction::Left,
                )
            }
            (coord, Direction::Left) if coord.is_edge_d() => {
                let delta = coord.y;
                (
                    Point {
                        x: 0,
                        y: 149 - delta,
                    },
                    Direction::Right,
                )
            }
            (coord, Direction::Left) if coord.is_edge_e() => {
                let delta = coord.y - 50;
                (Point { x: delta, y: 100 }, Direction::Down)
            }
            (coord, Direction::Right) if coord.is_edge_f() => {
                let delta = coord.y - 50;
                (
                    Point {
                        x: 100 + delta,
                        y: 49,
                    },
                    Direction::Up,
                )
            }
            (coord, Direction::Up) if coord.is_edge_g() => {
                let delta = coord.x;
                (
                    Point {
                        x: 50,
                        y: 50 + delta,
                    },
                    Direction::Right,
                )
            }
            (coord, Direction::Left) if coord.is_edge_h() => {
                let delta = coord.y - 100;
                (
                    Point {
                        x: 50,
                        y: 49 - delta,
                    },
                    Direction::Right,
                )
            }
            (coord, Direction::Right) if coord.is_edge_i() => {
                let delta = coord.y - 100;
                (
                    Point {
                        x: 149,
                        y: 49 - delta,
                    },
                    Direction::Left,
                )
            }
            (coord, Direction::Down) if coord.is_edge_j() => {
                let delta = coord.x - 50;
                (
                    Point {
                        x: 49,
                        y: 150 + delta,
                    },
                    Direction::Left,
                )
            }
            (coord, Direction::Left) if coord.is_edge_k() => {
                let delta = coord.y - 150;
                (
                    Point {
                        x: 50 + delta,
                        y: 0,
                    },
                    Direction::Down,
                )
            }
            (coord, Direction::Right) if coord.is_edge_l() => {
                let delta = coord.y - 150;
                (
                    Point {
                        x: 50 + delta,
                        y: 149,
                    },
                    Direction::Up,
                )
            }
            (coord, Direction::Down) if coord.is_edge_m() => {
                let delta = coord.x;
                // Pretty sure we don't flip this
                (
                    Point {
                        x: 100 + delta,
                        y: 0,
                    },
                    Direction::Down,
                )
            }
            (coord, Direction::Down) if coord.is_edge_n() => {
                let delta = coord.x - 100;
                (
                    Point {
                        x: 99,
                        y: 50 + delta,
                    },
                    Direction::Left,
                )
            }
            (coord, direction) => (&coord + &direction.as_coordinate(), direction),
        }
    }

    pub fn run_instruction(&mut self, map: &Map, instruction: &Instruction, is_day_b: bool) {
        match instruction {
            Instruction::RotateLeft => self.direction = self.direction.rotate_left(),
            Instruction::RotateRight => self.direction = self.direction.rotate_right(),
            Instruction::MoveForward(n) => {
                for _ in 0..*n {
                    if is_day_b {
                        self.move_forward_one_day_b(map)
                    } else {
                        self.move_forward_one_day_a(map)
                    }
                }
            }
        }
    }

    pub fn get_password(&self) -> usize {
        ((1000 * (self.coord.y + 1) + 4 * (self.coord.x + 1)) as usize)
            + self.direction.get_password()
    }
}

impl Point {
    fn is_edge_a(&self) -> bool {
        (50..=99).contains(&self.x) && self.y == 0
    }

    fn is_edge_b(&self) -> bool {
        (100..=149).contains(&self.x) && self.y == 0
    }

    fn is_edge_c(&self) -> bool {
        self.x == 149 && (0..=49).contains(&self.y)
    }

    fn is_edge_d(&self) -> bool {
        self.x == 50 && (0..=49).contains(&self.y)
    }

    fn is_edge_e(&self) -> bool {
        self.x == 50 && (50..=99).contains(&self.y)
    }

    fn is_edge_f(&self) -> bool {
        self.x == 99 && (50..=99).contains(&self.y)
    }

    fn is_edge_g(&self) -> bool {
        (0..=49).contains(&self.x) && self.y == 100
    }

    fn is_edge_h(&self) -> bool {
        self.x == 0 && (100..=149).contains(&self.y)
    }

    fn is_edge_i(&self) -> bool {
        self.x == 99 && (100..=149).contains(&self.y)
    }

    fn is_edge_j(&self) -> bool {
        (50..=149).contains(&self.x) && self.y == 149
    }

    fn is_edge_k(&self) -> bool {
        self.x == 0 && (150..=199).contains(&self.y)
    }

    fn is_edge_l(&self) -> bool {
        self.x == 49 && (150..=199).contains(&self.y)
    }

    fn is_edge_m(&self) -> bool {
        (0..=49).contains(&self.x) && self.y == 199
    }

    fn is_edge_n(&self) -> bool {
        (100..=149).contains(&self.x) && self.y == 49
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
