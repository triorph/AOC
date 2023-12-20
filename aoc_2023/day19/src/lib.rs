mod filter_chain;
mod filter_rule;
mod parser;
mod part;
use std::collections::HashMap;

use crate::parser::parse_data;
use crate::part::Part;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};
use filter_chain::FilterChain;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day19 {
    filter_chains: HashMap<String, FilterChain>,
    parts: Vec<Part>,
}

impl AOCCalculator for Day19 {
    fn new(filename: &str) -> Result<Day19, AOCFileOrParseError> {
        let (filter_rules, parts) = parse_data(&read_input_file(filename)?)?;
        Ok(Day19 {
            filter_chains: filter_rules,
            parts,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day19 {
    fn repeat_filter_chains(&self, part: &Part) -> bool {
        let mut target = "in".to_string();
        while target != "A" && target != "R" {
            let chain = self.filter_chains.get(&target).unwrap();
            target = chain.next_target(part);
        }
        target == "A"
    }

    fn calculate_day_a(&self) -> usize {
        self.parts
            .iter()
            .filter(|part| self.repeat_filter_chains(part))
            .map(|part| part.get_rating())
            .sum()
    }

    fn calculate_day_b(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day19 = Day19::new("data/test_data.txt").unwrap();
        let expected = 19114;
        let actual = day19.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day19 = Day19::new("data/test_data.txt").unwrap();
        let expected = 0;
        let actual = day19.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day19 = Day19::new("data/input_data.txt").unwrap();
        let expected = 0;
        let actual = day19.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day19 = Day19::new("data/input_data.txt").unwrap();
        let expected = 0;
        let actual = day19.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
