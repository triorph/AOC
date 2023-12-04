mod parser;
use std::cmp::min;
use std::collections::HashSet;

use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};

pub struct Day4 {
    cards: Vec<(HashSet<usize>, HashSet<usize>)>,
}

impl AOCCalculator for Day4 {
    fn new(filename: &str) -> Result<Day4, AOCFileOrParseError> {
        Ok(Day4 {
            cards: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day4 {
    fn calculate_day_a(&self) -> usize {
        self.cards
            .iter()
            .map(|(winners, candidates)| winners.intersection(candidates).count())
            .map(|x| {
                if x == 0 {
                    0
                } else {
                    2_i32.pow(x as u32 - 1) as usize
                }
            })
            .sum()
    }

    fn calculate_day_b(&self) -> usize {
        let mut card_quantities = vec![1; self.cards.len()];
        for i in (0..self.cards.len()).rev() {
            let matches = self.cards[i].0.intersection(&self.cards[i].1).count();
            println!("Card {:?} has matches {:?}", i, matches);
            for j in (i + 1)..min(i + matches + 1, self.cards.len()) {
                card_quantities[i] += card_quantities[j];
            }
        }
        println!("{:?}", card_quantities);
        card_quantities.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day4 = Day4::new("data/test_data.txt").unwrap();
        let expected = 13;
        let actual = day4.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day4 = Day4::new("data/test_data.txt").unwrap();
        let expected = 30;
        let actual = day4.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day4 = Day4::new("data/input_data.txt").unwrap();
        let expected = 21213;
        let actual = day4.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day4 = Day4::new("data/input_data.txt").unwrap();
        let expected = 8549735;
        let actual = day4.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
