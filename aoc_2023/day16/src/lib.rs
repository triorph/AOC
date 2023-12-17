mod laser;
mod parser;
mod room;
use crate::laser::{Direction, Laser};
use std::collections::HashSet;

use crate::parser::parse_data;
use crate::room::Room;
use aoc_helpers::{point2d::Point2D, read_input_file, AOCCalculator, AOCFileOrParseError};

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

    fn room_at_laser(&self, laser: &Laser) -> Option<Room> {
        self.room_at_point(&laser.location)
    }

    fn get_energy_from_starting_laser(&self, starting_laser: &Laser) -> usize {
        let mut visited_lasers: HashSet<Laser> = HashSet::new();
        let mut lasers = vec![*starting_laser];
        while !lasers.is_empty() {
            lasers = lasers
                .iter()
                .map(|movement| movement.next_movement())
                .flat_map(|movement| movement.next_direction(&self.room_at_laser(&movement)))
                .collect();
            lasers.retain(|movement| !visited_lasers.contains(movement));
            visited_lasers.extend(&lasers);
        }
        HashSet::<Point2D>::from_iter(visited_lasers.into_iter().map(|laser| laser.location)).len()
    }

    fn calculate_day_a(&self) -> usize {
        self.get_energy_from_starting_laser(&Laser::new(Point2D { x: -1, y: 0 }, Direction::Right))
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

    fn get_day_b_possible_starting_lasers(&self) -> Vec<Laser> {
        let mut ret = Vec::new();
        for y in 0..self.layout.len() {
            ret.push(Laser::new(
                Point2D {
                    x: -1,
                    y: y as isize,
                },
                Direction::Right,
            ));
            ret.push(Laser::new(
                Point2D {
                    x: self.layout[y].len() as isize,
                    y: y as isize,
                },
                Direction::Left,
            ));
        }
        for x in 0..self.layout[0].len() {
            ret.push(Laser::new(
                Point2D {
                    x: x as isize,
                    y: -1,
                },
                Direction::Down,
            ));
            ret.push(Laser::new(
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
        self.get_day_b_possible_starting_lasers()
            .iter()
            .map(|starting_movement| self.get_energy_from_starting_laser(starting_movement))
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
