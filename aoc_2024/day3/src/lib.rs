mod parser;
use crate::parser::parse_data;
use crate::parser::Instruction;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day3 {
    data: Vec<Instruction>,
}

impl AOCCalculator for Day3 {
    fn new(filename: &str) -> Result<Day3, AOCFileOrParseError> {
        Ok(Day3 {
            data: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day3 {
    fn get_instruction_value(instruction: &Instruction) -> usize {
        match instruction {
            Instruction::Multiply(a, b) => a * b,
            _ => 0,
        }
    }

    fn calculate_day_a(&self) -> usize {
        self.data
            .iter()
            .map(Day3::get_instruction_value)
            .reduce(|acc, x| acc + x)
            .unwrap_or(0)
    }

    fn calculate_day_b(&self) -> usize {
        self.data
            .iter()
            .fold(
                (1, 0),
                |(multiplier, accumulator), instruction| match instruction {
                    Instruction::Do => (1, accumulator),
                    Instruction::DoNot => (0, accumulator),
                    Instruction::Multiply(a, b) => (multiplier, accumulator + multiplier * a * b),
                },
            )
            .1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day3 = Day3::new("data/test_data.txt").unwrap();
        let expected = 161;
        let actual = day3.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day3 = Day3::new("data/test_data.txt").unwrap();
        let expected = 48;
        let actual = day3.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day3 = Day3::new("data/input_data.txt").unwrap();
        let expected = 155955228;
        let actual = day3.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day3 = Day3::new("data/input_data.txt").unwrap();
        let expected = 100189366;
        let actual = day3.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
