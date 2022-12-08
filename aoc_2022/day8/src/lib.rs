mod parser;
mod types;
use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};
use itertools::Itertools;
use types::Point;

pub struct Day8 {
    trees: Vec<Vec<u8>>,
}

impl AOCCalculator<usize> for Day8 {
    fn new(filename: &str) -> Result<Day8, AOCFileOrParseError> {
        Ok(Day8 {
            trees: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn calculate_day_a(&self) -> usize {
        self.get_all_points()
            .map(|point| self.is_point_viewable_from_edge(&point))
            .filter(|v| *v)
            .count()
    }

    fn calculate_day_b(&self) -> usize {
        self.get_all_points()
            .map(|point| self.get_beauty_score(&point))
            .max()
            .unwrap_or(0)
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day8 {
    fn get_slice_left<'a>(&'a self, point: &'a Point) -> Vec<u8> {
        (0..point.0)
            .map(|x| self.get_at_point(&(x, point.1)))
            .rev()
            .collect()
    }

    fn get_slice_right<'a>(&'a self, point: &'a Point) -> Vec<u8> {
        ((point.0 + 1)..self.trees[point.1].len())
            .map(|x| self.get_at_point(&(x, point.1)))
            .collect()
    }

    fn get_slice_above<'a>(&'a self, point: &'a Point) -> Vec<u8> {
        (0..point.1)
            .map(|y| self.get_at_point(&(point.0, y)))
            .rev()
            .collect()
    }

    fn get_slice_below<'a>(&'a self, point: &'a Point) -> Vec<u8> {
        ((point.1 + 1)..self.trees.len())
            .map(|y| self.get_at_point(&(point.0, y)))
            .collect()
    }

    fn get_all_slices<'a>(&'a self, point: &'a Point) -> [Vec<u8>; 4] {
        [
            self.get_slice_left(point),
            self.get_slice_right(point),
            self.get_slice_above(point),
            self.get_slice_below(point),
        ]
    }

    fn is_slice_viewable_from_edge(&self, point: &Point, slice: &[u8]) -> bool {
        slice
            .iter()
            .filter(|&&tree| tree >= self.get_at_point(point))
            .count()
            == 0
    }

    fn is_point_viewable_from_edge(&self, point: &Point) -> bool {
        self.get_all_slices(point)
            .iter()
            .filter(|slice| self.is_slice_viewable_from_edge(point, *slice))
            .count()
            > 0
    }

    fn trees_viewable_on_path(&self, point: &Point, slice: &[u8]) -> usize {
        let this_tree = self.get_at_point(point);
        let mut count = 1;
        for tree in slice.iter() {
            if tree >= &this_tree {
                return count;
            } else {
                count += 1;
            }
        }
        count - 1
    }

    fn get_beauty_score(&self, point: &Point) -> usize {
        self.get_all_slices(point)
            .iter()
            .map(|slice| self.trees_viewable_on_path(point, slice))
            .product::<usize>()
    }

    fn get_all_points(&self) -> Box<dyn Iterator<Item = Point>> {
        Box::new((0..self.trees.len()).cartesian_product(0..self.trees[0].len()))
    }

    fn get_at_point(&self, point: &Point) -> u8 {
        self.trees[point.1][point.0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day8 = Day8::new("data/test_data.txt").unwrap();
        let expected = 21;
        let actual = day8.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day8 = Day8::new("data/test_data.txt").unwrap();
        let expected = 8;
        let actual = day8.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_beauty_score() {
        let day8 = Day8::new("data/test_data.txt").unwrap();
        let expected = 8;
        let actual = day8.get_beauty_score(&(2, 3));
        assert_eq!(actual, expected);
    }
}
