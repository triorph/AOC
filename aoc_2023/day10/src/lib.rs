mod parser;
mod pipe;
mod point;
use crate::parser::parse_data;
use crate::pipe::Pipe;
use crate::point::Point;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};
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
    fn find_starter_point(&self) -> Point {
        for (y, line) in self.map.iter().enumerate() {
            for (x, value) in line.iter().enumerate() {
                if value == &Pipe::Start {
                    return Point {
                        x: x as isize,
                        y: y as isize,
                    };
                }
            }
        }
        panic!("No starter found")
    }

    fn point_within_bounds(&self, point: &Point) -> bool {
        (0..self.map.len()).contains(&(point.y as usize))
            && (0..self.map[point.y as usize].len()).contains(&(point.x as usize))
    }

    fn point_within_bounds_after_tripled(&self, point: &Point) -> bool {
        (0..(self.map.len() * 3)).contains(&(point.y as usize))
            && (0..(self.map[point.y as usize / 3].len() * 3)).contains(&(point.x as usize))
    }

    fn get_at_point(&self, point: &Point) -> Option<&Pipe> {
        if self.point_within_bounds(point) {
            Some(&self.map[point.y as usize][point.x as usize])
        } else {
            None
        }
    }

    fn neighbours(&self, point: &Point) -> Vec<Point> {
        if let Some(pipe) = self.get_at_point(point) {
            point.neighbours(pipe)
        } else {
            Vec::new()
        }
    }

    fn explore(&self, distance: usize, point: &Point) -> HashMap<Point, usize> {
        let mut point_minimums: HashMap<Point, usize> = HashMap::new();
        // use a BFS search to stop a stack depth being reached
        let mut to_explore = VecDeque::new();
        to_explore.push_back((*point, distance));
        while let Some((point, distance)) = to_explore.pop_front() {
            if !self.point_within_bounds(&point) {
                continue;
            }
            let current_minimum = point_minimums.get(&point);
            if current_minimum.is_none() || current_minimum.unwrap() > &distance {
                point_minimums.insert(point.clone(), distance);
                for neighbour in self.neighbours(&point).into_iter() {
                    if self.neighbours(&neighbour).contains(&point) {
                        to_explore.push_back((neighbour, distance + 1))
                    }
                }
            }
        }
        point_minimums
    }

    fn print_part_b(
        &self,
        invalid_points: &HashSet<Point>,
        found_points: &HashSet<Point>,
    ) -> String {
        let mut ret = "".to_string();
        for y in 0..(self.map.len() * 3) {
            for x in 0..(self.map[y / 3].len() * 3) {
                let point = Point {
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

    fn print_point_minimums(&self, point_minimums: &HashMap<Point, usize>) -> String {
        let mut ret = "".to_string();
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                match point_minimums.get(&Point {
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

    fn calculate_day_a(&self) -> usize {
        let starter = self.find_starter_point();
        let point_minimums = self.explore(0, &starter);
        *point_minimums.values().max().unwrap_or(&0) as usize
    }

    fn get_cycle_points_when_tripled(
        &self,
        point_minimums: HashMap<Point, usize>,
    ) -> HashSet<Point> {
        let mut ret = HashSet::new();
        for point in point_minimums.keys() {
            let starter = Point {
                x: point.x * 3,
                y: point.y * 3,
            };
            for tripled in self
                .get_at_point(&point)
                .unwrap_or(&Pipe::Empty)
                .invalid_when_tripled()
                .into_iter()
            {
                ret.insert(tripled + starter);
            }
        }
        ret
    }

    fn calculate_day_b(&self) -> usize {
        let starter = self.find_starter_point();
        let point_minimums = self.explore(0, &starter);
        let points_of_cycle = self.get_cycle_points_when_tripled(point_minimums);
        let mut valid_points: HashSet<Point> = HashSet::new();
        let mut explored_points = HashSet::new();
        for y in 1..((self.map.len() - 1) * 3) {
            for x in 1..((self.map[y / 3].len() - 1) * 3) {
                let mut to_explore = VecDeque::new();
                to_explore.push_back(Point {
                    x: x as isize,
                    y: y as isize,
                });
                let mut found_points = HashSet::new();
                let mut is_valid = true;
                while let Some(point) = to_explore.pop_front() {
                    if explored_points.contains(&point) || points_of_cycle.contains(&point) {
                        continue;
                    } else if !self.point_within_bounds_after_tripled(&point) {
                        is_valid = false;
                        continue;
                    } else {
                        found_points.insert(point);
                        explored_points.insert(point);
                        for neighbour in point.neighbours(&Pipe::Start).into_iter() {
                            to_explore.push_back(neighbour);
                        }
                    }
                }
                explored_points.extend(found_points.clone());
                if found_points.len() > 0 {
                    if is_valid {
                        valid_points.extend(found_points)
                    }
                }
            }
        }
        valid_points
            .iter()
            .filter(|p| p.x % 3 == 1 && p.y % 3 == 1)
            .count()
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
