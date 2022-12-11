mod parser;
use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};

pub struct Day12 {
    data: (),
}

impl AOCCalculator for Day12 {
    fn new(filename: &str) -> Result<Day12, AOCFileOrParseError> {
        Ok(Day12 {
            data: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day12 {
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
        let day12 = Day12::new("data/test_data.txt").unwrap();
        let expected = 0;
        let actual = day12.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day12 = Day12::new("data/test_data.txt").unwrap();
        let expected = 0;
        let actual = day12.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
