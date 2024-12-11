mod parser;
use std::collections::HashMap;

use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day11 {
    data: Vec<u64>,
}

impl AOCCalculator for Day11 {
    fn new(filename: &str) -> Result<Day11, AOCFileOrParseError> {
        Ok(Day11 {
            data: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day11 {
    fn iterate_stone_counts(&self, stone_counts: &HashMap<u64, u64>) -> HashMap<u64, u64> {
        let mut ret = HashMap::new();
        for (key, value) in stone_counts.iter() {
            if *key == 0 {
                *ret.entry(1).or_insert(0) += *value;
            } else {
                let key_str = (*key).to_string();
                if key_str.len() % 2 == 0 {
                    let left: u64 = str::parse(&key_str[0..(key_str.len() / 2)]).unwrap();
                    let right: u64 = str::parse(&key_str[(key_str.len() / 2)..]).unwrap();
                    *ret.entry(left).or_insert(0) += *value;
                    *ret.entry(right).or_insert(0) += *value;
                } else {
                    *ret.entry(key * 2024).or_insert(0) += *value;
                }
            }
        }
        ret
    }
    fn calculate_day_a(&self) -> u64 {
        self.iterate_n_times(25)
    }

    fn iterate_n_times(&self, n: u64) -> u64 {
        let mut stone_counts: HashMap<u64, u64> = self.data.iter().map(|x| (*x, 1)).collect();
        for _ in 0..n {
            stone_counts = self.iterate_stone_counts(&stone_counts);
        }

        stone_counts.into_values().sum()
    }

    fn calculate_day_b(&self) -> u64 {
        self.iterate_n_times(75)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day11 = Day11::new("data/test_data.txt").unwrap();
        let expected = 55312;
        let actual = day11.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day11 = Day11::new("data/test_data.txt").unwrap();
        let expected = 65601038650482;
        let actual = day11.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day11 = Day11::new("data/input_data.txt").unwrap();
        let expected = 211306;
        let actual = day11.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day11 = Day11::new("data/input_data.txt").unwrap();
        let expected = 250783680217283;
        let actual = day11.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_extreme() {
        let day11 = Day11::new("data/input_data.txt").unwrap();
        let expected = 9825332028805901806;
        let actual = day11.iterate_n_times(9999);
        assert_eq!(expected, actual);
    }
}
