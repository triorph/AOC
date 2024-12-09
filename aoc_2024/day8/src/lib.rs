mod parser;
use std::collections::{HashMap, HashSet};

use crate::parser::parse_data;
use aoc_helpers::{point2d::Point2D, read_input_file, AOCCalculator, AOCFileOrParseError};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day8 {
    data: Vec<Vec<char>>,
}

impl AOCCalculator for Day8 {
    fn new(filename: &str) -> Result<Day8, AOCFileOrParseError> {
        Ok(Day8 {
            data: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day8 {
    fn iter_all_points(&self) -> Box<dyn Iterator<Item = Point2D>> {
        Box::new(
            (0..self.data.len())
                .cartesian_product(0..self.data[0].len())
                .map(|(y, x)| Point2D::from_usize(x, y)),
        )
    }

    fn within_range(&self, point: &Point2D) -> bool {
        (0..self.data.len()).contains(&(point.y as usize))
            && (0..self.data[0].len()).contains(&(point.x as usize))
    }

    fn get_value_at_point(&self, point: &Point2D) -> Option<char> {
        if !self.within_range(point) {
            None
        } else {
            let c = self.data[point.y as usize][point.x as usize];
            if c == '.' {
                None
            } else {
                Some(c)
            }
        }
    }

    fn build_node_map(&self) -> HashMap<char, HashSet<Point2D>> {
        self.iter_all_points()
            .fold(HashMap::new(), |mut node_map, point| {
                if let Some(c) = self.get_value_at_point(&point) {
                    node_map.entry(c).or_default().insert(point);
                }
                node_map
            })
    }

    fn find_antinodes_part_a(&self, nodes: &HashSet<Point2D>) -> HashSet<Point2D> {
        nodes
            .iter()
            .combinations(2)
            .fold(HashSet::new(), |mut acc, x| {
                let diff = *x[0] + x[1] * -1;
                let anti_node1 = *x[0] + diff;
                let anti_node2 = *x[1] + (&diff * -1);
                if self.within_range(&anti_node1) {
                    acc.insert(anti_node1);
                }
                if self.within_range(&anti_node2) {
                    acc.insert(anti_node2);
                }
                acc
            })
    }

    fn find_antinodes_part_b(&self, nodes: &HashSet<Point2D>) -> HashSet<Point2D> {
        nodes
            .iter()
            .combinations(2)
            .fold(HashSet::new(), |mut acc, x| {
                let diff = *x[0] + x[1] * -1;
                let mut count = 0;
                let mut found = true;
                while found {
                    found = false;
                    let anti_node1 = *x[0] + (&diff * count);
                    let anti_node2 = *x[1] + (&diff * -count);
                    if self.within_range(&anti_node1) {
                        found = true;
                        acc.insert(anti_node1);
                    }
                    if self.within_range(&anti_node2) {
                        found = true;
                        acc.insert(anti_node2);
                    }
                    count += 1;
                }
                acc
            })
    }

    fn calculate_day_a(&self) -> usize {
        self.build_node_map()
            .into_values()
            .map(|nodes| self.find_antinodes_part_a(&nodes))
            .fold(HashSet::new(), |mut acc, x| {
                acc.extend(x);
                acc
            })
            .len()
    }

    fn calculate_day_b(&self) -> usize {
        self.build_node_map()
            .into_values()
            .map(|nodes| self.find_antinodes_part_b(&nodes))
            .fold(HashSet::new(), |mut acc, x| {
                acc.extend(x);
                acc
            })
            .len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::hash_utils::FromVec;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_antinodes() {
        let day8 = Day8::new("data/test_data.txt").unwrap();
        let input = HashSet::from_vec(&[Point2D::from_usize(7, 8), Point2D::from_usize(9, 12)]);
        let actual = day8.find_antinodes_part_a(&input);
        // (5,4) is an antinode, (11,16) should be also but is out of bounds
        let expected = HashSet::from_vec(&[Point2D::from_usize(5, 4)]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_a() {
        let day8 = Day8::new("data/test_data.txt").unwrap();
        let expected = 14;
        let actual = day8.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day8 = Day8::new("data/test_data.txt").unwrap();
        let expected = 34;
        let actual = day8.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day8 = Day8::new("data/input_data.txt").unwrap();
        let expected = 273;
        let actual = day8.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day8 = Day8::new("data/input_data.txt").unwrap();
        let expected = 1017;
        let actual = day8.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
