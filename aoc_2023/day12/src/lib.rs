mod parser;
use std::collections::HashMap;

use crate::parser::parse_data;
use crate::parser::Line;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};

pub struct Day12 {
    lines: Vec<Line>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct ArrangementLocation {
    truth_len: usize,
    arrangement_index: usize,
}

impl ArrangementLocation {
    fn first() -> ArrangementLocation {
        ArrangementLocation {
            truth_len: 0,
            arrangement_index: 0,
        }
    }
}

fn get_next_false(
    previous: &HashMap<ArrangementLocation, usize>,
    arrangements: &[usize],
) -> HashMap<ArrangementLocation, usize> {
    let mut ret = HashMap::new();
    for (key, value) in previous.iter() {
        if key.truth_len == 0 {
            *ret.entry(*key).or_insert(0) += value;
        } else if key.truth_len == arrangements[key.arrangement_index] {
            *ret.entry(ArrangementLocation {
                truth_len: 0,
                arrangement_index: key.arrangement_index + 1,
            })
            .or_insert(0) += value;
        }
    }
    ret
}

fn get_next_true(
    previous: &HashMap<ArrangementLocation, usize>,
    arrangements: &[usize],
) -> HashMap<ArrangementLocation, usize> {
    let mut ret = HashMap::new();
    for (key, value) in previous.iter() {
        if key.arrangement_index < arrangements.len()
            && key.truth_len < arrangements[key.arrangement_index]
        {
            *ret.entry(ArrangementLocation {
                truth_len: key.truth_len + 1,
                arrangement_index: key.arrangement_index,
            })
            .or_insert(0) += value
        }
    }
    ret
}

fn get_next_both(
    previous: &HashMap<ArrangementLocation, usize>,
    arrangements: &[usize],
) -> HashMap<ArrangementLocation, usize> {
    let mut ret = get_next_true(previous, arrangements);
    for (key, value) in get_next_false(previous, arrangements) {
        *ret.entry(key).or_insert(0) += value;
    }
    ret
}

fn get_permutations(
    path: &[Option<bool>],
    arrangements: &[usize],
) -> HashMap<ArrangementLocation, usize> {
    let mut ret = HashMap::new();
    ret.insert(ArrangementLocation::first(), 1);
    for value in path.iter() {
        ret = match value {
            Some(true) => get_next_true(&ret, arrangements),
            Some(false) => get_next_false(&ret, arrangements),
            None => get_next_both(&ret, arrangements),
        };
    }
    ret
}

fn find_length_day_a(path: &[Option<bool>], arrangements: &[usize]) -> usize {
    let permutations = get_permutations(path, arrangements);
    let final_permute = get_next_false(&permutations, arrangements);
    println!("Final permutation is: {:?}", final_permute);
    *final_permute
        .get(&ArrangementLocation {
            truth_len: 0,
            arrangement_index: arrangements.len(),
        })
        .unwrap_or(&0)
}

fn find_length_day_b(path: &[Option<bool>], arrangements: &[usize]) -> usize {
    let mut arrangements_multiplied = arrangements.to_owned();
    let mut path_multiplied = path.to_owned();
    for _ in 0..4 {
        arrangements_multiplied.extend(arrangements);
        path_multiplied.push(None);
        path_multiplied.extend(path);
    }
    find_length_day_a(&path_multiplied, &arrangements_multiplied)
}

impl AOCCalculator for Day12 {
    fn new(filename: &str) -> Result<Day12, AOCFileOrParseError> {
        Ok(Day12 {
            lines: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day12 {
    fn calculate_day_a(&self) -> usize {
        self.lines
            .iter()
            .map(|(places, arrangements)| find_length_day_a(places, arrangements))
            .sum()
    }

    fn calculate_day_b(&self) -> usize {
        self.lines
            .iter()
            .map(|(places, arrangements)| find_length_day_b(places, arrangements))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day12 = Day12::new("data/test_data.txt").unwrap();
        let expected = 21;
        let actual = day12.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day12 = Day12::new("data/test_data.txt").unwrap();
        let expected = 525152;
        let actual = day12.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day12 = Day12::new("data/input_data.txt").unwrap();
        let expected = 7084;
        let actual = day12.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day12 = Day12::new("data/input_data.txt").unwrap();
        let expected = 8414003326821;
        let actual = day12.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
