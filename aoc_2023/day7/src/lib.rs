mod parser;
mod poker_hand;
use crate::parser::parse_data;
use crate::poker_hand::PokerHand;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};

pub struct Day6 {
    bets: Vec<(PokerHand, usize)>,
}

impl AOCCalculator for Day6 {
    fn new(filename: &str) -> Result<Day6, AOCFileOrParseError> {
        Ok(Day6 {
            bets: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day6 {
    fn calculate_day_a(&self) -> usize {
        let mut bets = self.bets.clone();
        bets.sort_by(|first, second| first.0.compare_hand_day_a(&second.0));
        return bets
            .iter()
            .enumerate()
            .map(|(index, (_, bet))| (index + 1) * bet)
            .sum();
    }

    fn calculate_day_b(&self) -> usize {
        let mut bets = self.bets.clone();
        bets.sort_by(|first, second| first.0.compare_hand_day_b(&second.0));
        return bets
            .iter()
            .enumerate()
            .map(|(index, (_, bet))| (index + 1) * bet)
            .sum();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day6 = Day6::new("data/test_data.txt").unwrap();
        let expected = 6440;
        let actual = day6.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day6 = Day6::new("data/test_data.txt").unwrap();
        let expected = 5905;
        let actual = day6.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day6 = Day6::new("data/input_data.txt").unwrap();
        let expected = 255048101;
        let actual = day6.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day6 = Day6::new("data/input_data.txt").unwrap();
        let expected = 0;
        let actual = day6.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
