mod map_point;
mod parser;
use std::collections::VecDeque;

use crate::map_point::{MapPoint, Point};
use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};

#[derive(Clone)]
pub struct Day12 {
    map_points: Vec<Vec<MapPoint>>,
    best_effort: Vec<Vec<Option<usize>>>,
}

impl AOCCalculator for Day12 {
    fn new(filename: &str) -> Result<Day12, AOCFileOrParseError> {
        let map_points = parse_data(&read_input_file(filename)?)?;
        let best_effort: Vec<Vec<Option<usize>>> = map_points
            .iter()
            .map(|v| v.iter().map(|_| None).collect())
            .collect();
        Ok(Day12 {
            map_points,
            best_effort,
        })
    }

    fn print_results(&self, name: &str) {
        let mut obj = self.clone();
        println!("{}a answer is {:?}", name, obj.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day12 {
    fn calculate_day_a(&mut self) -> usize {
        self.find_end(&self.find_starter_point())
            .expect("A real answer")
    }

    fn find_end(&mut self, starting_point: &Point) -> Option<usize> {
        let mut points: VecDeque<(Point, usize)> =
            VecDeque::from(vec![(starting_point.clone(), 0)]);
        let mut current_best: Option<usize> = None;
        while let Some((point, new_best)) = points.pop_front() {
            if self.check_and_set_current_value(&point, new_best) {
                if let Some(val) = self.get_value_at_point(&point) {
                    if val == MapPoint::End {
                        if let Some(best) = current_best {
                            if new_best < best {
                                current_best = Some(new_best);
                            }
                        } else {
                            current_best = Some(new_best);
                        }
                        continue;
                    }
                }
                for neighbour in self.find_valid_neighbours(&point) {
                    points.push_back((neighbour.clone(), new_best + 1))
                }
            }
        }
        current_best
    }

    fn get_value_at_point(&self, point: &Point) -> Option<MapPoint> {
        if (0..(self.map_points.len())).contains(&point.y)
            && (0..(self.map_points[point.y].len())).contains(&point.x)
        {
            Some(self.map_points[point.y][point.x].clone())
        } else {
            None
        }
    }

    fn find_valid_neighbours(&self, current: &Point) -> Vec<Point> {
        let current_value = self
            .get_value_at_point(current)
            .expect("given path must be on valid lines");
        let mut next_neighbours: Vec<Point> = Vec::new();
        for neighbour in current.get_neighbours().iter() {
            if let Some(val) = self.get_value_at_point(neighbour) {
                if val.get_value() <= current_value.get_value() + 1 {
                    next_neighbours.push(neighbour.clone());
                }
            }
        }
        next_neighbours
    }

    fn find_starter_point(&self) -> Point {
        for (y, x_axis) in self.map_points.iter().enumerate() {
            for (x, val) in x_axis.iter().enumerate() {
                if val == &MapPoint::Start {
                    return Point { x, y };
                }
            }
        }
        panic!("Did not find a point");
    }

    fn check_and_set_current_value(&mut self, point: &Point, new_val: usize) -> bool {
        let old = self.best_effort[point.y][point.x];
        match (old, new_val) {
            (None, new_val) => {
                self.best_effort[point.y][point.x] = Some(new_val);
                true
            }
            (Some(old_val), new_val) if new_val < old_val => {
                self.best_effort[point.y][point.x] = Some(new_val);
                true
            }
            _ => false,
        }
    }

    fn calculate_day_b(&self) -> usize {
        self.find_all_points_at_0()
            .iter()
            .flat_map(|starting_point| {
                let mut obj = self.clone();
                obj.find_end(starting_point)
            })
            .min()
            .unwrap()
    }

    fn find_all_points_at_0(&self) -> Vec<Point> {
        let mut ret = Vec::new();
        for y in 0..self.map_points.len() {
            for x in 0..self.map_points[y].len() {
                let p = Point { x, y };
                if let Some(MapPoint::Other(0)) = self.get_value_at_point(&p) {
                    ret.push(p);
                };
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let mut day12 = Day12::new("data/test_data.txt").unwrap();
        let expected = 31;
        let actual = day12.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_data() {
        let mut day12 = Day12::new("data/input_data.txt").unwrap();
        let actual = day12.calculate_day_a();
        // Worked out by borrowing someone else's code from reddit to check.
        // Note: First person I borrowed from had the same 470 that I do, so something with my
        // input data causes a common gotcha.
        assert_eq!(actual, 468);
    }

    #[test]
    fn test_calculate_day_b() {
        let day12 = Day12::new("data/test_data.txt").unwrap();
        let expected = 29;
        let actual = day12.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
