mod blizzard;
mod parser;
use std::collections::HashSet;
use std::collections::VecDeque;

use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};

use crate::blizzard::AllBlizzards;
use crate::blizzard::Point;

#[derive(Clone)]
pub struct Day24 {
    blizzards: AllBlizzards,
}

impl AOCCalculator for Day24 {
    fn new(filename: &str) -> Result<Day24, AOCFileOrParseError> {
        Ok(Day24 {
            blizzards: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        let mut obj = self.clone();
        println!("{}a answer is {:?}", name, obj.calculate_day_a());
        println!("{}b answer is {:?}", name, obj.calculate_day_b());
    }
}

impl Day24 {
    fn calculate_day_a(&mut self) -> usize {
        self.find_time_until(
            &Point { x: 0, y: -1 },
            &Point {
                x: self.blizzards.width as isize - 1,
                y: self.blizzards.height as isize,
            },
            0,
        )
    }

    fn calculate_day_b(&mut self) -> usize {
        let start = Point { x: 0, y: -1 };
        let end = Point {
            x: self.blizzards.width as isize - 1,
            y: self.blizzards.height as isize,
        };
        let time_to_end = self.find_time_until(&start, &end, 0);
        let time_back = self.find_time_until(&end, &start, time_to_end);
        self.find_time_until(&start, &end, time_back)
    }
}

impl Day24 {
    fn find_time_until(
        &mut self,
        current_node: &Point,
        target: &Point,
        starting_time: usize,
    ) -> usize {
        let mut stack = VecDeque::new();
        stack.push_back((*current_node, starting_time));
        let mut best_time: Option<usize> = None;
        let mut seen = HashSet::new();
        while let Some((current_node, time_taken)) = stack.pop_front() {
            if seen.contains(&(current_node, time_taken)) {
                continue;
            } else {
                seen.insert((current_node, time_taken));
            }
            if &current_node == target {
                if best_time.is_none() || best_time.unwrap() > time_taken {
                    best_time = Some(time_taken);
                }
                continue;
            }
            if best_time.is_some() && time_taken >= best_time.unwrap() {
                continue;
            }
            let next_moves = current_node
                .next_moves()
                .into_iter()
                .filter(|next_move| self.blizzards.is_move_valid(next_move, time_taken + 1));
            for next_move in next_moves {
                stack.push_back((next_move, time_taken + 1));
            }
        }
        best_time.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let mut day24 = Day24::new("data/test_data.txt").unwrap();
        let expected = 18;
        let actual = day24.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let mut day24 = Day24::new("data/test_data.txt").unwrap();
        let expected = 54;
        let actual = day24.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let mut day24 = Day24::new("data/input_data.txt").unwrap();
        let expected = 260;
        let actual = day24.calculate_day_a();
        assert!(actual < 525);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let mut day24 = Day24::new("data/input_data.txt").unwrap();
        let expected = 747;
        let actual = day24.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
