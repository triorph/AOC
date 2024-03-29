mod parser;
mod priority;

use crate::parser::parse_data;
use aoc_helpers::hash_utils::FromVec;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};
use priority::HasPriority;

use std::collections::HashSet;

pub struct Day3 {
    rucksack_lines: Vec<Vec<char>>,
}

impl AOCCalculator for Day3 {
    fn new(filename: &str) -> Result<Day3, AOCFileOrParseError> {
        Ok(Day3 {
            rucksack_lines: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day3 {
    fn calculate_day_a(&self) -> usize {
        self.get_rucksack_pairs_day_a()
            .get_priority()
            .try_into()
            .expect("should convert to usize correctly")
    }

    fn calculate_day_b(&self) -> usize {
        self.get_rucksack_groups_day_b()
            .get_priority()
            .try_into()
            .expect("should convert to usize correctly")
    }

    fn get_rucksack_pairs_day_a(&self) -> Vec<Vec<HashSet<char>>> {
        self.rucksack_lines
            .iter()
            .map(|line| {
                vec![
                    HashSet::from_vec(&line[0..(line.len() / 2)]),
                    HashSet::from_vec(&line[(line.len() / 2)..]),
                ]
            })
            .collect()
    }

    fn get_rucksack_groups_day_b(&self) -> Vec<Vec<HashSet<char>>> {
        let mut ret = Vec::new();
        for i in 0..(self.rucksack_lines.len() / 3) {
            let mut inner_ret = Vec::new();
            for j in 0..3 {
                let line = &self.rucksack_lines[3 * i + j];
                inner_ret.push(HashSet::from_vec(line))
            }
            ret.push(inner_ret);
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day3 = Day3::new("data/test_data.txt").unwrap();
        let expected = 157;
        let actual = day3.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day3 = Day3::new("data/test_data.txt").unwrap();
        let expected = 70;
        let actual = day3.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
