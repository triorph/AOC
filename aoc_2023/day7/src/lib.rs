mod parser;
mod poker_hand;
use crate::parser::parse_data;
use crate::poker_hand::PokerHand;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};

pub struct Day7 {
    bets: Vec<(PokerHand, usize)>,
}

impl AOCCalculator for Day7 {
    fn new(filename: &str) -> Result<Day7, AOCFileOrParseError> {
        Ok(Day7 {
            bets: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day7 {
    fn calculate_day_a(&self) -> usize {
        let mut bets = self.bets.clone();
        bets.sort_by(|first, second| first.0.compare_day_a(&second.0));
        return bets
            .iter()
            .enumerate()
            .map(|(index, (_, bet))| (index + 1) * bet)
            .sum();
    }

    fn calculate_day_b(&self) -> usize {
        let mut bets = self.bets.clone();
        bets.sort_by(|first, second| first.0.compare_day_b(&second.0));
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
        let day7 = Day7::new("data/test_data.txt").unwrap();
        let expected = 6440;
        let actual = day7.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day7 = Day7::new("data/test_data.txt").unwrap();
        let expected = 5905;
        let actual = day7.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day7 = Day7::new("data/input_data.txt").unwrap();
        let expected = 255048101;
        let actual = day7.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day7 = Day7::new("data/input_data.txt").unwrap();
        let expected = 253718286;
        let actual = day7.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
