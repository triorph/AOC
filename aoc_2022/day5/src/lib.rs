mod parser;
use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};

pub struct Day5 {
    data: (),
}

impl AOCCalculator<usize> for Day5 {
    fn new(filename: &str) -> Result<Day5, AOCFileOrParseError> {
        Ok(Day5 {
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

    #[test]
    fn test_calculate_day_a() {
        let day5 = Day5::new("data/test_data.txt").unwrap();
        let expected = 0;
        let actual = day5.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day5 = Day5::new("data/test_data.txt").unwrap();
        let expected = 0;
        let actual = day5.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
