mod operator;
mod parser;
use std::ops::Range;

use crate::operator::Operator;
use crate::parser::parse_data;
use aoc_helpers::{read_input_file, vec::Transposable, AOCCalculator, AOCFileOrParseError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day6 {
    numbers: Vec<String>,
    operators: Vec<(Operator, usize)>,
}

impl AOCCalculator for Day6 {
    fn new(filename: &str) -> Result<Day6, AOCFileOrParseError> {
        let (numbers, operators) = parse_data(&read_input_file(filename)?)?;
        Ok(Day6 { numbers, operators })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day6 {
    fn build_slice_indices(&self) -> Vec<Range<usize>> {
        self.operators
            .iter()
            .fold(
                (0, vec![]),
                |(start_index, mut slice_indices), (_, space_count)| {
                    slice_indices.push(start_index..(start_index + space_count + 1));
                    (start_index + space_count + 2, slice_indices)
                },
            )
            .1
    }

    fn slice_number_line(&self, number_line: &str) -> Vec<Vec<char>> {
        self.build_slice_indices()
            .into_iter()
            .map(|slice_index| number_line[slice_index].chars().collect())
            .collect()
    }

    fn slice_numbers_into_quadrants(&self) -> Vec<Vec<Vec<char>>> {
        self.numbers
            .iter()
            .map(|number_line| self.slice_number_line(number_line))
            .collect::<Vec<Vec<Vec<char>>>>()
            .transpose()
    }

    fn convert_to_number(&self, number_chars: &[char]) -> usize {
        number_chars
            .iter()
            .cloned()
            .collect::<String>()
            .trim()
            .parse::<usize>()
            .unwrap()
    }

    fn convert_quadrant_to_numbers(&self, number_quadrant: &[Vec<char>]) -> Vec<usize> {
        number_quadrant
            .iter()
            .map(|number_chars| self.convert_to_number(number_chars))
            .collect::<Vec<usize>>()
    }

    fn apply_operator(&self, numbers: &[usize], operator: &Operator) -> usize {
        numbers
            .iter()
            .copied()
            .reduce(|acc, x| operator.reduce(acc, x))
            .unwrap_or(0)
    }

    fn calculate_day_a(&self) -> usize {
        self.slice_numbers_into_quadrants()
            .iter()
            .zip(self.operators.iter())
            .map(|(number_quadrant, (operator, _))| {
                self.apply_operator(&self.convert_quadrant_to_numbers(number_quadrant), operator)
            })
            .sum()
    }

    fn calculate_day_b(&self) -> usize {
        self.slice_numbers_into_quadrants()
            .iter()
            .zip(self.operators.iter())
            .map(|(number_quadrant, (operator, _))| {
                self.apply_operator(
                    &self.convert_quadrant_to_numbers(&number_quadrant.transpose()),
                    operator,
                )
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day6 = Day6::new("data/test_data.txt").unwrap();
        let expected = 4277556;
        let actual = day6.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day6 = Day6::new("data/test_data.txt").unwrap();
        let expected = 3263827;
        let actual = day6.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day6 = Day6::new("data/input_data.txt").unwrap();
        let expected = 8108520669952;
        let actual = day6.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day6 = Day6::new("data/input_data.txt").unwrap();
        let expected = 11708563470209;
        let actual = day6.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
