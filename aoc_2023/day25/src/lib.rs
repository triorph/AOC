mod parser;
use crate::parser::parse_data;
use aoc_helpers::{hash_utils::HashVec, read_input_file, AOCCalculator, AOCFileOrParseError};
use std::collections::HashSet;

fn build_map(connections: &[(String, String)]) -> HashVec<String, String> {
    let mut ret = HashVec::new();
    for (left, right) in connections.iter() {
        ret.push(left.to_string(), right.to_string());
        ret.push(right.to_string(), left.to_string());
    }
    ret
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day25 {
    connections: Vec<(String, String)>,
    connections_map: HashVec<String, String>,
}

impl AOCCalculator for Day25 {
    fn new(filename: &str) -> Result<Day25, AOCFileOrParseError> {
        let connections = parse_data(&read_input_file(filename)?)?;
        Ok(Day25 {
            connections_map: build_map(&connections),
            connections,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day25 {
    fn how_many_neighbours_in_set(&self, set: &HashSet<String>, node: &str) -> usize {
        HashSet::from_iter(self.connections_map.get(&node.to_string()).iter().cloned())
            .difference(set)
            .count()
    }

    fn get_groups(&self) -> HashSet<String> {
        let mut others = self.all_components();
        while others
            .iter()
            .map(|node| self.how_many_neighbours_in_set(&others, node))
            .sum::<usize>()
            != 3
        {
            let max = others
                .iter()
                .max_by(|left, right| {
                    self.how_many_neighbours_in_set(&others, left)
                        .cmp(&self.how_many_neighbours_in_set(&others, right))
                })
                .unwrap()
                .clone();
            others.remove(&max);
        }
        others
    }

    fn all_components(&self) -> HashSet<String> {
        let mut ret = HashSet::new();
        for (left, right) in self.connections.iter() {
            ret.insert(left.clone());
            ret.insert(right.clone());
        }
        ret
    }

    fn calculate_day_a(&self) -> usize {
        let groups = self.get_groups();
        let first = self.all_components().len() - groups.len();
        let second = groups.len();
        println!(
            "Got group sizes {}, {} and {}, giving total {}",
            self.all_components().len(),
            first,
            second,
            first * second
        );
        first * second
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
        let day25 = Day25::new("data/test_data.txt").unwrap();
        let expected = 54;
        let actual = day25.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day25 = Day25::new("data/test_data.txt").unwrap();
        let expected = 0;
        let actual = day25.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day25 = Day25::new("data/input_data.txt").unwrap();
        let expected = 0;
        let actual = day25.calculate_day_a();
        assert!(actual < 554840, "answer of {} is too high", actual);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day25 = Day25::new("data/input_data.txt").unwrap();
        let expected = 0;
        let actual = day25.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
