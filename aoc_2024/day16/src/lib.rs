mod parser;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use crate::parser::parse_data;
use aoc_helpers::{
    point2d::{Neighbours, Point2D},
    read_input_file, AOCCalculator, AOCFileOrParseError,
};
use itertools::Itertools;
use parser::MazeTile;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day16 {
    maze: Vec<Vec<MazeTile>>,
    start: Point2D,
    end: Point2D,
}

impl AOCCalculator for Day16 {
    fn new(filename: &str) -> Result<Day16, AOCFileOrParseError> {
        let maze = parse_data(&read_input_file(filename)?)?;
        let start_end: Vec<Point2D> = (0..maze.len())
            .cartesian_product(0..maze[0].len())
            .filter(|(y, x)| maze[*y][*x] == MazeTile::Start || maze[*y][*x] == MazeTile::End)
            .map(|(y, x)| Point2D::from_usize(x, y))
            .collect();
        let start = start_end
            .iter()
            .find(|p| maze[p.y as usize][p.x as usize] == MazeTile::Start);
        let end = start_end
            .iter()
            .find(|p| maze[p.y as usize][p.x as usize] == MazeTile::End);

        match (start, end) {
            (Some(start), Some(end)) => Ok(Day16 {
                maze,
                start: *start,
                end: *end,
            }),
            _ => Err(AOCFileOrParseError),
        }
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day16 {
    fn can_move(&self, point: &Point2D) -> bool {
        (0..(self.maze.len() as isize)).contains(&point.y)
            && (0..(self.maze[0].len() as isize)).contains(&point.x)
            && [MazeTile::Room, MazeTile::End]
                .contains(&self.maze[point.y as usize][point.x as usize])
    }

    fn explore(&self) -> (usize, usize) {
        let mut exploration_heap = BinaryHeap::new();
        exploration_heap.push(Reverse((0, vec![self.start], Point2D::from_usize(1, 0))));
        let mut best_score: HashMap<Point2D, usize> = HashMap::new();
        let mut nodes_on_best_path: HashSet<Point2D> = HashSet::new();
        while let Some(Reverse((score, path, direction))) = exploration_heap.pop() {
            let location = path[path.len() - 1];
            let this_best_score = best_score.get(&location);
            if this_best_score.is_some_and(|best_score| score <= *best_score)
                || this_best_score.is_none()
            {
                if !best_score.contains_key(&location)
                    || best_score
                        .get(&location)
                        .is_some_and(|best_score| score < *best_score)
                {
                    best_score.insert(location, score);
                }

                for neighbour in location.get_neighbours().into_iter() {
                    if path.contains(&neighbour) {
                        continue;
                    }
                    if neighbour == location + direction && self.can_move(&neighbour) {
                        let mut new_path = path.clone();
                        new_path.push(neighbour);
                        exploration_heap.push(Reverse((score + 1, new_path, neighbour - location)));
                    } else if neighbour != location - direction && self.can_move(&neighbour) {
                        let mut new_path = path.clone();
                        new_path.push(neighbour);
                        exploration_heap.push(Reverse((
                            score + 1001,
                            new_path,
                            neighbour - location,
                        )));
                    }
                }
            }
        }
        exploration_heap.push(Reverse((0, vec![self.start], Point2D::from_usize(1, 0))));

        while let Some(Reverse((score, path, direction))) = exploration_heap.pop() {
            let location = path[path.len() - 1];
            let this_best_score = best_score.get(&location);
            if this_best_score.is_some_and(|best_score| score <= *best_score + 1000)
                || this_best_score.is_none()
            {
                if location == self.end {
                    let best_end_score = best_score
                        .iter()
                        .filter(|(k, _)| **k == self.end)
                        .map(|(_, v)| v)
                        .min();
                    if best_end_score.is_some_and(|best_end_score| score < *best_end_score) {
                        nodes_on_best_path.clear();
                    }
                    if best_end_score.is_some_and(|best_end_score| score <= *best_end_score)
                        || best_end_score.is_none()
                    {
                        nodes_on_best_path.extend(path.iter().clone());
                    }
                }
                for neighbour in location.get_neighbours().into_iter() {
                    if path.contains(&neighbour) {
                        continue;
                    }
                    if neighbour == location + direction && self.can_move(&neighbour) {
                        let mut new_path = path.clone();
                        new_path.push(neighbour);
                        exploration_heap.push(Reverse((score + 1, new_path, neighbour - location)));
                    } else if neighbour != location - direction && self.can_move(&neighbour) {
                        let mut new_path = path.clone();
                        new_path.push(neighbour);
                        exploration_heap.push(Reverse((
                            score + 1001,
                            new_path,
                            neighbour - location,
                        )));
                    }
                }
            }
        }
        (
            *best_score
                .iter()
                .find(|(k, _)| **k == self.end)
                .expect("Should get an answer")
                .1,
            nodes_on_best_path.len(),
        )
    }

    fn calculate_day_a(&self) -> usize {
        self.explore().0
    }

    fn calculate_day_b(&self) -> usize {
        self.explore().1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day16 = Day16::new("data/test_data.txt").unwrap();
        let expected = 7036;
        let actual = day16.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_a_second_example() {
        let day16 = Day16::new("data/test_data_2.txt").unwrap();
        let expected = 11048;
        let actual = day16.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day16 = Day16::new("data/test_data.txt").unwrap();
        let expected = 45;
        let actual = day16.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b_second_example() {
        let day16 = Day16::new("data/test_data_2.txt").unwrap();
        let expected = 64;
        let actual = day16.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day16 = Day16::new("data/input_data.txt").unwrap();
        let expected = 91464;
        let actual = day16.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day16 = Day16::new("data/input_data.txt").unwrap();
        let expected = 494;
        let actual = day16.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
