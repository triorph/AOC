mod direction;
mod parser;
use std::collections::HashMap;

use crate::parser::parse_data;
use aoc_helpers::modular_math::least_common_multiple;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};
use direction::Direction;
use parser::NodeMap;

pub struct Day8 {
    directions: Vec<Direction>,
    all_nodes: NodeMap,
}

impl AOCCalculator for Day8 {
    fn new(filename: &str) -> Result<Day8, AOCFileOrParseError> {
        let (directions, all_nodes) = parse_data(&read_input_file(filename)?)?;
        Ok(Day8 {
            directions,
            all_nodes,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day8 {
    fn get_next_node(&self, node_name: &String, direction: &Direction) -> String {
        let (left, right) = self.all_nodes.get(node_name).unwrap();
        match direction {
            Direction::Left => left.clone(),
            Direction::Right => right.clone(),
        }
    }
    fn calculate_day_a(&self) -> usize {
        let mut current_node = "AAA".to_string();
        for (i, direction) in self.directions.iter().cycle().enumerate() {
            if current_node == "ZZZ" {
                return i;
            }
            current_node = self.get_next_node(&current_node, direction);
        }
        panic!("Will never reach here");
    }

    fn detect_cycle(path: &[String]) -> Option<usize> {
        const MIN_REPEATS: usize = 5;
        (1..=(path.len() / MIN_REPEATS)).find(|&cycle_size| {
            (0..cycle_size)
                .map(|offset| {
                    // check all elements at offset are the same
                    (0..MIN_REPEATS)
                        .map(|cycle| path[path.len() - 1 - cycle * cycle_size - offset].clone())
                        .collect::<Vec<String>>()
                        .windows(2)
                        .all(|window| window[0] == window[1])
                })
                .all(|x| x)
            // all offsets are equal at cycle_size steps
        })
    }

    fn build_full_direction_node_map(&self) -> HashMap<String, String> {
        let starter_nodes: Vec<String> = self.all_nodes.keys().cloned().collect();
        let mut current_nodes = starter_nodes.clone();
        for direction in self.directions.iter() {
            current_nodes = current_nodes
                .into_iter()
                .map(|current_node| self.get_next_node(&current_node, direction))
                .collect()
        }
        HashMap::from_iter(starter_nodes.into_iter().zip(current_nodes))
    }

    fn get_cycle_lengths_for_day_b_nodes(&self, starter_nodes: &[String]) -> Vec<usize> {
        let mut node_paths: Vec<Vec<String>> = starter_nodes
            .iter()
            .map(|node| vec![node.clone()])
            .collect();
        let full_direction_node_map = self.build_full_direction_node_map();
        let mut cycle_lengths: Vec<Option<usize>> = node_paths.iter().map(|_| None).collect();
        loop {
            // insert the next value from the node_map to the node_path
            node_paths = node_paths
                .into_iter()
                .map(|mut x| {
                    x.push(
                        full_direction_node_map
                            .get(&x[x.len() - 1])
                            .unwrap()
                            .clone(),
                    );
                    x
                })
                .collect();
            // update the cycle_lengths if not found yet
            cycle_lengths = cycle_lengths
                .into_iter()
                .enumerate()
                .map(|(path_i, cycle_length)| {
                    if cycle_length.is_none() {
                        Day8::detect_cycle(&node_paths[path_i])
                    } else {
                        cycle_length
                    }
                })
                .collect();
            if cycle_lengths.iter().all(|x| x.is_some()) {
                break;
            }
        }
        cycle_lengths
            .iter()
            .map(|x| x.unwrap() * self.directions.len())
            .collect()
    }

    fn calculate_day_b(&self) -> usize {
        let starter_nodes: Vec<String> = self
            .all_nodes
            .keys()
            .filter(|x| x.ends_with('A'))
            .cloned()
            .collect();
        let cycle_lengths = self.get_cycle_lengths_for_day_b_nodes(&starter_nodes);
        let lcm = cycle_lengths
            .iter()
            .copied()
            .reduce(least_common_multiple)
            .unwrap();
        lcm
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day8 = Day8::new("data/test_data.txt").unwrap();
        let expected = 6;
        let actual = day8.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day8 = Day8::new("data/test_data_b.txt").unwrap();
        let expected = 6;
        let actual = day8.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day8 = Day8::new("data/input_data.txt").unwrap();
        let expected = 24253;
        let actual = day8.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day8 = Day8::new("data/input_data.txt").unwrap();
        let expected = 12357789728873;
        let actual = day8.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn detect_cycle_size() {
        for (cycle, expected) in [
            (
                [
                    "C", "C", "C", "C", "C", "A", "B", "A", "B", "A", "B", "A", "B", "A", "B",
                ],
                Some(2),
            ),
            (
                [
                    "C", "C", "C", "C", "C", "A", "B", "A", "B", "A", "A", "A", "B", "A", "B",
                ],
                None,
            ),
            (
                [
                    "C", "C", "C", "C", "C", "A", "B", "A", "B", "A", "B", "B", "B", "B", "B",
                ],
                Some(1),
            ),
            (
                [
                    "C", "A", "B", "C", "A", "B", "C", "A", "B", "C", "A", "B", "C", "A", "B",
                ],
                Some(3),
            ),
        ]
        .into_iter()
        {
            let actual = Day8::detect_cycle(
                &cycle
                    .into_iter()
                    .map(|x: &str| x.to_string())
                    .collect::<Vec<String>>(),
            );
            assert_eq!(actual, expected);
        }
    }
}
