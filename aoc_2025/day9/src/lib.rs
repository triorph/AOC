mod parser;
use std::ops::RangeInclusive;

use crate::parser::parse_data;
use aoc_helpers::{point2d::Point2D, read_input_file, AOCCalculator, AOCFileOrParseError};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day9 {
    data: Vec<Point2D>,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl AOCCalculator for Day9 {
    fn new(filename: &str) -> Result<Day9, AOCFileOrParseError> {
        Ok(Day9 {
            data: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day9 {
    fn calculate_day_a(&self) -> usize {
        self.data
            .iter()
            .cartesian_product(self.data.iter())
            .filter(|(a, b)| a != b)
            .map(|(a, b)| self.get_area_of_points(a, b))
            .max()
            .unwrap()
    }

    fn calculate_day_b(&self) -> usize {
        let lines = self.get_edges();
        self.data
            .iter()
            .cartesian_product(self.data.iter())
            .filter(|(a, b)| a != b)
            .filter(|(a, b)| self.is_inner_rectangle(&lines, a, b))
            .map(|(a, b)| self.get_area_of_points(a, b))
            .max()
            .unwrap()
    }

    fn get_area_of_points(&self, a: &Point2D, b: &Point2D) -> usize {
        ((a.x - b.x).unsigned_abs() + 1) * ((a.y - b.y).unsigned_abs() + 1)
    }

    fn get_edges(&self) -> Vec<(Point2D, Point2D)> {
        [
            self.data
                .windows(2)
                .map(|x| (x[0], x[1]))
                .collect::<Vec<(Point2D, Point2D)>>(),
            vec![(self.data[self.data.len() - 1], self.data[0])],
        ]
        .concat()
    }

    fn is_inner_rectangle(&self, edges: &[(Point2D, Point2D)], a: &Point2D, b: &Point2D) -> bool {
        let top_left = Point2D {
            x: a.x.min(b.x),
            y: a.y.min(b.y),
        };
        let top_right = Point2D {
            x: a.x.max(b.x),
            y: a.y.min(b.y),
        };
        let bottom_left = Point2D {
            x: a.x.min(b.x),
            y: a.y.max(b.y),
        };
        let bottom_right = Point2D {
            x: a.x.max(b.x),
            y: a.y.max(b.y),
        };
        self.is_edge_bound_by_any_line(edges, (top_left, bottom_left), Direction::Left)
            && self.is_edge_bound_by_any_line(edges, (bottom_left, bottom_right), Direction::Down)
            && self.is_edge_bound_by_any_line(edges, (bottom_right, top_right), Direction::Right)
            && self.is_edge_bound_by_any_line(edges, (top_right, top_left), Direction::Up)
    }

    fn get_ranges_for_direction(
        &self,
        edges: &[(Point2D, Point2D)],
        edge: (Point2D, Point2D),
        direction: Direction,
    ) -> Vec<RangeInclusive<isize>> {
        edges
            .iter()
            .filter(|(a, b)| match direction {
                Direction::Up => a.y <= edge.0.y && b.y <= edge.0.y,
                Direction::Down => a.y >= edge.0.y && b.y >= edge.0.y,
                Direction::Left => a.x <= edge.0.x && b.x <= edge.0.x,
                Direction::Right => a.x >= edge.0.x && b.x >= edge.0.x,
            })
            .map(|(a, b)| match direction {
                Direction::Up | Direction::Down => (a.x.min(b.x))..=a.x.max(b.x),
                Direction::Left | Direction::Right => (a.y.min(b.y))..=a.y.max(b.y),
            })
            .collect()
    }

    fn is_edge_bound_by_any_line(
        &self,
        lines: &[(Point2D, Point2D)],
        edge: (Point2D, Point2D),
        direction: Direction,
    ) -> bool {
        let x_range = edge.0.x.min(edge.1.x)..=edge.0.x.max(edge.1.x);
        let y_range = edge.0.y.min(edge.1.y)..=edge.0.y.max(edge.1.y);
        let range_to_use = match direction {
            Direction::Up | Direction::Down => x_range,
            Direction::Left | Direction::Right => y_range,
        };
        let ranges = self.get_ranges_for_direction(lines, edge, direction);
        self.is_range_covered_by_ranges(range_to_use.clone(), &ranges)
    }

    fn is_range_covered_by_ranges(
        &self,
        range: RangeInclusive<isize>,
        covers: &[RangeInclusive<isize>],
    ) -> bool {
        covers
            .iter()
            .fold(vec![range], |acc, x| {
                acc.iter()
                    .flat_map(|range| self.subtract_range(range, x))
                    .collect()
            })
            .is_empty()
    }

    fn subtract_range(
        &self,
        range: &RangeInclusive<isize>,
        to_subtract: &RangeInclusive<isize>,
    ) -> Vec<RangeInclusive<isize>> {
        if range.contains(to_subtract.start()) && range.contains(to_subtract.end()) {
            vec![
                *range.start()..=(to_subtract.start() - 1),
                (to_subtract.end() + 1)..=*range.end(),
            ]
        } else if range.contains(to_subtract.start()) {
            vec![*range.start()..=(to_subtract.start() - 1)]
        } else if range.contains(to_subtract.end()) {
            vec![(to_subtract.end() + 1)..=*range.end()]
        } else if to_subtract.contains(range.start()) && to_subtract.contains(range.end()) {
            vec![]
        } else {
            vec![range.clone()]
        }
        .into_iter()
        .filter(|r| r.end() - r.start() >= 0)
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day9 = Day9::new("data/test_data.txt").unwrap();
        let expected = 50;
        let actual = day9.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day9 = Day9::new("data/test_data.txt").unwrap();
        let expected = 24;
        let actual = day9.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day9 = Day9::new("data/input_data.txt").unwrap();
        let expected = 4763932976;
        let actual = day9.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day9 = Day9::new("data/input_data.txt").unwrap();
        let expected = 1501292304;
        let actual = day9.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
