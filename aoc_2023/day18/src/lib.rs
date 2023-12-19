mod direction;
mod parser;
use itertools::Itertools;

use crate::direction::Direction;
use crate::parser::parse_data;
use aoc_helpers::{point2d::Point2D, read_input_file, AOCCalculator, AOCFileOrParseError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day18 {
    dig_plan: Vec<(Direction, usize, usize)>,
}

impl AOCCalculator for Day18 {
    fn new(filename: &str) -> Result<Day18, AOCFileOrParseError> {
        Ok(Day18 {
            dig_plan: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

fn shoelace_area(corners: &[Point2D]) -> isize {
    (corners
        .iter()
        .circular_tuple_windows()
        .map(|(a, b)| a.x * b.y - b.x * a.y)
        .sum::<isize>())
    .abs()
        / 2
}

fn perimeter(corners: &[Point2D]) -> usize {
    corners
        .iter()
        .circular_tuple_windows()
        .map(|(a, b)| a.get_manhattan_distance(b))
        .sum::<usize>()
}

fn convert_colour_to_instruction(colour: usize) -> (Direction, usize) {
    (
        match colour % 16 {
            0 => Direction::Right,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Up,
            _ => panic!("Invalid direction pointer"),
        },
        colour / 16,
    )
}

impl Day18 {
    fn calculate_day_a(&self) -> usize {
        let mut corners = Vec::new();
        let mut last_point = Point2D { x: 0, y: 0 };
        corners.push(last_point);
        for (direction, length, _) in self.dig_plan.iter() {
            last_point = last_point + &direction.as_point_delta() * (*length as isize);
            corners.push(last_point);
        }
        shoelace_area(&corners) as usize + perimeter(&corners) / 2 + 1
    }

    fn calculate_day_b(&self) -> usize {
        let mut corners = Vec::new();
        let mut last_point = Point2D { x: 0, y: 0 };
        corners.push(last_point);
        for (_, _, colour) in self.dig_plan.iter() {
            let (direction, length) = convert_colour_to_instruction(*colour);
            last_point = last_point + &direction.as_point_delta() * (length as isize);
            corners.push(last_point);
        }
        shoelace_area(&corners) as usize + perimeter(&corners) / 2 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day18 = Day18::new("data/test_data.txt").unwrap();
        let expected = 62;
        let actual = day18.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day18 = Day18::new("data/test_data.txt").unwrap();
        let expected = 952408144115;
        let actual = day18.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day18 = Day18::new("data/input_data.txt").unwrap();
        let expected = 33491;
        let actual = day18.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day18 = Day18::new("data/input_data.txt").unwrap();
        let expected = 87716969654406;
        let actual = day18.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_shoelace_area_basic() {
        let actual = shoelace_area(&[
            Point2D { x: 0, y: 0 },
            Point2D { x: 0, y: 2 },
            Point2D { x: 2, y: 2 },
            Point2D { x: 2, y: 0 },
        ]);
        assert_eq!(actual, 4);
    }

    #[test]
    fn test_shoelace_area_slightly_more_complicated() {
        let actual = shoelace_area(&[
            Point2D { x: 0, y: 0 },
            Point2D { x: 0, y: 4 },
            Point2D { x: 2, y: 4 },
            Point2D { x: 2, y: 2 },
            Point2D { x: 4, y: 2 },
            Point2D { x: 4, y: 0 },
        ]);
        assert_eq!(actual, 12);
    }
}
