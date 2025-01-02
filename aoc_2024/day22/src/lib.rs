mod parser;
use std::collections::HashMap;

use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day22 {
    data: Vec<usize>,
}

impl AOCCalculator for Day22 {
    fn new(filename: &str) -> Result<Day22, AOCFileOrParseError> {
        Ok(Day22 {
            data: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day22 {
    fn mix_number(&self, a: usize, b: usize) -> usize {
        a ^ b
    }

    fn prune_number(&self, n: usize) -> usize {
        n % 16777216
    }

    fn mix_and_prune(&self, a: usize, b: usize) -> usize {
        self.prune_number(self.mix_number(a, b))
    }

    fn get_next_item(&self, n: usize) -> usize {
        let n = self.mix_and_prune(n, n * 64);
        let n = self.mix_and_prune(n, n / 32);
        self.mix_and_prune(n, n * 2048)
    }

    fn iterate_number_n_times(&self, n: usize, times: usize) -> usize {
        (0..times).fold(n, |seq, _| self.get_next_item(seq))
    }

    fn calculate_day_a(&self) -> usize {
        self.data
            .iter()
            .map(|n| self.iterate_number_n_times(*n, 2000))
            .sum()
    }

    fn get_sequences_for_starting_n(&self, starting_n: usize) -> HashMap<[isize; 4], usize> {
        let mut quantities = HashMap::new();
        let mut seq_diff = [starting_n as isize % 10, 0, 0, 0];
        let mut n = starting_n;
        for i in 0..=2000 {
            let prev = n;
            seq_diff[0] = seq_diff[1];
            seq_diff[1] = seq_diff[2];
            seq_diff[2] = seq_diff[3];
            n = self.get_next_item(n);
            seq_diff[3] = (n as isize % 10) - prev as isize % 10;
            if i >= 3 && !quantities.contains_key(&seq_diff) {
                quantities.insert(seq_diff, n % 10);
            }
        }
        quantities
    }

    fn calculate_day_b(&self) -> usize {
        let mut quantities: HashMap<[isize; 4], usize> = HashMap::new();
        for starting_n in self.data.iter() {
            for (k, v) in self.get_sequences_for_starting_n(*starting_n).into_iter() {
                *quantities.entry(k).or_default() += v;
            }
        }
        println!("{:?}", quantities.get(&[-3, 0, 4, 0]));
        *quantities.values().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day22 = Day22::new("data/test_data.txt").unwrap();
        let expected = 37327623;
        let actual = day22.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sequences_for_starting_number() {
        let day22 = Day22::new("data/test_data.txt").unwrap();
        let expected = 5;
        let actual = *day22
            .get_sequences_for_starting_n(3338402)
            .get(&[-3, 0, 4, 0])
            .unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day22 = Day22::new("data/test_data_2.txt").unwrap();
        let expected = 23;
        let actual = day22.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day22 = Day22::new("data/input_data.txt").unwrap();
        let expected = 13753970725;
        let actual = day22.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day22 = Day22::new("data/input_data.txt").unwrap();
        let expected = 1570;
        let actual = day22.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
