mod parser;
use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day2 {
    data: Vec<Vec<isize>>,
}

impl AOCCalculator for Day2 {
    fn new(filename: &str) -> Result<Day2, AOCFileOrParseError> {
        Ok(Day2 {
            data: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day2 {
    fn is_line_valid_day_a(line: &[isize]) -> bool {
        let mut differences = line.windows(2).map(|x| x[0] - x[1]);
        differences.clone().all(|x| (-3..=-1).contains(&x))
            || differences.all(|x| (1..=3).contains(&x))
    }

    fn is_line_valid_day_b(&self, line: &[isize]) -> bool {
        Day2::is_line_valid_day_a(line)
            || line.iter().combinations(line.len() - 1).any(|subline| {
                Day2::is_line_valid_day_a(&subline.into_iter().cloned().collect::<Vec<isize>>())
            })
    }

    fn calculate_day_a(&self) -> usize {
        self.data
            .iter()
            .filter(|line| Day2::is_line_valid_day_a(line))
            .count()
    }

    fn calculate_day_b(&self) -> usize {
        self.data
            .iter()
            .filter(|x| self.is_line_valid_day_b(x))
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day2 = Day2::new("data/test_data.txt").unwrap();
        let expected = 2;
        let actual = day2.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day2 = Day2::new("data/test_data.txt").unwrap();
        let expected = 4;
        let actual = day2.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day2 = Day2::new("data/input_data.txt").unwrap();
        let expected = 572;
        let actual = day2.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day2 = Day2::new("data/input_data.txt").unwrap();
        let expected = 612;
        let actual = day2.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
