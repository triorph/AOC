mod parser;
use std::collections::HashSet;

use crate::parser::parse_data;
use aoc_helpers::{
    hash_utils::FromVec, point2d::Point2D, read_input_file, AOCCalculator, AOCFileOrParseError,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day7 {
    start: Point2D,
    splitters: HashSet<Point2D>,
}

impl AOCCalculator for Day7 {
    fn new(filename: &str) -> Result<Day7, AOCFileOrParseError> {
        let (start, splitters) = parse_data(&read_input_file(filename)?)?;
        Ok(Day7 { start, splitters })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day7 {
    fn move_beams(&self, beam_positions: &HashSet<Point2D>) -> HashSet<Point2D> {
        beam_positions
            .union(
                &beam_positions
                    .clone()
                    .difference(&self.splitters)
                    .map(|beam_pos| *beam_pos + Point2D { x: 0, y: 1 })
                    .collect::<HashSet<Point2D>>(),
            )
            .copied()
            .collect()
    }

    fn split_beams(&self, beam_positions: &HashSet<Point2D>) -> HashSet<Point2D> {
        beam_positions
            .union(
                &self
                    .splitters
                    .clone()
                    .difference(beam_positions)
                    .flat_map(|splitter_pos| {
                        vec![
                            *splitter_pos + Point2D { x: -1, y: 0 },
                            *splitter_pos + Point2D { x: 1, y: 0 },
                        ]
                    })
                    .collect::<HashSet<Point2D>>(),
            )
            .copied()
            .collect()
    }

    fn next_splitters(&self, beam_positions: &HashSet<Point2D>) -> HashSet<Point2D> {
        self.split_beams(&self.move_beams(beam_positions))
    }

    fn all_moves(&self) -> HashSet<Point2D> {
        let max = self.splitters.iter().map(|p| p.y).max().unwrap() as usize;
        (0..=max).fold(HashSet::from_vec(&[self.start]), |acc, _| {
            self.next_splitters(&acc)
        })
    }

    fn calculate_day_a(&self) -> usize {
        self.all_moves().intersection(&self.splitters).count()
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
        let day7 = Day7::new("data/test_data.txt").unwrap();
        let expected = 21;
        let actual = day7.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day7 = Day7::new("data/test_data.txt").unwrap();
        let expected = 0;
        let actual = day7.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day7 = Day7::new("data/input_data.txt").unwrap();
        let expected = 1499;
        let actual = day7.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day7 = Day7::new("data/input_data.txt").unwrap();
        let expected = 0;
        let actual = day7.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
