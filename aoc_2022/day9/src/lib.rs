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
        self.calculate_x_followers(1)
    }

    fn calculate_day_b(&self) -> usize {
        self.calculate_x_followers(9)
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day9 {
    fn calculate_x_followers(&self, x: usize) -> usize {
        let mut t_vals: HashSet<Point> = HashSet::new();
        let mut knots = Day9::make_empty_knots(x + 1);
        for dir in self.data.iter() {
            for one_step in dir.get_steps().iter() {
                knots = Day9::follow_the_leader(&knots, one_step);
                t_vals.insert(knots[knots.len() - 1]);
            }
        }
        t_vals.len()
    }

    fn make_empty_knots(capacity: usize) -> Vec<Point> {
        let mut ret = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            ret.push(Point(0, 0));
        }
        ret
    }

    fn follow_the_leader(knots: &[Point], one_step: &Point) -> Vec<Point> {
        let mut ret = Day9::make_empty_knots(knots.len());
        ret[0] = &knots[0] + one_step;
        for i in 1..knots.len() {
            ret[i] = knots[i].follow_other(&ret[i - 1], 1)
        }
        ret
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
