mod parser;
use crate::parser::{parse_data_a, parse_data_b};
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};

pub struct Day1 {
    input_data: String,
}

impl AOCCalculator for Day1 {
    fn new(filename: &str) -> Result<Day1, AOCFileOrParseError> {
        Ok(Day1 {
            input_data: read_input_file(filename)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day1 {
    fn calculate_day_a(&self) -> usize {
        parse_data_a(&self.input_data)
            .map(|x| {
                x.iter()
                    .map(|line| line[0] * 10 + line[line.len() - 1])
                    .sum()
            })
            .unwrap_or(0)
    }

    fn calculate_day_b(&self) -> usize {
        parse_data_b(&self.input_data)
            .map(|x| {
                x.iter()
                    .map(|line| line[0] * 10 + line[line.len() - 1])
                    .sum()
            })
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day1 = Day1::new("data/test_data_a.txt").unwrap();
        let expected = 142;
        let actual = day1.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day1 = Day1::new("data/test_data_b.txt").unwrap();
        let expected = 281;
        let actual = day1.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day1 = Day1::new("data/input_data.txt").unwrap();
        let expected = 53386;
        let actual = day1.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day1 = Day1::new("data/input_data.txt").unwrap();
        let expected = 53312;
        let actual = day1.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
