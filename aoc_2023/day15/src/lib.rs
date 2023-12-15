mod lens_box;
mod parser;
mod step;
use crate::lens_box::{LensBoxes, LensBoxesType};
use crate::parser::parse_data;
use crate::step::{AOCHash, Step};
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day15 {
    steps: Vec<Step>,
}

impl AOCCalculator for Day15 {
    fn new(filename: &str) -> Result<Day15, AOCFileOrParseError> {
        Ok(Day15 {
            steps: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day15 {
    fn calculate_day_a(&self) -> usize {
        self.steps
            .iter()
            .map(|step| step.to_day_a_string().get_aoc_hash())
            .sum()
    }

    fn calculate_day_b(&self) -> usize {
        LensBoxesType::new()
            .process_steps(&self.steps)
            .get_focusing_power()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day15 = Day15::new("data/test_data.txt").unwrap();
        let expected = 1320;
        let actual = day15.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day15 = Day15::new("data/test_data.txt").unwrap();
        let expected = 145;
        let actual = day15.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day15 = Day15::new("data/input_data.txt").unwrap();
        let expected = 513158;
        let actual = day15.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day15 = Day15::new("data/input_data.txt").unwrap();
        let expected = 200277;
        let actual = day15.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
