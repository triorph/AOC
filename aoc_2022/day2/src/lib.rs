mod hand;
mod parser;
use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};
use hand::Hand;

pub struct Day2 {
    hand_pairs: Vec<(Hand, Hand)>,
}

impl AOCCalculator<usize> for Day2 {
    fn new(filename: &str) -> Result<Day2, AOCFileOrParseError> {
        Ok(Day2 {
            hand_pairs: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn calculate_day_a(&self) -> usize {
        self.hand_pairs
            .iter()
            .map(|(them, us)| us.calculate_score_vs_day_a(them))
            .sum()
    }

    fn calculate_day_b(&self) -> usize {
        self.hand_pairs
            .iter()
            .map(|(them, us)| us.calculate_score_vs_day_b(them))
            .sum()
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_day_a() {
        let day2 = Day2::new("data/test_data.txt").unwrap();
        let expected = 15;
        let actual = day2.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day2 = Day2::new("data/test_data.txt").unwrap();
        let expected = 12;
        let actual = day2.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
