extern crate peg;
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct PolymerInsertionRule {
    match_rule: String,
    rules_updated: [String; 2],
    char_to_insert: char,
    rule_count: usize,
}

#[derive(Clone)]
pub struct Day14Setup {
    polymer_insertion_rules: HashMap<String, PolymerInsertionRule>,
    polymer_counts: HashMap<char, usize>,
}

struct Day14Iterator {
    current_iteration: Day14Setup,
}

peg::parser! { grammar day14_parser() for str {
    rule atom() -> String
        = n:$(['A'..='Z']+) { String::from(n) }
    rule polymer_rule() -> PolymerInsertionRule
        = match_rule:atom() " -> " replace_char:atom() {
            let mut match_chars = match_rule.chars();
            let char_to_insert = replace_char.chars().next().unwrap();
            let rule_updated_1: String = [
                match_chars.next().unwrap() ,
                char_to_insert,
            ].iter().collect();
            let rule_updated_2: String = [
                char_to_insert,
                match_chars.next().unwrap(),
            ].iter().collect();
            PolymerInsertionRule {
                match_rule,
                rules_updated: [rule_updated_1, rule_updated_2],
                rule_count: 0,
                char_to_insert
            }
        }
    pub rule parse() -> Day14Setup
        = current_polymer:atom() "\n" * polymer_insertion_rules:polymer_rule() ++ "\n"  "\n" * {
            let mut rules_map: HashMap<String, PolymerInsertionRule> = HashMap::new();
            let mut count_map: HashMap<char, usize> = HashMap::new();
            for mut polymer_insertion_rule in polymer_insertion_rules.into_iter() {
                polymer_insertion_rule.count_in(&current_polymer[..]);
                rules_map.insert(polymer_insertion_rule.match_rule.clone(), polymer_insertion_rule);
            }
            for c in current_polymer.chars() {
                *count_map.entry(c).or_insert(0) += 1;
            }
            Day14Setup { polymer_insertion_rules: rules_map, polymer_counts: count_map }
        }
}}

impl PolymerInsertionRule {
    fn count_in(&mut self, polymer: &str) {
        let mut count = 0;
        for i in 0..(polymer.len() - 1) {
            if polymer[i..i + 2] == self.match_rule[..] {
                count += 1;
            }
        }
        self.rule_count = count;
    }

    fn blank() -> PolymerInsertionRule {
        PolymerInsertionRule {
            match_rule: "".to_string(),
            rules_updated: ["foo".to_string(), "bar".to_string()],
            rule_count: 0,
            char_to_insert: '\r',
        }
    }
}

impl Iterator for Day14Iterator {
    type Item = Day14Setup;
    fn next(self: &mut Day14Iterator) -> Option<Self::Item> {
        self.current_iteration = self.current_iteration.iterate();
        Some(self.current_iteration.clone())
    }
}

impl Day14Setup {
    /// Generates a new Day14Setup object to be calculated upon
    ///
    /// Inputs: the full string contents of the input data set.
    ///
    /// Returns: A new Day14Setup object, with methods `calculate_day_a` and `calculate_day_b`
    /// available
    pub fn new(input_str: &str) -> Day14Setup {
        day14_parser::parse(input_str).unwrap()
    }

    fn iterate(self: &Day14Setup) -> Day14Setup {
        let mut next = self.clone();
        for polymer_insertion_rule in self.polymer_insertion_rules.values() {
            (*next
                .polymer_insertion_rules
                .entry(polymer_insertion_rule.rules_updated[0].clone())
                .or_insert_with(PolymerInsertionRule::blank))
            .rule_count += polymer_insertion_rule.rule_count;
            (*next
                .polymer_insertion_rules
                .entry(polymer_insertion_rule.rules_updated[1].clone())
                .or_insert_with(PolymerInsertionRule::blank))
            .rule_count += polymer_insertion_rule.rule_count;
            (*next
                .polymer_insertion_rules
                .entry(polymer_insertion_rule.match_rule.clone())
                .or_insert_with(PolymerInsertionRule::blank))
            .rule_count -= polymer_insertion_rule.rule_count;
            *next
                .polymer_counts
                .entry(polymer_insertion_rule.char_to_insert)
                .or_insert(0) += polymer_insertion_rule.rule_count;
        }
        next
    }

    fn iter(self: &Day14Setup) -> Day14Iterator {
        Day14Iterator {
            current_iteration: self.clone(),
        }
    }

    fn iterate_n_times(&self, n: usize) -> Day14Setup {
        self.iter().take(n).last().unwrap()
    }

    fn get_max_minus_min(&self) -> usize {
        let max = self.polymer_counts.values().reduce(std::cmp::max).unwrap();
        let min = self.polymer_counts.values().reduce(std::cmp::min).unwrap();
        max - min
    }

    /// Calculate the part a response
    pub fn calculate_day_a(self: &Day14Setup) -> usize {
        let state_after_10 = self.iterate_n_times(10);
        state_after_10.get_max_minus_min()
    }

    /// Calculate the part b response
    pub fn calculate_day_b(self: &Day14Setup) -> usize {
        let state_after_40 = self.iterate_n_times(40);
        state_after_40.get_max_minus_min()
    }
}

#[cfg(test)]
mod test {
    use crate::Day14Setup;

    #[test]
    fn test_parse() {
        let day14_setup = Day14Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day14_setup.polymer_insertion_rules.len(), 16);
        assert_eq!(day14_setup.polymer_counts.get(&'N').unwrap(), &2);
        assert_eq!(day14_setup.polymer_counts.get(&'C').unwrap(), &1);
        assert_eq!(day14_setup.polymer_counts.get(&'B').unwrap(), &1);
    }

    #[test]
    fn test_1_iteration() {
        // "NCNBCHB"
        let after_iteration = Day14Setup::new(include_str!("../test_data.txt")).iterate_n_times(1);
        assert_eq!(after_iteration.polymer_counts.get(&'N').unwrap(), &2);
        assert_eq!(after_iteration.polymer_counts.get(&'C').unwrap(), &2);
        assert_eq!(after_iteration.polymer_counts.get(&'B').unwrap(), &2);
        assert_eq!(after_iteration.polymer_counts.get(&'H').unwrap(), &1);
    }

    #[test]
    fn test_2_iterations() {
        // "NBCCNBBBCBHCB"
        let after_iteration = Day14Setup::new(include_str!("../test_data.txt")).iterate_n_times(2);
        assert_eq!(after_iteration.polymer_counts.get(&'N').unwrap(), &2);
        assert_eq!(after_iteration.polymer_counts.get(&'C').unwrap(), &4);
        assert_eq!(after_iteration.polymer_counts.get(&'B').unwrap(), &6);
        assert_eq!(after_iteration.polymer_counts.get(&'H').unwrap(), &1);
    }

    #[test]
    fn test_day_a() {
        let day14_setup = Day14Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day14_setup.calculate_day_a(), 1588);
    }

    #[test]
    fn test_day_b() {
        let day14_setup = Day14Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day14_setup.calculate_day_b(), 2188189693529);
    }
}
