mod parser;
use std::collections::HashSet;

use crate::parser::parse_data;
use aoc_helpers::{
    point2d::{Neighbours, Point2D},
    read_input_file, AOCCalculator, AOCFileOrParseError,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day4 {
    paper_locations: HashSet<Point2D>,
}

impl AOCCalculator for Day4 {
    fn new(filename: &str) -> Result<Day4, AOCFileOrParseError> {
        Ok(Day4 {
            paper_locations: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day4 {
    fn find_moveable_paper_locations(
        &self,
        paper_locations: &HashSet<Point2D>,
    ) -> HashSet<Point2D> {
        paper_locations
            .iter()
            .filter(|paper_location| {
                paper_location
                    .get_8_neighbours()
                    .into_iter()
                    .filter(|neighbour_location| paper_locations.contains(neighbour_location))
                    .count()
                    < 4
            })
            .copied()
            .collect()
    }

    fn calculate_day_a(&self) -> usize {
        self.find_moveable_paper_locations(&self.paper_locations)
            .len()
    }

    fn iterate_paper_location_removal(
        &self,
        paper_locations: &HashSet<Point2D>,
    ) -> HashSet<Point2D> {
        paper_locations
            .difference(&self.find_moveable_paper_locations(paper_locations))
            .copied()
            .collect()
    }

    fn calculate_day_b(&self) -> usize {
        let mut paper_locations = self.paper_locations.clone();
        loop {
            let new_paper_locations = self.iterate_paper_location_removal(&paper_locations);
            if paper_locations.len() == new_paper_locations.len() {
                break;
            }
            paper_locations = new_paper_locations;
        }
        self.paper_locations.len() - paper_locations.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day4 = Day4::new("data/test_data.txt").unwrap();
        let expected = 13;
        let actual = day4.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day4 = Day4::new("data/test_data.txt").unwrap();
        let expected = 43;
        let actual = day4.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day4 = Day4::new("data/input_data.txt").unwrap();
        let expected = 1464;
        let actual = day4.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day4 = Day4::new("data/input_data.txt").unwrap();
        let expected = 8409;
        let actual = day4.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
