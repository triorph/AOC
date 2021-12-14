extern crate peg;
use std::collections::HashMap;

#[derive(Clone)]
struct PolymerInsertionRule {
    match_rule: String,
    replacement: String,
}

#[derive(Clone)]
pub struct Day14Setup {
    current_polymer: String,
    polymer_insertion_rules: Vec<PolymerInsertionRule>,
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
            let replacement: String = [
                match_chars.next().unwrap() ,
                replace_char.chars().next().unwrap() ,
            ].iter().collect();
            PolymerInsertionRule {match_rule, replacement}
        }
    pub rule parse() -> Day14Setup
        = current_polymer:atom() "\n" * polymer_insertion_rules:polymer_rule() ++ "\n"  "\n" * {
            Day14Setup {current_polymer, polymer_insertion_rules}
        }
}}

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

    fn find_matching_polymer_insertion_rule(
        &self,
        to_match: &str,
    ) -> Option<&PolymerInsertionRule> {
        for rule in self.polymer_insertion_rules.iter() {
            if &rule.match_rule[..] == to_match {
                return Some(rule);
            }
        }
        None
    }

    fn iterate(self: &Day14Setup) -> Day14Setup {
        println!("Running an iteration step");
        let mut next_string = String::new();
        for i in 0..(self.current_polymer.len() - 1) {
            let to_match = &self.current_polymer[i..i + 2];
            if let Some(polymer_insertion_rule) =
                self.find_matching_polymer_insertion_rule(to_match)
            {
                next_string += &polymer_insertion_rule.replacement;
            } else {
                next_string += &self.current_polymer[i..i + 1].to_string();
            }
        }
        next_string += &self.current_polymer
            [self.current_polymer.len() - 1..self.current_polymer.len()]
            .to_string();
        Day14Setup {
            current_polymer: next_string,
            polymer_insertion_rules: self.polymer_insertion_rules.clone(),
        }
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
        let mut counts: HashMap<char, usize> = HashMap::new();
        for c in self.current_polymer.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }
        let max = counts.values().reduce(std::cmp::max).unwrap();
        let min = counts.values().reduce(std::cmp::min).unwrap();
        max - min
    }

    /// Calculate the part a response
    pub fn calculate_day_a(self: &Day14Setup) -> usize {
        let state_after_10 = self.iterate_n_times(10);
        state_after_10.get_max_minus_min()
    }

    /// Calculate the part b response
    pub fn calculate_day_b(self: &Day14Setup) -> usize {
        let state_after_40 = self.iterate_n_times(30);
        state_after_40.get_max_minus_min()
    }
}

#[cfg(test)]
mod test {
    use crate::Day14Setup;

    #[test]
    fn test_parse() {
        let day14_setup = Day14Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day14_setup.current_polymer, "NNCB".to_string());
        assert_eq!(day14_setup.polymer_insertion_rules.len(), 16);
    }

    #[test]
    /// Test the iterations needed for day a, give us the string we expect.
    fn test_iterations_a() {
        let day14_setup = Day14Setup::new(include_str!("../test_data.txt"));
        let next = day14_setup.iterate();
        assert_eq!(next.current_polymer, "NCNBCHB".to_string());
        let next = day14_setup.iterate_n_times(2);
        assert_eq!(next.current_polymer, "NBCCNBBBCBHCB".to_string());
        let next = day14_setup.iterate_n_times(3);
        assert_eq!(
            next.current_polymer,
            "NBBBCNCCNBBNBNBBCHBHHBCHB".to_string()
        );
        let next = day14_setup.iterate_n_times(4);
        assert_eq!(
            next.current_polymer,
            "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB".to_string()
        );
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
