mod cube_set;
mod game;
mod parser;
use crate::cube_set::{CubeSet, CubeSetTrait};
use crate::game::Game;
use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};

pub struct Day2 {
    games: Vec<Game>,
}

impl AOCCalculator for Day2 {
    fn new(filename: &str) -> Result<Day2, AOCFileOrParseError> {
        Ok(Day2 {
            games: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day2 {
    fn calculate_day_a(&self) -> usize {
        let reference = CubeSet::from([
            ("red".to_string(), 12),
            ("green".to_string(), 13),
            ("blue".to_string(), 14),
        ]);
        self.games
            .iter()
            .filter(|game| game.could_be_from_cubeset(&reference))
            .map(|game| game.id)
            .sum()
    }

    fn calculate_day_b(&self) -> usize {
        self.games
            .iter()
            .map(|x| x.find_minimum_coverage())
            .map(|x| x.get_power())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day2 = Day2::new("data/test_data.txt").unwrap();
        let expected = 8;
        let actual = day2.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day2 = Day2::new("data/test_data.txt").unwrap();
        let expected = 2286;
        let actual = day2.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day2 = Day2::new("data/input_data.txt").unwrap();
        let expected = 2256;
        let actual = day2.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day2 = Day2::new("data/input_data.txt").unwrap();
        let expected = 74229;
        let actual = day2.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
