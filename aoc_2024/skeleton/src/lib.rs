mod parser;
use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Skeleton {
    data: Vec<usize>,
}

impl AOCCalculator for Skeleton {
    fn new(filename: &str) -> Result<Skeleton, AOCFileOrParseError> {
        Ok(Skeleton {
            data: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Skeleton {
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
        let skeleton = Skeleton::new("data/test_data.txt").unwrap();
        let expected = 0;
        let actual = skeleton.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let skeleton = Skeleton::new("data/test_data.txt").unwrap();
        let expected = 0;
        let actual = skeleton.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let skeleton = Skeleton::new("data/input_data.txt").unwrap();
        let expected = 0;
        let actual = skeleton.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let skeleton = Skeleton::new("data/input_data.txt").unwrap();
        let expected = 0;
        let actual = skeleton.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
