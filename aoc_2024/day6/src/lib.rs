mod parser;
use std::collections::HashSet;

use crate::parser::parse_data;
use aoc_helpers::{point2d::Point2D, read_input_file, AOCCalculator, AOCFileOrParseError};
use itertools::Itertools;
use parser::MapTile;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day6 {
    map: Vec<Vec<MapTile>>,
}

impl AOCCalculator for Day6 {
    fn new(filename: &str) -> Result<Day6, AOCFileOrParseError> {
        Ok(Day6 {
            map: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day6 {
    fn get_player_and_rocks(&self) -> (Option<Point2D>, HashSet<Point2D>) {
        (0..self.map.len())
            .cartesian_product(0..self.map[0].len())
            .fold(
                (None, HashSet::new()),
                |(player, mut rocks), (y, x)| match self.map[y][x] {
                    MapTile::Rock => {
                        rocks.insert(Point2D::from_usize(x, y));
                        (player, rocks)
                    }
                    MapTile::Player => (Some(Point2D::from_usize(x, y)), rocks),
                    _ => (player, rocks),
                },
            )
    }

    fn rotate_direction(&self, direction: &Point2D) -> Point2D {
        Point2D {
            x: -direction.y,
            y: direction.x,
        }
    }

    fn within_bounds(&self, player: &Point2D) -> bool {
        (0..self.map.len() as isize).contains(&player.y)
            && (0..self.map[0].len() as isize).contains(&player.x)
    }

    fn get_guard_positions(&self) -> HashSet<Point2D> {
        let (player, rocks) = self.get_player_and_rocks();
        let mut player = player.expect("Must have a player start position");
        let mut direction = Point2D { x: 0, y: -1 };
        let mut visited_points = HashSet::new();
        visited_points.insert(player);
        while self.within_bounds(&player) {
            let next_pos = player + direction;
            if rocks.contains(&next_pos) {
                direction = self.rotate_direction(&direction);
            } else {
                player = next_pos;
                visited_points.insert(player);
            }
        }
        visited_points
    }

    fn calculate_day_a(&self) -> usize {
        self.get_guard_positions().len() - 1 // remove the out of bounds entry
    }

    fn check_for_loop(
        &self,
        player: &Point2D,
        rocks: &HashSet<Point2D>,
        new_rock: &Point2D,
    ) -> bool {
        let mut rocks = rocks.clone();
        rocks.insert(*new_rock);
        let mut player = *player;
        let mut direction = Point2D { x: 0, y: -1 };
        let mut visited_points = HashSet::new();
        visited_points.insert((player, direction));
        while self.within_bounds(&player) {
            let next_pos = player + direction;
            if rocks.contains(&next_pos) {
                direction = self.rotate_direction(&direction);
            } else {
                player = next_pos;
                if visited_points.contains(&(player, direction)) {
                    return true;
                } else {
                    visited_points.insert((player, direction));
                }
            }
        }
        false
    }

    fn calculate_day_b(&self) -> usize {
        let (player, rocks) = self.get_player_and_rocks();
        let player = player.expect("Must have a player start position");
        self.get_guard_positions()
            .iter()
            .filter(|new_rock| {
                **new_rock != player
                    && !rocks.contains(new_rock)
                    && self.check_for_loop(&player, &rocks, new_rock)
            })
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let skeleton = Day6::new("data/test_data.txt").unwrap();
        let expected = 41;
        let actual = skeleton.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let skeleton = Day6::new("data/test_data.txt").unwrap();
        let expected = 6;
        let actual = skeleton.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let skeleton = Day6::new("data/input_data.txt").unwrap();
        let expected = 4647;
        let actual = skeleton.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let skeleton = Day6::new("data/input_data.txt").unwrap();
        let expected = 1723;
        let actual = skeleton.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
