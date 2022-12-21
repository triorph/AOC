mod parser;
use std::collections::HashMap;

use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};

use self::types::Operation;
mod types;

#[derive(Clone)]
pub struct Day21 {
    monkeys: HashMap<String, Operation>,
}

impl AOCCalculator for Day21 {
    fn new(filename: &str) -> Result<Day21, AOCFileOrParseError> {
        Ok(Day21 {
            monkeys: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        let mut obj = self.clone();
        println!("{}a answer is {:?}", name, obj.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day21 {
    fn calculate_day_a(&mut self) -> usize {
        self.get_value_at("root") as usize
    }

    fn get_value_at(&mut self, name: &str) -> isize {
        let monkey = self.monkeys.get(name).unwrap().clone();
        let ret = match monkey {
            Operation::Literal(val) => val as isize,
            Operation::Add(left, right) => self.get_value_at(&left) + self.get_value_at(&right),
            Operation::Subtract(left, right) => {
                self.get_value_at(&left) - self.get_value_at(&right)
            }
            Operation::Multiply(left, right) => {
                self.get_value_at(&left) * self.get_value_at(&right)
            }
            Operation::Divide(left, right) => self.get_value_at(&left) / self.get_value_at(&right),
        };
        self.monkeys
            .insert(name.to_string(), Operation::Literal(ret));
        ret
    }

    fn get_diff_at(&self, human_value: isize) -> isize {
        let mut obj = self.clone();
        let root_monkey = self.monkeys.get("root").unwrap();
        let (left, right) = match root_monkey {
            Operation::Add(left, right) => (left, right),
            Operation::Subtract(left, right) => (left, right),
            Operation::Divide(left, right) => (left, right),
            Operation::Multiply(left, right) => (left, right),
            _ => panic!("Should be +-*/"),
        };
        obj.monkeys
            .insert("humn".to_string(), Operation::Literal(human_value));
        obj.monkeys.insert(
            "root".to_string(),
            Operation::Subtract(left.to_string(), right.to_string()),
        );
        obj.get_value_at("root")
    }

    fn calculate_day_b(&self) -> usize {
        let (mut prev, mut next) = (1000, 10000);
        let mut prev_val = self.get_diff_at(prev);
        let mut next_val = self.get_diff_at(next);
        while next_val != 0 {
            let val_diff = next_val - prev_val;
            let input_diff = next - prev;
            prev = next;
            prev_val = next_val;
            let mut slope = prev_val * input_diff / val_diff;
            if slope == 0 {
                slope = 1
            }
            next = prev - slope;
            next_val = self.get_diff_at(next);
        }
        // there can be more than 1 correct answer, so get the smallest
        ((next - 5)..(next + 5))
            .filter(|i| self.get_diff_at(*i) == 0)
            .min()
            .unwrap() as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let mut day21 = Day21::new("data/test_data.txt").unwrap();
        let expected = 152;
        let actual = day21.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day21 = Day21::new("data/test_data.txt").unwrap();
        let expected = 301;
        let actual = day21.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let mut day21 = Day21::new("data/input_data.txt").unwrap();
        let expected = 56490240862410;
        let actual = day21.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_day_b_stuff() {
        let day21 = Day21::new("data/test_data.txt").unwrap();
        assert_eq!(day21.get_diff_at(301), 0);
        assert_eq!(day21.get_diff_at(302), 0);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day21 = Day21::new("data/input_data.txt").unwrap();
        let expected = 3403989691757;
        let actual = day21.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
