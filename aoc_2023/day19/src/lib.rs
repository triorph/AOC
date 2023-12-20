mod filter_chain;
mod filter_rule;
mod parser;
mod part;
use std::collections::HashMap;

use crate::parser::parse_data;
use crate::part::Part;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};
use filter_chain::FilterChain;
use part::PartRange;

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

    fn get_next_range(&self, range: &PartRange) -> Vec<PartRange> {
        if range.target == "A" || range.target == "R" {
            vec![range.clone()]
        } else {
            self.filter_chains
                .get(&range.target)
                .unwrap()
                .next_target_range(range)
        }
    }

    fn get_range_options(&self) -> Vec<PartRange> {
        let mut ranges = vec![PartRange::first()];
        while ranges
            .iter()
            .any(|range| range.target != "A" && range.target != "R")
        {
            ranges = ranges
                .into_iter()
                .flat_map(|range| self.get_next_range(&range))
                .collect();
        }
        println!("Got ranges: {:?}", ranges);
        ranges
    }

    fn calculate_day_b(&self) -> usize {
        self.get_range_options()
            .into_iter()
            .filter(|range| range.target == "A")
            .map(|range| range.get_size())
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
        let expected = 19114;
        let actual = day19.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day19 = Day19::new("data/test_data.txt").unwrap();
        let expected = 167409079868000;
        let actual = day19.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day19 = Day19::new("data/input_data.txt").unwrap();
        let expected = 420739;
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
