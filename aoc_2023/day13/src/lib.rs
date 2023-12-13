mod parser;
use std::cmp::min;

use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};

pub struct Day13 {
    locations: Vec<Vec<Vec<bool>>>,
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
    fn assess_location_column(&self, location: &[Vec<bool>], smudge_count: usize) -> Option<usize> {
        for i in 0..(location[0].len() - 1) {
            let size = min(i + 1, location[0].len() - i - 1);
            if (0..size)
                .map(|s| {
                    let column_1 = i - s;
                    let column_2 = i + s + 1;
                    (0..location.len())
                        .filter(|y| location[*y][column_1] != location[*y][column_2])
                        .count()
                })
                .sum::<usize>()
                == smudge_count
            {
                return Some(i + 1);
            }
        }
        None
    }

    fn assess_location_row(&self, location: &[Vec<bool>], smudge_count: usize) -> Option<usize> {
        for i in 0..(location.len() - 1) {
            let size = min(i + 1, location.len() - i - 1);
            if (0..size)
                .map(|s| {
                    let row_1 = i - s;
                    let row_2 = i + s + 1;
                    (0..location[0].len())
                        .filter(|x| location[row_1][*x] != location[row_2][*x])
                        .count()
                })
                .sum::<usize>()
                == smudge_count
            {
                return Some((i + 1) * 100);
            }
        }
        None
    }

    fn get_location_score(&self, smudge_count: usize) -> usize {
        self.locations
            .iter()
            .map(|location| {
                self.assess_location_column(location, smudge_count)
                    .unwrap_or_else(|| {
                        self.assess_location_row(location, smudge_count)
                            .expect("should exist")
                    })
            })
            .sum()
    }

    fn calculate_day_a(&self) -> usize {
        self.get_location_score(0)
    }

    fn calculate_day_b(&self) -> usize {
        self.get_location_score(1)
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
