mod parser;
mod tunnel;
use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};
use itertools::Itertools;
use std::collections::VecDeque;

use tunnel::Tunnels;

pub struct Day16 {
    tunnels: Tunnels,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct ExploreState {
    visited: Vec<String>,
    relief: usize,
    distance_left: usize,
}

impl ExploreState {
    fn next(&self, new_node: String, distance_taken: usize, flow_rate: usize) -> ExploreState {
        let mut visited = self.visited.clone();
        visited.push(new_node);
        let distance_left = self.distance_left - distance_taken - 1;
        ExploreState {
            visited,
            distance_left,
            relief: self.relief + flow_rate * distance_left,
        }
    }
}

impl AOCCalculator for Day16 {
    fn new(filename: &str) -> Result<Day16, AOCFileOrParseError> {
        Ok(Day16 {
            tunnels: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}
impl Day16 {
    fn calculate_day_a(&self) -> usize {
        self.find_best_pressure_relief_day_a()
    }

    fn find_best_pressure_relief_day_a(&self) -> usize {
        let mut best_result = 0;
        let mut paths = VecDeque::new();
        paths.push_back(ExploreState {
            relief: 0,
            visited: vec!["AA".to_string()],
            distance_left: 30,
        });
        while let Some(state) = paths.pop_front() {
            if state.relief > best_result {
                best_result = state.relief;
            }
            let current_room = state.visited.last().unwrap();
            for unvisited in self.tunnels.get_unvisited_valves(&state.visited).iter() {
                let distance_away = self.tunnels.get_distance_between(current_room, unvisited);
                if distance_away + 1 < state.distance_left {
                    paths.push_back(state.next(
                        unvisited.to_string(),
                        distance_away,
                        self.tunnels.flow_rate(unvisited),
                    ))
                }
            }
        }
        best_result
    }

    fn calculate_day_b(&self) -> usize {
        self.find_best_pressure_relief_day_b()
    }

    fn find_best_pressure_relief_day_b(&self) -> usize {
        let mut best_result = 0;
        let mut paths = VecDeque::new();
        paths.push_back(vec![
            ExploreState {
                relief: 0,
                visited: vec!["AA".to_string()],
                distance_left: 26,
            },
            ExploreState {
                relief: 0,
                visited: vec!["AA".to_string()],
                distance_left: 26,
            },
        ]);
        while let Some(state_vec) = paths.pop_back() {
            let total_relief = state_vec.iter().map(|state| state.relief).sum();
            if total_relief > best_result {
                best_result = total_relief
            }
            let all_visited_nodes: Vec<String> = state_vec
                .iter()
                .flat_map(|state| state.visited.clone())
                .collect();
            let all_unvisited_nodes: Vec<String> =
                self.tunnels.get_unvisited_valves(&all_visited_nodes);
            for (i, state) in state_vec.iter().enumerate() {
                let current_room = state.visited.last().unwrap();
                let potential: usize = all_unvisited_nodes
                    .iter()
                    .map(|dest| {
                        self.tunnels.get_possible_pressure_relief(
                            current_room,
                            dest,
                            state.distance_left,
                        )
                    })
                    .sum();
                if total_relief + potential < best_result {
                    continue;
                }
                for unvisited in all_unvisited_nodes.iter() {
                    let distance_away = self.tunnels.get_distance_between(current_room, unvisited);
                    if distance_away + 1 < state.distance_left {
                        let mut next_state_vec = state_vec.clone();
                        next_state_vec[i] = state.next(
                            unvisited.to_string(),
                            distance_away,
                            self.tunnels.flow_rate(unvisited),
                        );
                        paths.push_back(next_state_vec)
                    }
                }
            }
        }
        best_result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day16 = Day16::new("data/test_data.txt").unwrap();
        let expected = 1651;
        let actual = day16.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day16 = Day16::new("data/test_data.txt").unwrap();
        let expected = 1707;
        let actual = day16.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_a_real_input() {
        let day16 = Day16::new("data/input_data.txt").unwrap();
        let expected = 2080;
        let actual = day16.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b_real_input() {
        let day16 = Day16::new("data/input_data.txt").unwrap();
        let expected = 0;
        let actual = day16.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
