mod parser;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use crate::parser::parse_data;
use aoc_helpers::{
    point2d::{Neighbours, Point2D},
    read_input_file, AOCCalculator, AOCFileOrParseError,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day18 {
    walls: Vec<Point2D>,
}

impl AOCCalculator for Day18 {
    fn new(filename: &str) -> Result<Day18, AOCFileOrParseError> {
        Ok(Day18 {
            walls: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!(
            "{}a answer is {:?}",
            name,
            self.calculate_day_a(71, 71, 1024)
        );
        println!(
            "{}b answer is {:?}",
            name,
            self.calculate_day_b(71, 71, 1024)
        );
    }
}

impl Day18 {
    fn within_bounds(&self, point: &Point2D, width: usize, height: usize) -> bool {
        (0..(width as isize)).contains(&point.x) && (0..(height as isize)).contains(&point.y)
    }

    fn explore(&self, width: usize, height: usize, num_walls: usize) -> Option<usize> {
        let mut explore_queue = BinaryHeap::new();
        explore_queue.push(Reverse((0, Point2D::from_usize(0, 0))));
        let mut best_value: HashMap<Point2D, usize> = HashMap::new();
        let walls: HashSet<Point2D> = HashSet::from_iter(self.walls[0..num_walls].iter().cloned());
        while let Some(Reverse((score, location))) = explore_queue.pop() {
            if !best_value.contains_key(&location)
                || best_value
                    .get(&location)
                    .is_some_and(|best_score| score < *best_score)
            {
                best_value.insert(location, score);
                for neighbour in location.get_neighbours().into_iter() {
                    if self.within_bounds(&neighbour, width, height) && !walls.contains(&neighbour)
                    {
                        explore_queue.push(Reverse((score + 1, neighbour)));
                    }
                }
            }
        }
        best_value
            .get(&Point2D::from_usize(width - 1, height - 1))
            .copied()
    }

    fn calculate_day_a(&self, width: usize, height: usize, num_walls: usize) -> usize {
        self.explore(width, height, num_walls)
            .expect("Day a must find a result")
    }

    fn calculate_day_b(&self, width: usize, height: usize, start_wall_size: usize) -> Point2D {
        for i in start_wall_size..=self.walls.len() {
            if self.explore(width, height, i).is_none() {
                return self.walls[i - 1];
            }
        }
        panic!("Must find an answer")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day18 = Day18::new("data/test_data.txt").unwrap();
        let expected = 22;
        let actual = day18.calculate_day_a(7, 7, 12);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day18 = Day18::new("data/test_data.txt").unwrap();
        let expected = Point2D::from_usize(6, 1);
        let actual = day18.calculate_day_b(7, 7, 12);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day18 = Day18::new("data/input_data.txt").unwrap();
        let expected = 298;
        let actual = day18.calculate_day_a(71, 71, 1024);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day18 = Day18::new("data/input_data.txt").unwrap();
        let expected = Point2D::from_usize(52, 32);
        let actual = day18.calculate_day_b(71, 71, 1024);
        assert_eq!(expected, actual);
    }
}
