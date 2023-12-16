mod parser;
use std::collections::HashSet;

use crate::parser::parse_data;
use aoc_helpers::{point2d::Point2D, read_input_file, AOCCalculator, AOCFileOrParseError};
use parser::Room;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day16 {
    layout: Vec<Vec<Room>>,
}

impl AOCCalculator for Day16 {
    fn new(filename: &str) -> Result<Day16, AOCFileOrParseError> {
        Ok(Day16 {
            layout: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Left,
    Down,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Movement(Point2D, Direction);

impl Movement {
    fn next_movement(&self) -> Movement {
        Movement(
            self.0
                + match self.1 {
                    Direction::Up => Point2D { x: 0, y: -1 },
                    Direction::Right => Point2D { x: 1, y: 0 },
                    Direction::Left => Point2D { x: -1, y: 0 },
                    Direction::Down => Point2D { x: 0, y: 1 },
                },
            self.1,
        )
    }

    fn next_direction(&self, room: &Option<Room>) -> Vec<Movement> {
        match (self.1, room) {
            (_, None) => vec![],
            (_, Some(Room::Empty)) => vec![*self],
            (Direction::Up, Some(Room::SplitterVertical)) => vec![*self],
            (Direction::Down, Some(Room::SplitterVertical)) => vec![*self],
            (Direction::Left, Some(Room::SplitterHorizontal)) => vec![*self],
            (Direction::Right, Some(Room::SplitterHorizontal)) => vec![*self],
            (Direction::Up, Some(Room::SplitterHorizontal)) => vec![
                Movement(self.0, Direction::Left),
                Movement(self.0, Direction::Right),
            ],
            (Direction::Down, Some(Room::SplitterHorizontal)) => vec![
                Movement(self.0, Direction::Left),
                Movement(self.0, Direction::Right),
            ],
            (Direction::Left, Some(Room::SplitterVertical)) => vec![
                Movement(self.0, Direction::Up),
                Movement(self.0, Direction::Down),
            ],
            (Direction::Right, Some(Room::SplitterVertical)) => vec![
                Movement(self.0, Direction::Up),
                Movement(self.0, Direction::Down),
            ],
            (Direction::Up, Some(Room::DiagonalForward)) => {
                vec![Movement(self.0, Direction::Right)]
            }
            (Direction::Up, Some(Room::DiagonalBackward)) => {
                vec![Movement(self.0, Direction::Left)]
            }
            (Direction::Down, Some(Room::DiagonalForward)) => {
                vec![Movement(self.0, Direction::Left)]
            }
            (Direction::Down, Some(Room::DiagonalBackward)) => {
                vec![Movement(self.0, Direction::Right)]
            }
            (Direction::Right, Some(Room::DiagonalForward)) => {
                vec![Movement(self.0, Direction::Up)]
            }
            (Direction::Right, Some(Room::DiagonalBackward)) => {
                vec![Movement(self.0, Direction::Down)]
            }
            (Direction::Left, Some(Room::DiagonalForward)) => {
                vec![Movement(self.0, Direction::Down)]
            }
            (Direction::Left, Some(Room::DiagonalBackward)) => {
                vec![Movement(self.0, Direction::Up)]
            }
        }
    }
}

impl Day16 {
    fn room_at_point(&self, point: &Point2D) -> Option<Room> {
        if (0..self.layout.len() as isize).contains(&point.y)
            && (0..self.layout[point.y as usize].len() as isize).contains(&point.x)
        {
            Some(self.layout[point.y as usize][point.x as usize])
        } else {
            None
        }
    }

    fn room_at_movement(&self, movement: &Movement) -> Option<Room> {
        self.room_at_point(&movement.0)
    }

    fn get_energy_for_starting_movement(&self, starting_movement: &Movement) -> usize {
        let mut visited_movements: HashSet<Movement> = HashSet::new();
        let mut movements = vec![*starting_movement];
        while !movements.is_empty() {
            movements = movements
                .iter()
                .map(|movement| movement.next_movement())
                .flat_map(|movement| movement.next_direction(&self.room_at_movement(&movement)))
                .collect();
            movements.retain(|movement| !visited_movements.contains(movement));
            visited_movements.extend(&movements);
        }
        HashSet::<Point2D>::from_iter(visited_movements.into_iter().map(|movement| movement.0))
            .len()
    }

    fn calculate_day_a(&self) -> usize {
        self.get_energy_for_starting_movement(&Movement(Point2D { x: -1, y: 0 }, Direction::Right))
    }

    #[allow(dead_code)]
    fn print_visited_tiles(&self, visited: &HashSet<Point2D>) -> String {
        let mut ret = "".to_string();
        for y in 0..self.layout.len() {
            for x in 0..self.layout[y].len() {
                ret += if visited.contains(&Point2D {
                    x: x as isize,
                    y: y as isize,
                }) {
                    "#"
                } else {
                    " "
                }
            }
            ret += "\n"
        }
        ret
    }

    fn get_day_b_possible_starting_movements(&self) -> Vec<Movement> {
        let mut ret = Vec::new();
        for y in 0..self.layout.len() {
            ret.push(Movement(
                Point2D {
                    x: -1,
                    y: y as isize,
                },
                Direction::Right,
            ));
            ret.push(Movement(
                Point2D {
                    x: self.layout[y].len() as isize,
                    y: y as isize,
                },
                Direction::Left,
            ));
        }
        for x in 0..self.layout[0].len() {
            ret.push(Movement(
                Point2D {
                    x: x as isize,
                    y: -1,
                },
                Direction::Down,
            ));
            ret.push(Movement(
                Point2D {
                    x: x as isize,
                    y: self.layout[0].len() as isize,
                },
                Direction::Down,
            ));
        }
        ret
    }

    fn calculate_day_b(&self) -> usize {
        self.get_day_b_possible_starting_movements()
            .iter()
            .map(|starting_movement| self.get_energy_for_starting_movement(starting_movement))
            .max()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day16 = Day16::new("data/test_data.txt").unwrap();
        let expected = 46;
        let actual = day16.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day16 = Day16::new("data/test_data.txt").unwrap();
        let expected = 51;
        let actual = day16.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day16 = Day16::new("data/input_data.txt").unwrap();
        let expected = 7498;
        let actual = day16.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day16 = Day16::new("data/input_data.txt").unwrap();
        let expected = 7846;
        let actual = day16.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
