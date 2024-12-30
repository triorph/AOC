mod parser;
use std::collections::{HashMap, VecDeque};

use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day19 {
    patterns: Vec<String>,
    desired: Vec<String>,
}

impl AOCCalculator for Day19 {
    fn new(filename: &str) -> Result<Day19, AOCFileOrParseError> {
        let (patterns, desired) = parse_data(&read_input_file(filename)?)?;
        Ok(Day19 { patterns, desired })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day19 {
    fn is_desired_possible(&self, desired: &str) -> bool {
        let mut to_explore = VecDeque::new();
        to_explore.push_back(0);
        while let Some(explore) = to_explore.pop_front() {
            for pattern in self.patterns.iter() {
                if desired[explore..desired.len()].starts_with(pattern) {
                    if explore + pattern.len() == desired.len() {
                        return true;
                    }
                    if !to_explore.contains(&(explore + pattern.len())) {
                        to_explore.push_back(explore + pattern.len())
                    }
                }
            }
        }
        false
    }

    fn ways_to_end(&self, desired: &str) -> usize {
        let mut ways_to_point: HashMap<usize, usize> = HashMap::new();
        ways_to_point.insert(0, 1);
        for i in 0..desired.len() {
            let paths_here = *ways_to_point.get(&i).unwrap_or(&0);
            if paths_here == 0 {
                continue;
            }
            for pattern in self.patterns.iter() {
                if desired[i..desired.len()].starts_with(pattern) {
                    let next = i + pattern.len();
                    *ways_to_point.entry(next).or_default() += paths_here;
                }
            }
        }
        *ways_to_point.get(&(desired.len())).unwrap_or(&0)
    }

    fn calculate_day_a(&self) -> usize {
        self.desired
            .iter()
            .filter(|desired| self.is_desired_possible(desired))
            .count()
    }

    fn calculate_day_b(&self) -> usize {
        self.desired
            .iter()
            .map(|desired| self.ways_to_end(desired))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day19 = Day19::new("data/test_data.txt").unwrap();
        let expected = 6;
        let actual = day19.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day19 = Day19::new("data/test_data.txt").unwrap();
        let expected = 16;
        let actual = day19.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day19 = Day19::new("data/input_data.txt").unwrap();
        let expected = 365;
        let actual = day19.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day19 = Day19::new("data/input_data.txt").unwrap();
        let expected = 730121486795169;
        let actual = day19.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
