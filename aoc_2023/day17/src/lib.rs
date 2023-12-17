mod parser;
use std::collections::{HashMap, VecDeque};

use crate::parser::parse_data;
use aoc_helpers::{point2d::Point2D, read_input_file, AOCCalculator, AOCFileOrParseError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day17 {
    lava_pool: Vec<Vec<usize>>,
}

impl AOCCalculator for Day17 {
    fn new(filename: &str) -> Result<Day17, AOCFileOrParseError> {
        Ok(Day17 {
            lava_pool: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

trait Neighbours {
    fn get_neighbours(&self) -> Vec<Point2D>;
}

impl Neighbours for Point2D {
    fn get_neighbours(&self) -> Vec<Point2D> {
        vec![
            Point2D { x: 0, y: -1 } + *self,
            Point2D { x: 1, y: 0 } + *self,
            Point2D { x: 0, y: 1 } + *self,
            Point2D { x: -1, y: 0 } + *self,
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
struct Crucible {
    dir: Direction,
    length: usize,
    min: usize,
    max: usize,
}

impl Direction {
    fn as_point_delta(&self) -> Point2D {
        match self {
            Direction::Up => Point2D { x: 0, y: -1 },
            Direction::Left => Point2D { x: -1, y: 0 },
            Direction::Right => Point2D { x: 1, y: 0 },
            Direction::Down => Point2D { x: 0, y: 1 },
        }
    }
}

impl Crucible {
    fn init(dir: Direction, min: usize, max: usize) -> Crucible {
        Crucible {
            dir,
            length: 1,
            min,
            max,
        }
    }

    fn get_neighbours(&self) -> Vec<Crucible> {
        let mut ret = Vec::new();
        if self.length < self.max {
            ret.push(Crucible {
                dir: self.dir,
                length: self.length + 1,
                min: self.min,
                max: self.max,
            })
        }
        if self.length >= self.min {
            ret.extend(match self.dir {
                Direction::Up => vec![
                    Crucible::init(Direction::Left, self.min, self.max),
                    Crucible::init(Direction::Right, self.min, self.max),
                ],
                Direction::Left => vec![
                    Crucible::init(Direction::Up, self.min, self.max),
                    Crucible::init(Direction::Down, self.min, self.max),
                ],
                Direction::Right => vec![
                    Crucible::init(Direction::Up, self.min, self.max),
                    Crucible::init(Direction::Down, self.min, self.max),
                ],
                Direction::Down => vec![
                    Crucible::init(Direction::Left, self.min, self.max),
                    Crucible::init(Direction::Right, self.min, self.max),
                ],
            });
        }
        ret
    }
}

impl Day17 {
    fn within_bounds(&self, point: &Point2D) -> bool {
        (0..self.lava_pool.len() as isize).contains(&point.y)
            && (0..self.lava_pool[0].len() as isize).contains(&point.x)
    }

    fn value_at_point(&self, point: &Point2D) -> Option<usize> {
        if self.within_bounds(point) {
            Some(self.lava_pool[point.y as usize][point.x as usize])
        } else {
            None
        }
    }

    fn find_shortest_path(&self, starting_crucible: Crucible) -> usize {
        let mut current_shortest: HashMap<(Point2D, Crucible), usize> = HashMap::new();
        let mut paths = VecDeque::new();
        let first_path = (Point2D { x: 0, y: 0 }, 0, starting_crucible);
        paths.push_back(first_path);
        while let Some((point, distance, direction)) = paths.pop_front() {
            if let Some(prev_distance) = current_shortest.get(&(point, direction)) {
                if prev_distance <= &distance {
                    continue;
                }
            }
            current_shortest.insert((point, direction), distance);
            for new_direction in direction.get_neighbours().into_iter() {
                let neighbour = point + new_direction.dir.as_point_delta();
                if let Some(distance_increase) = self.value_at_point(&neighbour) {
                    paths.push_back((neighbour, distance + distance_increase, new_direction));
                }
            }
        }

        *current_shortest
            .iter()
            .filter(|((point, crucible), _)| {
                crucible.length >= crucible.min
                    && point.x == self.lava_pool[self.lava_pool.len() - 1].len() as isize - 1
                    && point.y == self.lava_pool.len() as isize - 1
            })
            .map(|(_, value)| value)
            .min()
            .unwrap()
    }

    fn calculate_day_a(&self) -> usize {
        self.find_shortest_path(Crucible {
            dir: Direction::Right,
            length: 0,
            min: 0,
            max: 3,
        })
    }

    fn calculate_day_b(&self) -> usize {
        self.find_shortest_path(Crucible {
            dir: Direction::Right,
            length: 0,
            min: 4,
            max: 10,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day17 = Day17::new("data/test_data.txt").unwrap();
        let expected = 102;
        let actual = day17.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day17 = Day17::new("data/test_data.txt").unwrap();
        let expected = 94;
        let actual = day17.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b_example_2() {
        let day17 = Day17::new("data/test_data_b.txt").unwrap();
        let expected = 71;
        let actual = day17.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day17 = Day17::new("data/input_data.txt").unwrap();
        let expected = 861;
        let actual = day17.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day17 = Day17::new("data/input_data.txt").unwrap();
        let expected = 0;
        let actual = day17.calculate_day_b();
        assert!(
            actual > 1008,
            "Your answer of {} is not greater than 1008, which was said to be too low",
            actual
        );
        assert!(
            actual < 1066,
            "Your answer of {} is not less than 1066, which was said to be too high",
            actual
        );
        assert_eq!(expected, actual);
    }
}
