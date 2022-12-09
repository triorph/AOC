mod parser;
use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};

pub struct Day10 {
    data: (),
}

impl AOCCalculator<usize> for Day10 {
    fn new(filename: &str) -> Result<Day10, AOCFileOrParseError> {
        Ok(Day10 {
            data: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn calculate_day_a(&self) -> usize {
        0
    }

    fn calculate_day_b(&self) -> usize {
        0
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day10 = Day10::new("data/test_data.txt").unwrap();
        let expected = 0;
        let actual = day10.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day10 = Day10::new("data/test_data.txt").unwrap();
        let expected = 0;
        let actual = day10.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
