mod assignments;
mod parser;
use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};
use assignments::{Assignment, Overlaps};

pub struct Day4 {
    assignments: Vec<Assignment>,
}

impl AOCCalculator<usize> for Day4 {
    fn new(filename: &str) -> Result<Day4, AOCFileOrParseError> {
        Ok(Day4 {
            assignments: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn calculate_day_a(&self) -> usize {
        self.assignments
            .iter()
            .filter(|assignment| (*assignment).complete_overlap())
            .count()
    }

    fn calculate_day_b(&self) -> usize {
        self.assignments
            .iter()
            .filter(|assignment| (*assignment).partial_overlap())
            .count()
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
        let skeleton = Day4::new("data/test_data.txt").unwrap();
        let expected = 2;
        let actual = skeleton.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let skeleton = Day4::new("data/test_data.txt").unwrap();
        let expected = 4;
        let actual = skeleton.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
