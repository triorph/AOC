mod knot;
mod parser;
mod rope;
use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};
use knot::Knot;
use rope::Rope;
use std::collections::HashSet;

pub struct Day9 {
    data: Vec<Knot>,
}

impl AOCCalculator for Day9 {
    fn new(filename: &str) -> Result<Day9, AOCFileOrParseError> {
        Ok(Day9 {
            data: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day9 {
    fn calculate_day_a(&self) -> usize {
        self.calculate_x_followers(1)
    }

    fn calculate_day_b(&self) -> usize {
        self.calculate_x_followers(9)
    }

    fn calculate_x_followers(&self, x: usize) -> usize {
        let mut tail_locations: HashSet<Knot> = HashSet::new();
        let mut rope = Rope::new(x + 1);
        for instruction in self.data.iter() {
            tail_locations.extend(rope.follow_instruction(instruction).iter())
        }
        tail_locations.len()
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
        let expected = 1;
        let actual = day9.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b_2() {
        let day9 = Day9::new("data/test_data2.txt").unwrap();
        let expected = 36;
        let actual = day9.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
