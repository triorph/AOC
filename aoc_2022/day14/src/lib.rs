mod parser;
use std::collections::HashSet;

use crate::parser::parse_data;
mod point;
mod wall;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};
use wall::Wall;

use self::point::Point;

pub struct Day14 {
    walls: Vec<Wall>,
}

impl AOCCalculator for Day14 {
    fn new(filename: &str) -> Result<Day14, AOCFileOrParseError> {
        Ok(Day14 {
            walls: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day14 {
    fn calculate_day_a(&self) -> usize {
        let mut sands: HashSet<Point> = HashSet::new();
        let starting_point = Point { x: 500, y: 0 };
        let max_point = self.walls.iter().map(Wall::get_max_y).max().unwrap();
        loop {
            let final_point = self.drop_point_day_a(&starting_point.clone(), &sands, max_point);
            if let Some(final_point) = final_point {
                sands.insert(final_point);
            } else {
                break;
            }
        }
        sands.len()
    }

    fn drop_point_day_a(
        &self,
        starting_point: &Point,
        sands: &HashSet<Point>,
        max_point: isize,
    ) -> Option<Point> {
        let mut next_point = starting_point.clone();
        while next_point.is_on_board(max_point) {
            if let Some(point) = self.find_next_place_day_a(&next_point, sands) {
                next_point = point
            } else {
                return Some(next_point);
            }
        }
        None
    }

    fn walls_intersect_with(&self, point: &Point) -> bool {
        self.walls.iter().any(|wall| wall.intersects_with(point))
    }

    fn find_next_place_day_a(&self, point: &Point, sands: &HashSet<Point>) -> Option<Point> {
        for contender in point.get_next_contenders().iter() {
            if !sands.contains(contender) && !self.walls_intersect_with(contender) {
                return Some(contender.clone());
            }
        }
        None
    }

    fn calculate_day_b(&self) -> usize {
        let mut sands: HashSet<Point> = HashSet::new();
        let starting_point = Point { x: 500, y: 0 };
        let max_point = self.walls.iter().map(Wall::get_max_y).max().unwrap();
        loop {
            let final_point = self.drop_point_day_b(&starting_point.clone(), &sands, max_point);
            if let Some(final_point) = final_point {
                if sands.contains(&final_point) {
                    break;
                }
                sands.insert(final_point);
            } else {
                break;
            }
        }
        sands.len()
    }

    fn drop_point_day_b(
        &self,
        starting_point: &Point,
        sands: &HashSet<Point>,
        max_point: isize,
    ) -> Option<Point> {
        let mut next_point = starting_point.clone();
        loop {
            if let Some(point) = self.find_next_place_day_b(&next_point, sands, max_point) {
                next_point = point
            } else {
                return Some(next_point);
            }
        }
    }

    fn find_next_place_day_b(
        &self,
        point: &Point,
        sands: &HashSet<Point>,
        max_point: isize,
    ) -> Option<Point> {
        for contender in point.get_next_contenders().iter() {
            if !sands.contains(contender)
                && !self.walls_intersect_with(contender)
                && point.y < max_point + 1
            {
                return Some(contender.clone());
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day14 = Day14::new("data/test_data.txt").unwrap();
        let expected = 24;
        let actual = day14.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_a_real_input() {
        let day14 = Day14::new("data/input_data.txt").unwrap();
        let expected = 1330;
        let actual = day14.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day14 = Day14::new("data/test_data.txt").unwrap();
        let expected = 93;
        let actual = day14.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b_real_input() {
        let day14 = Day14::new("data/input_data.txt").unwrap();
        let expected = 93;
        let actual = day14.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
