mod parser;
use std::collections::HashMap;

use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day11 {
    server_rack: HashMap<String, Vec<String>>,
}

impl AOCCalculator for Day11 {
    fn new(filename: &str) -> Result<Day11, AOCFileOrParseError> {
        Ok(Day11 {
            server_rack: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day11 {
    fn calculate_day_a(&self) -> usize {
        self.find_paths_from_source_to_dest("you", "out")
    }

    fn calculate_day_b(&self) -> usize {
        self.find_paths_from_source_to_dest("svr", "fft")
            * self.find_paths_from_source_to_dest("fft", "dac")
            * self.find_paths_from_source_to_dest("dac", "out")
            + self.find_paths_from_source_to_dest("svr", "dac")
                * self.find_paths_from_source_to_dest("dac", "fft")
                * self.find_paths_from_source_to_dest("fft", "out")
    }

    fn find_paths_from_source_to_dest(&self, source: &str, dest: &str) -> usize {
        let path_count_map = self.build_path_count_to_target(&HashMap::new(), source, dest);
        *path_count_map.get(source).unwrap()
    }

    fn build_path_count_to_target(
        &self,
        path_count_cache: &HashMap<String, usize>,
        source: &str,
        dest: &str,
    ) -> HashMap<String, usize> {
        // basically doing memoisation of the result (assumes no cycles)
        let mut path_count = 0;
        let mut ret = path_count_cache.clone();
        if let Some(next_nodes) = self.server_rack.get(source) {
            for next_node in next_nodes.iter() {
                if next_node == dest {
                    path_count += 1;
                } else if let Some(next_node_num_paths) = ret.get(next_node) {
                    path_count += next_node_num_paths
                } else {
                    ret = self.build_path_count_to_target(&ret, next_node, dest);
                    path_count += ret.get(next_node).unwrap();
                }
            }
        }
        ret.insert(source.to_string(), path_count);
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day11 = Day11::new("data/test_data.txt").unwrap();
        let expected = 5;
        let actual = day11.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day11 = Day11::new("data/test_data_2.txt").unwrap();
        let expected = 2;
        let actual = day11.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day11 = Day11::new("data/input_data.txt").unwrap();
        let expected = 566;
        let actual = day11.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day11 = Day11::new("data/input_data.txt").unwrap();
        let expected = 331837854931968;
        let actual = day11.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
