mod parser;
use std::collections::{HashMap, HashSet};

use crate::parser::parse_data;
use aoc_helpers::{point2d::Point2D, read_input_file, AOCCalculator, AOCFileOrParseError};

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
    fn calculate_day_a(&self) -> usize {
        // find how many tachyon lines overlap splitters
        self.get_all_tachyon_lines()
            .values()
            .map(|line| line.keys().copied().collect::<HashSet<Point2D>>())
            .reduce(|a, b| a.union(&b).copied().collect::<HashSet<Point2D>>())
            .unwrap()
            .intersection(&self.splitters)
            .count()
    }

    fn calculate_day_b(&self) -> usize {
        // sum how many tachyon lines made it to the final line
        let all_moves = self.get_all_tachyon_lines();
        all_moves[all_moves.keys().max().unwrap()].values().sum()
    }

    fn get_all_tachyon_lines(&self) -> HashMap<usize, HashMap<Point2D, usize>> {
        let max = self.splitters.iter().map(|p| p.y).max().unwrap() as usize;
        let start = HashMap::from([(0, HashMap::from([(self.start, 1)]))]);
        (0..=max).fold(start, |mut acc, i| {
            acc.insert(i + 1, self.next_tachyon_line(&acc[&i]));
            acc
        })
    }

    fn next_tachyon_line(
        &self,
        beam_positions: &HashMap<Point2D, usize>,
    ) -> HashMap<Point2D, usize> {
        self.split_beams(&self.move_beams(beam_positions))
    }

    fn move_beams(&self, beam_positions: &HashMap<Point2D, usize>) -> HashMap<Point2D, usize> {
        // move down by 1 all values that aren't on splitters
        beam_positions
            .iter()
            .filter(|(key, _)| !self.splitters.contains(key))
            .map(|(key, value)| (*key + Point2D { x: 0, y: 1 }, *value))
            .collect::<HashMap<Point2D, usize>>()
    }

    fn split_beams(&self, beam_positions: &HashMap<Point2D, usize>) -> HashMap<Point2D, usize> {
        // add left/right all values that are on splitters
        beam_positions
            .iter()
            .filter(|(key, _)| self.splitters.contains(key))
            .fold(beam_positions.clone(), |mut acc, (splitter_pos, value)| {
                [
                    *splitter_pos + Point2D { x: -1, y: 0 },
                    *splitter_pos + Point2D { x: 1, y: 0 },
                ]
                .iter()
                .for_each(|split_pos| *(acc.entry(*split_pos).or_insert(0)) += value);
                acc
            })
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
        let expected = 40;
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
        let expected = 24743903847942;
        let actual = day7.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
