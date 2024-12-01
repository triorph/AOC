mod parser;
use std::collections::HashMap;

use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day01 {
    data: Vec<(usize, usize)>,
}

impl AOCCalculator for Day01 {
    fn new(filename: &str) -> Result<Day01, AOCFileOrParseError> {
        Ok(Day01 {
            data: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day01 {
    fn calculate_day_a(&self) -> usize {
        let mut left: Vec<usize> = self.data.iter().map(|(l, _)| l).copied().collect();
        left.sort();
        let mut right: Vec<usize> = self.data.iter().map(|(_, r)| r).copied().collect();
        right.sort();
        left.into_iter()
            .zip(right)
            .map(|(l, r)| l.abs_diff(r))
            .sum()
    }

    fn calculate_day_b(&self) -> usize {
        let left = self.data.iter().map(|(l, _)| l);
        let right = self.data.iter().map(|(_, r)| r);
        let right_frequency = right.fold(HashMap::new(), |mut acc: HashMap<usize, usize>, e| {
            *acc.entry(*e).or_insert(0) += 1;
            acc
        });
        left.map(|l| l * right_frequency.get(l).unwrap_or(&0)).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day01 = Day01::new("data/test_data.txt").unwrap();
        let expected = 11;
        let actual = day01.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day01 = Day01::new("data/test_data.txt").unwrap();
        let expected = 31;
        let actual = day01.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day01 = Day01::new("data/input_data.txt").unwrap();
        let expected = 2166959;
        let actual = day01.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day01 = Day01::new("data/input_data.txt").unwrap();
        let expected = 23741109;
        let actual = day01.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
