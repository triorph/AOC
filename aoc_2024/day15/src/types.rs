use std::{collections::HashSet, fmt::Display, ops::Add};

use aoc_helpers::point2d::Point2D;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MapTile {
    Wall,
    Box,
    Character,
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RobotMap {
    pub walls: HashSet<WideTile>,
    pub boxes: HashSet<WideTile>,
    pub character: WideTile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WideTile {
    pub start: Point2D,
    pub end: Point2D,
}

impl Add<&Point2D> for &WideTile {
    type Output = WideTile;

    fn add(self, rhs: &Point2D) -> Self::Output {
        WideTile {
            start: self.start + *rhs,
            end: self.end + *rhs,
        }
    }
}

impl WideTile {
    fn new_from_point(point: &Point2D) -> WideTile {
        WideTile {
            start: *point,
            end: *point,
        }
    }

    fn overlaps(&self, other: &WideTile) -> bool {
        self.point_overlaps(&other.start) || self.point_overlaps(&other.end)
    }

    fn point_overlaps(&self, other: &Point2D) -> bool {
        other.x >= self.start.x
            && other.x <= self.end.x
            && other.y >= self.start.y
            && other.y <= self.end.y
    }

    fn as_day_b_wide_tile(&self) -> WideTile {
        WideTile {
            start: Point2D {
                x: self.start.x * 2,
                y: self.start.y,
            },
            end: Point2D {
                x: self.end.x * 2 + 1,
                y: self.end.y,
            },
        }
    }

    fn as_character_day_b_wide_tile(&self) -> WideTile {
        WideTile {
            start: Point2D {
                x: self.start.x * 2,
                y: self.start.y,
            },
            end: Point2D {
                x: self.end.x * 2,
                y: self.end.y,
            },
        }
    }
}

impl RobotMap {
    pub fn from_maptiles(map_tiles: &[Vec<MapTile>]) -> RobotMap {
        let mut walls = HashSet::new();
        let mut boulders = HashSet::new();
        let mut character = None;
        for point in (0..map_tiles.len())
            .cartesian_product(0..map_tiles[0].len())
            .map(|(y, x)| Point2D::from_usize(x, y))
        {
            match map_tiles[point.y as usize][point.x as usize] {
                MapTile::Wall => {
                    walls.insert(WideTile::new_from_point(&point));
                }
                MapTile::Box => {
                    boulders.insert(WideTile::new_from_point(&point));
                }
                MapTile::Character => {
                    character = Some(WideTile::new_from_point(&point));
                }
                _ => (),
            }
        }
        RobotMap {
            walls,
            boxes: boulders,
            character: character.expect("Would find a character"),
        }
    }

    pub fn to_dayb_map(&self) -> RobotMap {
        RobotMap {
            walls: self
                .walls
                .iter()
                .map(|wall| wall.as_day_b_wide_tile())
                .collect(),
            boxes: self
                .boxes
                .iter()
                .map(|robot_box| robot_box.as_day_b_wide_tile())
                .collect(),
            character: self.character.as_character_day_b_wide_tile(),
        }
    }

    pub fn walls_overlapping(&self, widetile: &WideTile) -> Vec<WideTile> {
        self.walls
            .iter()
            .filter(|wall| wall.overlaps(widetile))
            .copied()
            .collect()
    }

    pub fn walls_overlapping_point(&self, point: &Point2D) -> Vec<WideTile> {
        self.walls_overlapping(&WideTile::new_from_point(point))
    }

    pub fn boxes_overlapping(&self, widetile: &WideTile) -> Vec<WideTile> {
        self.boxes
            .iter()
            .filter(|wall| wall.overlaps(widetile))
            .copied()
            .collect()
    }

    pub fn boxes_overlapping_point(&self, point: &Point2D) -> Vec<WideTile> {
        self.boxes_overlapping(&WideTile::new_from_point(point))
    }
}

impl Display for RobotMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = self.walls.iter().map(|p| p.end.x).max().unwrap() + 1;
        let height = self.walls.iter().map(|p| p.end.y).max().unwrap() + 1;
        let mut ret = String::new();
        for y in 0..height {
            for x in 0..width {
                if !self.walls_overlapping_point(&Point2D { x, y }).is_empty() {
                    ret += "#";
                } else if !self.boxes_overlapping_point(&Point2D { x, y }).is_empty() {
                    ret += "O";
                } else if self.character.start.x == x && self.character.start.y == y {
                    ret += "@";
                } else {
                    ret += " ";
                }
            }
            ret += "\n";
        }
        write!(f, "{}", ret)
    }
}

impl Direction {
    pub fn as_point(&self) -> Point2D {
        match self {
            Direction::Up => Point2D { x: 0, y: -1 },
            Direction::Down => Point2D { x: 0, y: 1 },
            Direction::Left => Point2D { x: -1, y: 0 },
            Direction::Right => Point2D { x: 1, y: 0 },
        }
    }
}
