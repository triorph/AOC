mod parser;
use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};

pub struct Day9 {
    sequences: Vec<Vec<isize>>,
}

impl AOCCalculator for Day9 {
    fn new(filename: &str) -> Result<Day9, AOCFileOrParseError> {
        Ok(Day9 {
            sequences: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day9 {
    fn get_next_derivative_of_sequence(sequence: &[isize]) -> Vec<isize> {
        sequence
            .windows(2)
            .map(|x| x[1] - x[0])
            .collect::<Vec<isize>>()
    }

    fn calculate_next_in_sequence(sequence: &[isize]) -> isize {
        if sequence.iter().all(|x| *x == 0) {
            0
        } else {
            sequence[sequence.len() - 1]
                + Day9::calculate_next_in_sequence(&Day9::get_next_derivative_of_sequence(sequence))
        }
    }

    fn calculate_previous_in_sequence(sequence: &[isize]) -> isize {
        if sequence.iter().all(|x| *x == 0) {
            0
        } else {
            sequence[0]
                - Day9::calculate_previous_in_sequence(&Day9::get_next_derivative_of_sequence(
                    sequence,
                ))
        }
    }

    fn calculate_day_a(&self) -> isize {
        self.sequences
            .iter()
            .map(|sequence| Day9::calculate_next_in_sequence(sequence))
            .sum()
    }

    fn calculate_day_b(&self) -> isize {
        self.sequences
            .iter()
            .map(|sequence| Day9::calculate_previous_in_sequence(sequence))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day9 = Day9::new("data/test_data.txt").unwrap();
        let expected = 114;
        let actual = day9.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day9 = Day9::new("data/test_data.txt").unwrap();
        let expected = 2;
        let actual = day9.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day9 = Day9::new("data/input_data.txt").unwrap();
        let expected = 1972648895;
        let actual = day9.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day9 = Day9::new("data/input_data.txt").unwrap();
        let expected = 919;
        let actual = day9.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
