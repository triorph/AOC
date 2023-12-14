mod parser;
use std::cmp::min;

use crate::parser::parse_data;
use aoc_helpers::vec::Transposable;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};
use parser::Tile;

pub struct Day13 {
    locations: Vec<Vec<Vec<Tile>>>,
}

impl AOCCalculator for Day13 {
    fn new(filename: &str) -> Result<Day13, AOCFileOrParseError> {
        Ok(Day13 {
            locations: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day13 {
    fn find_error_count(&self, location: &[Vec<Tile>], mirror_column: usize) -> usize {
        (0..min(mirror_column + 1, location[0].len() - mirror_column - 1))
            .map(|column| {
                (0..location.len())
                    .filter(|y| {
                        location[*y][mirror_column - column]
                            != location[*y][mirror_column + column + 1]
                    })
                    .count()
            })
            .sum::<usize>()
    }

    fn find_column_of_mirror(&self, location: &[Vec<Tile>], smudge_count: usize) -> Option<usize> {
        for mirror_candidate in 0..(location[0].len() - 1) {
            if self.find_error_count(location, mirror_candidate) == smudge_count {
                return Some(mirror_candidate + 1);
            }
        }
        None
    }

    fn get_location_score(&self, location: &[Vec<Tile>], smudge_count: usize) -> usize {
        self.find_column_of_mirror(location, smudge_count)
            .unwrap_or_else(|| {
                self.find_column_of_mirror(&location.transpose(), smudge_count)
                    .expect("should exist")
                    * 100
            })
    }

    fn get_all_locations_score(&self, smudge_count: usize) -> usize {
        self.locations
            .iter()
            .map(|location| self.get_location_score(location, smudge_count))
            .sum()
    }

    fn calculate_day_a(&self) -> usize {
        self.get_all_locations_score(0)
    }

    fn calculate_day_b(&self) -> usize {
        self.get_all_locations_score(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day13 = Day13::new("data/test_data.txt").unwrap();
        let expected = 405;
        let actual = day13.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day13 = Day13::new("data/test_data.txt").unwrap();
        let expected = 400;
        let actual = day13.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day13 = Day13::new("data/input_data.txt").unwrap();
        let expected = 34100;
        let actual = day13.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day13 = Day13::new("data/input_data.txt").unwrap();
        let expected = 33106;
        let actual = day13.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
