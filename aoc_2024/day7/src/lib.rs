mod parser;
use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day7 {
    data: Vec<(isize, Vec<isize>)>,
}

impl AOCCalculator for Day7 {
    fn new(filename: &str) -> Result<Day7, AOCFileOrParseError> {
        Ok(Day7 {
            data: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day7 {
    fn is_line_true_a(target: isize, current: isize, remaining: &[isize]) -> bool {
        if current == target && remaining.is_empty() {
            true
        } else if current > target || remaining.is_empty() {
            false
        } else {
            Day7::is_line_true_a(
                target,
                current * remaining[0],
                &remaining[1..remaining.len()],
            ) || Day7::is_line_true_a(
                target,
                current + remaining[0],
                &remaining[1..remaining.len()],
            )
        }
    }
    fn calculate_day_a(&self) -> isize {
        self.data
            .iter()
            .filter(|(target, numbers)| {
                Day7::is_line_true_a(*target, numbers[0], &numbers[1..numbers.len()])
            })
            .map(|(target, _)| target)
            .sum()
    }

    fn concat(&self, a: isize, b: isize) -> isize {
        let concat_str = format!("{}{}", a, b);
        str::parse(&concat_str).expect("Numbers will concat to a parseable string")
    }

    fn is_line_true_b(&self, target: isize, current: isize, remaining: &[isize]) -> bool {
        if current == target && remaining.is_empty() {
            true
        } else if current > target || remaining.is_empty() {
            false
        } else {
            self.is_line_true_b(
                target,
                self.concat(current, remaining[0]),
                &remaining[1..remaining.len()],
            ) || self.is_line_true_b(
                target,
                current * remaining[0],
                &remaining[1..remaining.len()],
            ) || self.is_line_true_b(
                target,
                current + remaining[0],
                &remaining[1..remaining.len()],
            )
        }
    }

    fn calculate_day_b(&self) -> isize {
        self.data
            .iter()
            .filter(|(target, numbers)| {
                self.is_line_true_b(*target, numbers[0], &numbers[1..numbers.len()])
            })
            .map(|(target, _)| target)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day7 = Day7::new("data/test_data.txt").unwrap();
        let expected = 3749;
        let actual = day7.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day7 = Day7::new("data/test_data.txt").unwrap();
        let expected = 11387;
        let actual = day7.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day7 = Day7::new("data/input_data.txt").unwrap();
        let expected = 5540634308362;
        let actual = day7.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day7 = Day7::new("data/input_data.txt").unwrap();
        let expected = 472290821152397;
        let actual = day7.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
