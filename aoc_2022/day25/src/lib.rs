mod parser;
mod snafu;
use crate::parser::parse_data;
use crate::snafu::Snafu;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};

pub struct Day25 {
    snafu_numbers: Vec<Vec<isize>>,
}

impl AOCCalculator for Day25 {
    fn new(filename: &str) -> Result<Day25, AOCFileOrParseError> {
        Ok(Day25 {
            snafu_numbers: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!(
            "{}a answer is {:?}",
            name,
            self.calculate_day_a().to_string()
        );
    }
}

impl Day25 {
    fn calculate_day_a(&self) -> Vec<isize> {
        Vec::convert_from_isize(
            self.snafu_numbers
                .iter()
                .map(|number| number.convert_to_isize())
                .sum(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day25 = Day25::new("data/test_data.txt").unwrap();
        let expected = "2=-1=0";
        let actual = day25.calculate_day_a().to_string();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day25 = Day25::new("data/input_data.txt").unwrap();
        let expected = "2-=2-0=-0-=0200=--21";
        let actual = day25.calculate_day_a().to_string();
        assert_eq!(expected, actual);
    }
}
