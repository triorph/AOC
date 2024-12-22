mod parser;
mod types;

use std::collections::HashSet;

use crate::parser::parse_data;
use aoc_helpers::{point2d::Point2D, read_input_file, AOCCalculator, AOCFileOrParseError};
use types::{Direction, RobotMap, WideTile};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day15 {
    robot_map: RobotMap,
    directions: Vec<Direction>,
}

impl AOCCalculator for Day15 {
    fn new(filename: &str) -> Result<Day15, AOCFileOrParseError> {
        let (map_tiles, directions) = parse_data(&read_input_file(filename)?)?;
        Ok(Day15 {
            robot_map: RobotMap::from_maptiles(&map_tiles),
            directions,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day15 {
    fn try_move_point(
        &mut self,
        start: &WideTile,
        direction: &Point2D,
    ) -> (bool, HashSet<WideTile>) {
        let mut ret = HashSet::new();
        let target = start + direction;
        let boxes_overlapping = self
            .robot_map
            .boxes_overlapping(&target)
            .into_iter()
            .filter(|box_target| *box_target != *start)
            .collect::<HashSet<WideTile>>();
        let walls_overlapping = self.robot_map.walls_overlapping(&target).is_empty();
        if !walls_overlapping || boxes_overlapping.is_empty() {
            (walls_overlapping, HashSet::new())
        } else {
            for box_target in boxes_overlapping.iter() {
                let (found, moved) = self.try_move_point(box_target, direction);
                if !found {
                    return (false, HashSet::new());
                } else {
                    ret.insert(*box_target);
                    ret.extend(moved.into_iter());
                }
            }
            (true, ret)
        }
    }

    fn try_move_character(&mut self, direction: Direction) {
        let character = self.robot_map.character;
        let (moved, items_to_move) = self.try_move_point(&character, &direction.as_point());
        if moved {
            for item in items_to_move.iter() {
                self.robot_map.boxes.remove(item);
            }
            for item in items_to_move.iter() {
                self.robot_map.boxes.insert(item + &direction.as_point());
            }
            self.robot_map.character = &self.robot_map.character + &direction.as_point()
        }
    }

    fn calculate_score(&self) -> usize {
        self.robot_map
            .boxes
            .iter()
            .map(|point| point.start.x + point.start.y * 100)
            .sum::<isize>() as usize
    }

    fn calculate_day_a(&self) -> usize {
        let mut obj = self.clone();
        for direction in self.directions.iter() {
            obj.try_move_character(*direction);
        }
        obj.calculate_score()
    }

    fn calculate_day_b(&self) -> usize {
        let mut obj = self.clone();
        obj.robot_map = obj.robot_map.to_dayb_map();
        for direction in self.directions.iter() {
            obj.try_move_character(*direction);
        }
        obj.calculate_score()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a_large_example() {
        let day15 = Day15::new("data/test_data.txt").unwrap();
        let expected = 10092;
        let actual = day15.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_a_small_example() {
        let day15 = Day15::new("data/test_data_2.txt").unwrap();
        let expected = 2028;
        let actual = day15.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b_large() {
        let day15 = Day15::new("data/test_data.txt").unwrap();
        let expected = 9021;
        let actual = day15.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b_reddit() {
        // additional test case from reddit:
        // https://www.reddit.com/r/adventofcode/comments/1heoj7f/2024_day_15_part_2_more_sample_inputs_to_catch/
        //
        // turned out to be the problem, that if a boulder can be moved, I wasn't also
        // checking that a wall overlaps
        let day15 = Day15::new("data/test_data_3.txt").unwrap();
        let expected = 509;
        let actual = day15.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day15 = Day15::new("data/input_data.txt").unwrap();
        let expected = 1495147;
        let actual = day15.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day15 = Day15::new("data/input_data.txt").unwrap();
        let expected = 0;
        let actual = day15.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
