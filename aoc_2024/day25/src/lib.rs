mod parser;
use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day25 {
    locks_or_keys: Vec<Vec<Vec<bool>>>,
}

impl AOCCalculator for Day25 {
    fn new(filename: &str) -> Result<Day25, AOCFileOrParseError> {
        Ok(Day25 {
            locks_or_keys: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
    }
}

impl Day25 {
    fn parse_lock(&self, lock: &[Vec<bool>]) -> Vec<usize> {
        (0..5)
            .map(|i| (0..5).filter(|&j| lock[j + 1][i]).count())
            .collect()
    }

    fn parse_locks(&self, locks: &[Vec<Vec<bool>>]) -> Vec<Vec<usize>> {
        locks.iter().map(|lock| self.parse_lock(lock)).collect()
    }

    fn parse_key(&self, key: &[Vec<bool>]) -> Vec<usize> {
        (0..5)
            .map(|i| (0..5).filter(|&j| key[j + 1][i]).count())
            .collect()
    }

    fn parse_keys(&self, keys: &[Vec<Vec<bool>>]) -> Vec<Vec<usize>> {
        keys.iter().map(|key| self.parse_key(key)).collect()
    }

    fn calculate_day_a(&self) -> usize {
        let locks = self.parse_locks(
            &self
                .locks_or_keys
                .clone()
                .into_iter()
                .filter(|x| x[0][0])
                .collect::<Vec<Vec<Vec<bool>>>>(),
        );
        let keys = self.parse_keys(
            &self
                .locks_or_keys
                .clone()
                .into_iter()
                .filter(|x| !x[0][0])
                .collect::<Vec<Vec<Vec<bool>>>>(),
        );
        locks
            .into_iter()
            .cartesian_product(keys)
            .filter(|(lock, key)| self.key_fits_lock(lock, key))
            .count()
    }

    fn key_fits_lock(&self, lock: &[usize], key: &[usize]) -> bool {
        key.iter().zip(lock.iter()).all(|(l, k)| l + k <= 5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day25 = Day25::new("data/test_data.txt").unwrap();
        let expected = 3;
        let actual = day25.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day25 = Day25::new("data/input_data.txt").unwrap();
        let expected = 0;
        let actual = day25.calculate_day_a();
        assert_eq!(expected, actual);
    }
}
