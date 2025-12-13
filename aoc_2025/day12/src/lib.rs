mod parser;
use std::collections::HashMap;

use crate::parser::{parse_data, Goal, Present, PresentTile};
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day12 {
    presents: Vec<Present>,
    goals: Vec<Goal>,
}

impl AOCCalculator for Day12 {
    fn new(filename: &str) -> Result<Day12, AOCFileOrParseError> {
        let (presents, goals) = parse_data(&read_input_file(filename)?)?;
        Ok(Day12 { presents, goals })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("There is no {}b", name);
    }
}

impl Day12 {
    fn calculate_day_a(&self) -> usize {
        let count = self.build_present_size_map();
        self.goals
            .iter()
            .filter(|goal| !self.too_many(goal, &count))
            .count()
    }

    #[allow(dead_code)]
    fn too_few(&self, goal: &Goal) -> bool {
        let num_3x3_tiles = (goal.0 / 3) * (goal.1 / 3);
        num_3x3_tiles > goal.2.iter().sum::<usize>()
    }

    fn too_many(&self, goal: &Goal, count: &HashMap<usize, usize>) -> bool {
        goal.2
            .iter()
            .enumerate()
            .map(|(i, quantity)| count.get(&i).unwrap() * quantity)
            .sum::<usize>()
            > goal.0 * goal.1
    }

    fn build_present_size_map(&self) -> HashMap<usize, usize> {
        HashMap::from_iter(self.presents.iter().map(|(index, present)| {
            (
                *index,
                present
                    .iter()
                    .flatten()
                    .filter(|tile| **tile == PresentTile::Full)
                    .count(),
            )
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day12 = Day12::new("data/test_data.txt").unwrap();
        let expected = 2;
        let actual = day12.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day12 = Day12::new("data/input_data.txt").unwrap();
        let expected = 472;
        let actual = day12.calculate_day_a();
        assert_eq!(expected, actual);
    }
}
