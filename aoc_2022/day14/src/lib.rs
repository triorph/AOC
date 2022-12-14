mod parser;
use std::collections::HashSet;

use crate::parser::parse_data;
mod point;
mod wall;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};
use wall::Walls;

use self::point::Point;

pub struct Day14 {
    walls: Walls,
}

impl AOCCalculator for Day14 {
    fn new(filename: &str) -> Result<Day14, AOCFileOrParseError> {
        Ok(Day14 {
            walls: Walls::new(&parse_data(&read_input_file(filename)?)?),
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
        let mut last_path = vec![Point { x: 500, y: 0 }];
        let max_point = self.walls.get_max_y();
        while let Some(destination) = self.drop_point_day_a(&last_path, &sands, max_point) {
            let last_entry = &destination[destination.len() - 1];
            sands.insert(last_entry.clone());
            last_path = destination;
        }
        sands.len()
    }

    fn drop_point_day_a(
        &self,
        previous_path: &[Point],
        sands: &HashSet<Point>,
        max_point: isize,
    ) -> Option<Vec<Point>> {
        let mut path: Vec<Point> = previous_path.to_vec();
        while self
            .find_next_place_day_a(&path[path.len() - 1], sands)
            .is_none()
        {
            path.pop();
        }
        while path[path.len() - 1].is_on_board(max_point) {
            if let Some(point) = self.find_next_place_day_a(&path[path.len() - 1], sands) {
                path.push(point)
            } else {
                return Some(path);
            }
        }
        None
    }

    fn find_next_place_day_a(&self, point: &Point, sands: &HashSet<Point>) -> Option<Point> {
        for contender in point.get_next_contenders().iter() {
            if !sands.contains(contender) && !self.walls.intersects_with(contender) {
                return Some(contender.clone());
            }
        }
        None
    }

    fn calculate_day_b(&self) -> usize {
        let mut sands: HashSet<Point> = HashSet::new();
        let mut last_path = vec![Point { x: 500, y: 0 }];
        let max_point = self.walls.get_max_y();
        while let Some(destination) = self.drop_point_day_b(&last_path, &sands, max_point) {
            let last_entry = &destination[destination.len() - 1];
            sands.insert(last_entry.clone());
            last_path = destination;
        }
        sands.len() + 1 // include the starting point
    }

    fn drop_point_day_b(
        &self,
        previous_path: &[Point],
        sands: &HashSet<Point>,
        max_point: isize,
    ) -> Option<Vec<Point>> {
        let mut path: Vec<Point> = previous_path.to_vec();
        while !path.is_empty()
            && self
                .find_next_place_day_b(&path[path.len() - 1], sands, max_point)
                .is_none()
        {
            path.pop();
        }
        if path.is_empty() {
            return None;
        }
        loop {
            if let Some(point) = self.find_next_place_day_b(&path[path.len() - 1], sands, max_point)
            {
                path.push(point)
            } else if path.is_empty() {
                return None;
            } else {
                return Some(path);
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
                && !self.walls.intersects_with(contender)
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
        let expected = 26139;
        let actual = day14.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
