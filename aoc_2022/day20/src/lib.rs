mod parser;
use std::fmt::Display;

use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};

#[derive(Clone)]
pub struct Day20 {
    numbers: Vec<isize>,
    next_index: Vec<usize>,
    prev_index: Vec<usize>,
}

impl AOCCalculator for Day20 {
    fn new(filename: &str) -> Result<Day20, AOCFileOrParseError> {
        let numbers = parse_data(&read_input_file(filename)?)?;
        let mut next_index = vec![0; numbers.len()];
        let mut prev_index = vec![0; numbers.len()];

        for i in 0..numbers.len() {
            next_index[i] = (i + 1) % numbers.len();
            prev_index[i] = (numbers.len() + i - 1) % numbers.len();
        }
        Ok(Day20 {
            numbers,
            next_index,
            prev_index,
        })
    }

    fn print_results(&self, name: &str) {
        let mut obj = self.clone();
        println!("{}a answer is {:?}", name, obj.calculate_day_a());
        let mut obj = self.clone();
        println!("{}b answer is {:?}", name, obj.calculate_day_b());
    }
}

impl Day20 {
    fn calculate_day_a(&mut self) -> isize {
        for i in 0..self.numbers.len() {
            self.move_at_index(i);
        }
        self.get_result()
    }

    fn get_result(&self) -> isize {
        let mut starting_index = 0;
        for i in 0..self.numbers.len() {
            if self.numbers[i] == 0 {
                starting_index = i;
                break;
            }
        }
        self.numbers[self.walk_forward_n(starting_index, 1000)]
            + self.numbers[self.walk_forward_n(starting_index, 2000)]
            + self.numbers[self.walk_forward_n(starting_index, 3000)]
    }

    fn walk_forward_n(&self, starting_index: usize, n: usize) -> usize {
        let mut ret = starting_index;
        for _ in 0..n {
            ret = self.next_index[ret];
        }
        ret
    }

    fn walk_backward_n(&self, starting_index: usize, n: usize) -> usize {
        let mut ret = starting_index;
        for _ in 0..n {
            ret = self.prev_index[ret];
        }
        ret
    }

    fn move_at_index(&mut self, index: usize) {
        let move_amount = self.numbers[index] % (self.numbers.len() as isize - 1);
        let current_prev = self.prev_index[index];
        let current_next = self.next_index[index];
        self.next_index[current_prev] = current_next;
        self.prev_index[current_next] = current_prev;
        let (target_next, target_prev) = match move_amount {
            move_amount if move_amount > 0 => {
                let target_next = self.walk_forward_n(current_next, move_amount.unsigned_abs());
                let target_prev = self.walk_forward_n(current_prev, move_amount.unsigned_abs());
                (target_next, target_prev)
            }
            move_amount if move_amount < 0 => {
                let target_next = self.walk_backward_n(current_next, move_amount.unsigned_abs());
                let target_prev = self.walk_backward_n(current_prev, move_amount.unsigned_abs());
                (target_next, target_prev)
            }
            _ => (current_next, current_prev),
        };
        self.next_index[index] = target_next;
        self.prev_index[target_next] = index;
        self.prev_index[index] = target_prev;
        self.next_index[target_prev] = index;
    }

    fn calculate_day_b(&mut self) -> isize {
        for i in 0..self.numbers.len() {
            self.numbers[i] *= 811589153;
        }
        for _ in 0..10 {
            for i in 0..self.numbers.len() {
                self.move_at_index(i);
            }
        }
        self.get_result()
    }
}

impl Display for Day20 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut starting_index = 0;
        for i in 0..self.numbers.len() {
            if self.numbers[i] == 0 {
                starting_index = i;
            }
        }
        for i in 0..self.numbers.len() {
            write!(
                f,
                "{},",
                self.numbers[self.walk_forward_n(starting_index, i)]
            )?
        }
        writeln!(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let mut day20 = Day20::new("data/test_data.txt").unwrap();
        let expected = 3;
        let actual = day20.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let mut day20 = Day20::new("data/test_data.txt").unwrap();
        let expected = 1623178306;
        let actual = day20.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let mut day20 = Day20::new("data/input_data.txt").unwrap();
        let actual = day20.calculate_day_a();
        assert!(actual < 6786);
        assert!(actual > 5075);
        assert_eq!(actual, 5498);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let mut day20 = Day20::new("data/input_data.txt").unwrap();
        let expected = 3390007892081;
        let actual = day20.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
