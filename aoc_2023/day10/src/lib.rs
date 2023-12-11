mod parser;
mod pipe;
mod point;
use crate::parser::parse_data;
use crate::pipe::Pipe;
use crate::point::Neighbours;
use aoc_helpers::point2d::Point2D;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

pub struct Day10 {
    map: Vec<Vec<Pipe>>,
}

impl AOCCalculator for Day10 {
    fn new(filename: &str) -> Result<Day10, AOCFileOrParseError> {
        Ok(Day10 {
            map: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day10 {
    fn calculate_day_a(&self) -> usize {
        *self.get_cycle_distances().values().max().unwrap_or(&0)
    }

    fn calculate_day_b(&self) -> usize {
        let cycle_points = self.calculate_cycle_on_tiles();
        let mut valid_points: HashSet<Point2D> = HashSet::new();
        let mut explored_points = HashSet::new();
        for point in self.iter_all_tile_points() {
            let (is_valid, found) =
                self.explore_inside_cycle_tiles(&point, &mut explored_points, &cycle_points);
            if is_valid {
                valid_points.extend(found)
            }
        }
        valid_points
            .iter()
            .filter(|p| p.x % 3 == 1 && p.y % 3 == 1) // only count centre points in each 3x3 tile
            .count()
    }

    fn find_starter_point(&self) -> Point2D {
        for point in self.iter_all_points() {
            if self.get_at_point(&point) == Some(&Pipe::Start) {
                return point;
            }
        }
        panic!("No starter found")
    }

    fn iter_all_points(&self) -> Box<dyn Iterator<Item = Point2D>> {
        Box::new(
            (0..self.map.len())
                .cartesian_product(0..self.map[0].len())
                .map(|(y, x)| Point2D {
                    x: x as isize,
                    y: y as isize,
                }),
        )
    }

    fn get_cycle_distances(&self) -> HashMap<Point2D, usize> {
        let mut distance_along_cycle: HashMap<Point2D, usize> = HashMap::new();
        // use a non-recursive BFS search to stop a stack depth being reached
        let mut to_explore = VecDeque::new();
        to_explore.push_back((self.find_starter_point(), 0));
        while let Some((point, distance)) = to_explore.pop_front() {
            if !self.point_within_bounds(&point) {
                continue;
            }
            let current_minimum = distance_along_cycle.get(&point);
            if current_minimum.is_none() || current_minimum.unwrap() > &distance {
                distance_along_cycle.insert(point, distance);
                for neighbour in self.neighbours(&point).into_iter() {
                    if self.neighbours(&neighbour).contains(&point) {
                        // for the Start this check is necessary
                        to_explore.push_back((neighbour, distance + 1))
                    }
                }
            }
        }
        distance_along_cycle
    }

    fn iter_all_tile_points(&self) -> Box<dyn Iterator<Item = Point2D>> {
        Box::new(
            (0..(self.map.len() * 3))
                .cartesian_product(0..(self.map[0].len() * 3))
                .map(|(y, x)| Point2D {
                    x: x as isize,
                    y: y as isize,
                }),
        )
    }

    fn point_within_bounds(&self, point: &Point2D) -> bool {
        (0..self.map.len()).contains(&(point.y as usize))
            && (0..self.map[point.y as usize].len()).contains(&(point.x as usize))
    }

    fn point_within_tile_bounds(&self, point: &Point2D) -> bool {
        (0..(self.map.len() * 3)).contains(&(point.y as usize))
            && (0..(self.map[point.y as usize / 3].len() * 3)).contains(&(point.x as usize))
    }

    fn get_at_point(&self, point: &Point2D) -> Option<&Pipe> {
        if self.point_within_bounds(point) {
            Some(&self.map[point.y as usize][point.x as usize])
        } else {
            None
        }
    }

    fn neighbours(&self, point: &Point2D) -> Vec<Point2D> {
        if let Some(pipe) = self.get_at_point(point) {
            point.neighbours(pipe)
        } else {
            Vec::new()
        }
    }

    #[allow(dead_code)]
    fn print_part_b(
        &self,
        invalid_points: &HashSet<Point2D>,
        found_points: &HashSet<Point2D>,
    ) -> String {
        let mut ret = "".to_string();
        for y in 0..(self.map.len() * 3) {
            for x in 0..(self.map[y / 3].len() * 3) {
                let point = Point2D {
                    x: x as isize,
                    y: y as isize,
                };
                if found_points.contains(&point) {
                    ret += ".";
                } else if invalid_points.contains(&point) {
                    ret += "#";
                } else {
                    ret += " ";
                }
            }
            ret += "\n";
        }
        ret
    }

    #[allow(dead_code)]
    fn print_point_minimums(&self, point_minimums: &HashMap<Point2D, usize>) -> String {
        let mut ret = "".to_string();
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                match point_minimums.get(&Point2D {
                    x: x as isize,
                    y: y as isize,
                }) {
                    Some(x) if x == &0 => ret += "S",
                    Some(x) if x == &6833 => ret += "D",
                    Some(x) => ret += &(x / 690).to_string(),
                    None => ret += ".",
                };
            }
            ret += "\n";
        }
        ret
    }

    fn calculate_cycle_on_tiles(&self) -> HashSet<Point2D> {
        let original_cycle = self.get_cycle_distances();
        // create a 3x3 tile out of each point
        let mut ret = HashSet::new();
        for point in original_cycle.keys() {
            let tile_start = Point2D {
                x: point.x * 3,
                y: point.y * 3,
            };
            for tripled in self
                .get_at_point(point)
                .unwrap_or(&Pipe::Empty)
                .all_places_when_tiled()
                .into_iter()
            {
                ret.insert(tripled + tile_start);
            }
        }
        ret
    }

    fn explore_inside_cycle_tiles(
        &self,
        start: &Point2D,
        explored_points: &mut HashSet<Point2D>,
        points_of_cycle: &HashSet<Point2D>,
    ) -> (bool, HashSet<Point2D>) {
        let mut to_explore = VecDeque::new();
        to_explore.push_back(*start);
        let mut found_points = HashSet::new();
        let mut is_valid = true;
        while let Some(point) = to_explore.pop_front() {
            if explored_points.contains(&point) || points_of_cycle.contains(&point) {
                continue;
            } else if !self.point_within_tile_bounds(&point) {
                is_valid = false;
                continue;
            } else {
                found_points.insert(point);
                explored_points.insert(point);
                for neighbour in point.all_neighbours().into_iter() {
                    to_explore.push_back(neighbour);
                }
            }
        }
        (is_valid, found_points)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day10 = Day10::new("data/test_data.txt").unwrap();
        let expected = 8;
        let actual = day10.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day10 = Day10::new("data/test_data_b.txt").unwrap();
        let expected = 10;
        let actual = day10.calculate_day_b();
        assert_eq!(expected, actual);
        let day10 = Day10::new("data/test_data_c.txt").unwrap();
        let expected = 8;
        let actual = day10.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day10 = Day10::new("data/input_data.txt").unwrap();
        let expected = 6838;
        let actual = day10.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day10 = Day10::new("data/input_data.txt").unwrap();
        let expected = 451;
        let actual = day10.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
