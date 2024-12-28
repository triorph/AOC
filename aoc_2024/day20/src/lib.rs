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
pub struct Day20 {
    data: Vec<Vec<MazeTile>>,
}

impl AOCCalculator for Day20 {
    fn new(filename: &str) -> Result<Day20, AOCFileOrParseError> {
        Ok(Day20 {
            data: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day20 {
    fn all_points(&self) -> Vec<Point2D> {
        (0..(self.data.len()))
            .cartesian_product(0..(self.data[0].len()))
            .map(|(y, x)| Point2D::from_usize(x, y))
            .collect()
    }

    fn within_bounds(&self, point: &Point2D) -> bool {
        (0..self.data.len() as isize).contains(&point.y)
            && (0..self.data[0].len() as isize).contains(&point.x)
    }

    fn get_at_point(&self, point: &Point2D) -> Option<MazeTile> {
        if self.within_bounds(point) {
            Some(self.data[point.y as usize][point.x as usize])
        } else {
            None
        }
    }

    fn build_walls(&self) -> HashSet<Point2D> {
        self.all_points()
            .into_iter()
            .filter(|point| self.get_at_point(point) == Some(MazeTile::Wall))
            .collect()
    }

    fn find_shortest_path(&self) -> Vec<Point2D> {
        // simple dijkstra style algorithm to find the shortest path
        let walls = self.build_walls();
        let start = self
            .all_points()
            .into_iter()
            .find(|point| self.get_at_point(point) == Some(MazeTile::Start))
            .expect("There is a start");
        let end = self
            .all_points()
            .into_iter()
            .find(|point| self.get_at_point(point) == Some(MazeTile::End))
            .expect("There is an end");
        let mut to_explore: BinaryHeap<Reverse<(usize, Vec<Point2D>)>> = BinaryHeap::new();
        to_explore.push(Reverse((1, vec![start])));
        let mut fastest_to_point: HashMap<Point2D, Vec<Point2D>> = HashMap::new();
        while let Some(Reverse((distance, path))) = to_explore.pop() {
            let current_point = path[path.len() - 1];
            if !fastest_to_point.contains_key(&current_point)
                || fastest_to_point
                    .get(&current_point)
                    .is_some_and(|fastest_distance| distance < fastest_distance.len())
            {
                fastest_to_point.insert(current_point, path.clone());
                if current_point == end {
                    continue;
                }
                for neighbour in current_point.get_neighbours().into_iter() {
                    if !walls.contains(&neighbour) {
                        let mut neighbour_path = path.clone();
                        neighbour_path.push(neighbour);
                        to_explore.push(Reverse((distance + 1, neighbour_path)));
                    }
                }
            }
        }
        fastest_to_point
            .get(&end)
            .expect("We find a shortest path")
            .clone()
    }

    fn explore_cheats(
        &self,
        shortest_path: &[Point2D],
        cheat_size: usize,
    ) -> Vec<(usize, usize, usize)> {
        // assume that the cheats will start and end on the shortest path
        // (does this assumption hold? AoC gives me the right results so not going to complain, but
        // I don't see any reason why it would have to be true)
        // so we can just search for all points further on the path and check that they're within
        // the cheat size by manhattan distance
        let mut cheats = Vec::new();
        for (start_index, start) in shortest_path[0..shortest_path.len() - 1].iter().enumerate() {
            for (j, end) in shortest_path[start_index + 1..shortest_path.len()]
                .iter()
                .enumerate()
            {
                let end_index = start_index + j + 1;
                if start.get_manhattan_distance(end) <= cheat_size
                    && end_index - start_index > start.get_manhattan_distance(end)
                {
                    cheats.push((
                        end_index - start_index - start.get_manhattan_distance(end),
                        end_index,
                        start_index,
                    ));
                }
            }
        }
        cheats
    }

    fn calculate_day_a(&self) -> usize {
        let shortest_path = self.find_shortest_path();
        let cheats = self.explore_cheats(&shortest_path, 2);
        cheats
            .into_iter()
            .filter(|(size, _, _)| *size >= 100)
            .count()
    }

    fn calculate_day_b(&self) -> usize {
        let shortest_path = self.find_shortest_path();
        let cheats = self.explore_cheats(&shortest_path, 20);
        cheats
            .into_iter()
            .filter(|(size, _, _)| *size >= 100)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day20 = Day20::new("data/test_data.txt").unwrap();
        let cheats = day20.explore_cheats(&day20.find_shortest_path(), 2);
        let mut cheat_frequency = HashMap::new();
        for cheat in cheats.into_iter() {
            *cheat_frequency.entry(cheat.0).or_insert(0) += 1;
        }
        assert_eq!(cheat_frequency.get(&2), Some(&14));
        assert_eq!(cheat_frequency.get(&4), Some(&14));
        assert_eq!(cheat_frequency.get(&6), Some(&2));
        assert_eq!(cheat_frequency.get(&8), Some(&4));
        assert_eq!(cheat_frequency.get(&10), Some(&2));
        assert_eq!(cheat_frequency.get(&12), Some(&3));
        assert_eq!(cheat_frequency.get(&20), Some(&1));
        assert_eq!(cheat_frequency.get(&36), Some(&1));
        assert_eq!(cheat_frequency.get(&38), Some(&1));
        assert_eq!(cheat_frequency.get(&40), Some(&1));
        assert_eq!(cheat_frequency.get(&64), Some(&1));
    }

    #[test]
    fn test_calculate_day_b() {
        let day20 = Day20::new("data/test_data.txt").unwrap();
        let cheats = day20.explore_cheats(&day20.find_shortest_path(), 20);
        let mut cheat_frequency = HashMap::new();
        for cheat in cheats.into_iter() {
            *cheat_frequency.entry(cheat.0).or_insert(0) += 1;
        }
        assert_eq!(cheat_frequency.get(&50), Some(&32));
        assert_eq!(cheat_frequency.get(&52), Some(&31));
        assert_eq!(cheat_frequency.get(&54), Some(&29));
        assert_eq!(cheat_frequency.get(&56), Some(&39));
        assert_eq!(cheat_frequency.get(&58), Some(&25));
        assert_eq!(cheat_frequency.get(&60), Some(&23));
        assert_eq!(cheat_frequency.get(&62), Some(&20));
        assert_eq!(cheat_frequency.get(&64), Some(&19));
        assert_eq!(cheat_frequency.get(&66), Some(&12));
        assert_eq!(cheat_frequency.get(&68), Some(&14));
        assert_eq!(cheat_frequency.get(&70), Some(&12));
        assert_eq!(cheat_frequency.get(&72), Some(&22));
        assert_eq!(cheat_frequency.get(&74), Some(&4));
        assert_eq!(cheat_frequency.get(&76), Some(&3));
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day20 = Day20::new("data/input_data.txt").unwrap();
        let expected = 1452;
        let actual = day20.calculate_day_a();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day20 = Day20::new("data/input_data.txt").unwrap();
        let expected = 999556;
        let actual = day20.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
