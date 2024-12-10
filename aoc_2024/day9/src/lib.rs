mod parser;
use std::collections::HashMap;

use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day9 {
    data: Vec<usize>,
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
    fn calculate_checksum(&self) -> usize {
        0
    }

    fn build_sector_map(&self) -> (Vec<(usize, usize, usize)>, Vec<(usize, usize)>) {
        self.data
            .chunks(2)
            .enumerate()
            .fold(
                (0, (vec![], vec![])),
                |(counter, (mut used_sectors, mut free_sectors)), (i, x)| {
                    used_sectors.push((i, counter, (counter + x[0])));
                    free_sectors.push((counter + x[0], counter + x[0] + x[1]));
                    (counter + x[0] + x[1], (used_sectors, free_sectors))
                },
            )
            .1
    }

    fn get_largest_value(&self, used_sectors: &[(usize, usize, usize)]) -> usize {
        used_sectors
            .iter()
            .filter(|(_id, min, max)| min < max)
            .map(|(_id, _min, max)| *max)
            .max()
            .unwrap_or(0)
    }

    fn get_smallest_empty(&self, free_sectors: &[(usize, usize)]) -> Option<(usize, usize)> {
        free_sectors
            .iter()
            .filter(|(min, max)| min < max)
            .map(|(min, max)| (*min, *max))
            .min_by(|(min_a, _), (min_b, _)| match (min_a, min_b) {
                (min_a, min_b) if min_a < min_b => std::cmp::Ordering::Less,
                (min_a, min_b) if min_b < min_a => std::cmp::Ordering::Greater,
                _ => std::cmp::Ordering::Equal,
            })
    }

    fn more_to_do(
        &self,
        used_sectors: &[(usize, usize, usize)],
        free_sectors: &[(usize, usize)],
    ) -> bool {
        let smallest = self.get_smallest_empty(&free_sectors);
        smallest.is_some() && self.get_largest_value(&used_sectors) > smallest.unwrap().0
    }

    fn calculate_day_a(&self) -> usize {
        let (mut used_sectors, mut free_sectors) = self.build_sector_map();
        while self.more_to_do(&used_sectors, &free_sectors) {}
        self.calculate_checksum()
    }

    fn calculate_day_b(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day9 = Day9::new("data/test_data.txt").unwrap();
        let expected = 1928;
        let actual = day9.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day9 = Day9::new("data/test_data.txt").unwrap();
        let expected = 0;
        let actual = day9.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day9 = Day9::new("data/input_data.txt").unwrap();
        let expected = 0;
        let actual = day9.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day9 = Day9::new("data/input_data.txt").unwrap();
        let expected = 0;
        let actual = day9.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
