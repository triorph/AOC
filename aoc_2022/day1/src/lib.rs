mod parser;
use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};

pub struct Day1 {
    calories: Vec<usize>,
}

impl AOCCalculator<usize> for Day1 {
    fn new(filename: &str) -> Result<Day1, AOCFileOrParseError> {
        Ok(Day1 {
            calories: parse_data(&read_input_file(filename)?)?
                .iter()
                .map(|v| v.iter().sum())
                .collect(),
        })
    }

    fn calculate_day_a(&self) -> usize {
        self.calories
            .iter()
            .fold(0, |a, b| if a > *b { a } else { *b })
    }

    fn calculate_day_b(&self) -> usize {
        let mut calories = self.calories.clone();
        calories.sort();
        calories[(calories.len() - 3)..calories.len()].iter().sum()
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
        let day1 = Day1::new("data/test_data.txt").unwrap();
        let expected = 24000;
        let actual = day1.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day1 = Day1::new("data/test_data.txt").unwrap();
        let expected = 45000;
        let actual = day1.calculate_day_b();
        assert_eq!(expected, actual);
    }
}