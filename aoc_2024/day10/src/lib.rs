mod parser;
use std::collections::HashSet;

use crate::parser::parse_data;
use aoc_helpers::{
    point2d::{Neighbours, Point2D},
    read_input_file, AOCCalculator, AOCFileOrParseError,
};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day10 {
    topography: Vec<Vec<usize>>,
}

impl AOCCalculator for Day10 {
    fn new(filename: &str) -> Result<Day10, AOCFileOrParseError> {
        Ok(Day10 {
            topography: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day10 {
    fn within_limit(&self, point: &Point2D) -> bool {
        (0..self.topography.len()).contains(&(point.y as usize))
            && (0..self.topography[0].len()).contains(&(point.x as usize))
    }
    fn get_at_point(&self, point: &Point2D) -> Option<usize> {
        if self.within_limit(point) {
            Some(self.topography[point.y as usize][point.x as usize])
        } else {
            None
        }
    }
    fn iter_all_points(&self) -> Box<dyn Iterator<Item = Point2D>> {
        Box::new(
            (0..self.topography.len())
                .cartesian_product(0..self.topography[0].len())
                .map(|(y, x)| Point2D::from_usize(x, y)),
        )
    }

    fn explore_count_a(&self, point: &Point2D) -> HashSet<Point2D> {
        if self.get_at_point(point) == Some(9) {
            HashSet::from([*point])
        } else {
            point
                .get_neighbours()
                .iter()
                .map(
                    |neighbour| match (self.get_at_point(point), self.get_at_point(neighbour)) {
                        (Some(a), Some(b)) if b == a + 1 => self.explore_count_a(neighbour),
                        _ => HashSet::new(),
                    },
                )
                .fold(HashSet::new(), |acc, x| {
                    x.into_iter().fold(acc, |mut acc2, destination| {
                        acc2.insert(destination);
                        acc2
                    })
                })
        }
    }

    fn calculate_day_a(&self) -> usize {
        self.iter_all_points()
            .filter(|p| self.get_at_point(p) == Some(0))
            .map(|p| self.explore_count_a(&p).len())
            .sum()
    }

    fn explore_count_b(&self, point: &Point2D) -> usize {
        if self.get_at_point(point) == Some(9) {
            1
        } else {
            point
                .get_neighbours()
                .iter()
                .map(
                    |neighbour| match (self.get_at_point(point), self.get_at_point(neighbour)) {
                        (Some(a), Some(b)) if b == a + 1 => self.explore_count_b(neighbour),
                        _ => 0,
                    },
                )
                .sum()
        }
    }

    fn calculate_day_b(&self) -> usize {
        self.iter_all_points()
            .filter(|p| self.get_at_point(p) == Some(0))
            .map(|p| self.explore_count_b(&p))
            .sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day10 = Day10::new("data/test_data.txt").unwrap();
        let expected = 36;
        let actual = day10.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day10 = Day10::new("data/test_data.txt").unwrap();
        let expected = 81;
        let actual = day10.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day10 = Day10::new("data/input_data.txt").unwrap();
        let expected = 496;
        let actual = day10.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day10 = Day10::new("data/input_data.txt").unwrap();
        let expected = 1120;
        let actual = day10.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
