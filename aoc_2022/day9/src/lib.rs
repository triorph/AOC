mod parser;
mod types;
use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};
use std::collections::HashSet;
use types::Point;

pub struct Day9 {
    data: Vec<Point>,
}

impl AOCCalculator<usize> for Day9 {
    fn new(filename: &str) -> Result<Day9, AOCFileOrParseError> {
        Ok(Day9 {
            data: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn calculate_day_a(&self) -> usize {
        let mut t_vals: HashSet<Point> = HashSet::new();
        let mut head = Point(0, 0);
        let mut tails = Point(0, 0);
        for dir in self.data.iter() {
            for one_step in dir.get_steps().iter() {
                head = &head + one_step;
                tails = tails.follow_other(&head, 1);
                t_vals.insert(tails.clone());
            }
        }
        println!("t_vals: {:?}", t_vals);
        t_vals.len()
    }

    fn calculate_day_b(&self) -> usize {
        0
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day9 = Day9::new("data/test_data.txt").unwrap();
        let expected = 13;
        let actual = day9.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day9 = Day9::new("data/test_data.txt").unwrap();
        let expected = 0;
        let actual = day9.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
