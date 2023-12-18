mod parser;
use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day18 {
    data: Vec<usize>,
}

impl AOCCalculator for Day18 {
    fn new(filename: &str) -> Result<Day18, AOCFileOrParseError> {
        Ok(Day18 {
            data: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day18 {
    fn calculate_day_a(&self) -> usize {
        0
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
        let day18 = Day18::new("data/test_data.txt").unwrap();
        let expected = 0;
        let actual = day18.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day18 = Day18::new("data/test_data.txt").unwrap();
        let expected = 0;
        let actual = day18.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day18 = Day18::new("data/input_data.txt").unwrap();
        let expected = 0;
        let actual = day18.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day18 = Day18::new("data/input_data.txt").unwrap();
        let expected = 0;
        let actual = day18.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
