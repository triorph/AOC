mod hot_spring;
mod parser;

use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};
use hot_spring::ConditionReport;

pub struct Day12 {
    lines: Vec<ConditionReport>,
}

impl AOCCalculator for Day12 {
    fn new(filename: &str) -> Result<Day12, AOCFileOrParseError> {
        Ok(Day12 {
            lines: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day12 {
    fn calculate_day_a(&self) -> usize {
        self.lines
            .iter()
            .map(|condition_report| condition_report.solve_day_a())
            .sum()
    }

    fn calculate_day_b(&self) -> usize {
        self.lines
            .iter()
            .map(|condition_report| condition_report.solve_day_b())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day12 = Day12::new("data/test_data.txt").unwrap();
        let expected = 21;
        let actual = day12.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day12 = Day12::new("data/test_data.txt").unwrap();
        let expected = 525152;
        let actual = day12.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day12 = Day12::new("data/input_data.txt").unwrap();
        let expected = 7084;
        let actual = day12.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day12 = Day12::new("data/input_data.txt").unwrap();
        let expected = 8414003326821;
        let actual = day12.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
