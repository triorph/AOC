mod parser;
use std::collections::{HashMap, HashSet};

use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day23 {
    edges: Vec<(String, String)>,
}

impl AOCCalculator for Day23 {
    fn new(filename: &str) -> Result<Day23, AOCFileOrParseError> {
        Ok(Day23 {
            edges: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day23 {
    fn build_connected_map(&self) -> HashMap<String, HashSet<String>> {
        let mut connected: HashMap<String, HashSet<String>> = HashMap::new();
        for (left, right) in self.edges.iter() {
            connected
                .entry(left.clone())
                .or_default()
                .insert(right.clone());
            connected
                .entry(right.clone())
                .or_default()
                .insert(left.clone());
        }
        connected
    }

    fn find_largest_connection(&self) -> HashSet<String> {
        let connected = self.build_connected_map();
        let mut possible_groups: Vec<HashSet<String>> = connected
            .keys()
            .map(|x| HashSet::from([x.clone()]))
            .collect();
        let mut biggest_path = 1;
        loop {
            let mut next = vec![];
            let mut change_made = false;
            for group in possible_groups.clone().into_iter() {
                let mut all_neighbours = HashSet::new();
                for item in group.iter() {
                    all_neighbours.extend(connected.get(item).unwrap());
                }
                let mut found = false;
                for neighbour in all_neighbours.iter() {
                    if group
                        .iter()
                        .all(|n| connected.get(*neighbour).unwrap().contains(n))
                    {
                        found = true;
                        change_made = true;
                        let mut next_group = group.clone();
                        next_group.insert((*neighbour).clone());
                        biggest_path = next_group.len().max(biggest_path);
                        next.push(next_group);
                        break;
                    }
                }
                if !found && group.len() == biggest_path {
                    next.push(group.clone());
                }
            }
            if !change_made {
                return next.into_iter().find(|n| n.len() == biggest_path).unwrap();
            }
            possible_groups = next;
        }
    }

    fn find_all_connections(&self) -> HashSet<Vec<String>> {
        let connected = self.build_connected_map();
        let mut ret = HashSet::new();
        for (left, right) in self.edges.iter() {
            let overlap: HashSet<String> = connected
                .get(left)
                .unwrap()
                .intersection(connected.get(right).unwrap())
                .cloned()
                .collect();
            for node in overlap.iter() {
                let mut connection = vec![left.clone(), right.clone(), node.clone()];
                connection.sort();
                ret.insert(connection);
            }
        }
        ret
    }
    fn calculate_day_a(&self) -> usize {
        self.find_all_connections()
            .into_iter()
            .filter(|x| x.iter().any(|inner| inner.starts_with('t')))
            .count()
    }

    fn calculate_day_b(&self) -> String {
        let best_path = self.find_largest_connection();
        let mut best_path = Vec::from_iter(best_path);
        best_path.sort();
        best_path.join(",")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day23 = Day23::new("data/test_data.txt").unwrap();
        let expected = 7;
        let actual = day23.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day23 = Day23::new("data/test_data.txt").unwrap();
        let expected = "co,de,ka,ta".to_string();
        let actual = day23.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day23 = Day23::new("data/input_data.txt").unwrap();
        let expected = 998;
        let actual = day23.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day23 = Day23::new("data/input_data.txt").unwrap();
        let expected = "cc,ff,fh,fr,ny,oa,pl,rg,uj,wd,xn,xs,zw".to_string();
        let actual = day23.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
