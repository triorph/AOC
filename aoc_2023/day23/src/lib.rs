mod parser;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

use crate::parser::parse_data;
use aoc_helpers::{point2d::Point2D, read_input_file, AOCCalculator, AOCFileOrParseError};
mod map_tile;
use crate::map_tile::MapTile;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day23 {
    map: Vec<Vec<MapTile>>,
}

impl AOCCalculator for Day23 {
    fn new(filename: &str) -> Result<Day23, AOCFileOrParseError> {
        Ok(Day23 {
            map: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day23 {
    fn find_start_tile(&self) -> Point2D {
        for (x, value) in self.map[0].iter().enumerate() {
            if value == &MapTile::Path {
                return Point2D::from_usize(x, 0);
            }
        }
        panic!("Must have a start tile");
    }

    fn find_end_tile(&self) -> Point2D {
        for (x, value) in self.map[self.map.len() - 1].iter().enumerate() {
            if value == &MapTile::Path {
                return Point2D::from_usize(x, self.map.len() - 1);
            }
        }
        panic!("Must have a start tile");
    }

    fn get_all_nodes_a(&self) -> Vec<Point2D> {
        let mut ret = vec![self.find_start_tile(), self.find_end_tile()];
        ret.extend(
            (0..self.map[0].len())
                .cartesian_product(0..self.map.len())
                .filter(|(x, y)| self.map[*y][*x].is_slope_tile())
                .map(|(x, y)| Point2D::from_usize(x, y)),
        );
        ret
    }

    fn get(&self, point: &Point2D) -> MapTile {
        self.map[point.y as usize][point.x as usize]
    }

    fn within_bounds(&self, point: &Point2D) -> bool {
        point.y >= 0
            && point.y < self.map.len() as isize
            && point.x >= 0
            && point.x < self.map[point.y as usize].len() as isize
    }

    fn explore_node_day_a(&self, node: &Point2D) -> HashMap<Point2D, usize> {
        let mut paths = VecDeque::new();
        let mut ret = HashMap::new();
        let end_tile = self.find_end_tile();
        paths.push_back(vec![*node]);
        while let Some(path) = paths.pop_front() {
            let last_in_path = path[path.len() - 1];
            for neighbour_delta in self.get(&last_in_path).get_neighbour_deltas().into_iter() {
                let neighbour = last_in_path + neighbour_delta;
                if !self.within_bounds(&neighbour) {
                    continue;
                }
                let neighbour_tile = self.get(&neighbour);
                if !path.contains(&neighbour) {
                    if neighbour_tile.is_slope_tile()
                        && !neighbour_tile
                            .get_neighbour_deltas()
                            .into_iter()
                            .map(|p| neighbour + p)
                            .collect::<Vec<Point2D>>()
                            .contains(&last_in_path)
                        || neighbour == end_tile
                    {
                        ret.entry(neighbour)
                            .and_modify(|old: &mut usize| *old = (*old).max(path.len()))
                            .or_insert(path.len());
                    } else if neighbour_tile.is_path() {
                        let mut next_path = path.clone();
                        next_path.push(neighbour);
                        paths.push_back(next_path);
                    }
                }
            }
        }
        ret
    }

    fn build_simplified_map_day_a(&self) -> HashMap<Point2D, HashMap<Point2D, usize>> {
        let nodes = self.get_all_nodes_a();
        let mut ret = HashMap::new();
        for node in nodes.iter() {
            ret.insert(*node, self.explore_node_day_a(node));
        }
        ret
    }

    fn find_longest_path_day_a(&self, map: &HashMap<Point2D, HashMap<Point2D, usize>>) -> usize {
        let mut ret = 0;
        let mut paths = VecDeque::new();
        let end_tile = self.find_end_tile();
        paths.push_back((self.find_start_tile(), 0));
        while let Some((tile, distance)) = paths.pop_front() {
            if tile == end_tile {
                ret = ret.max(distance);
            }
            for (next, delta) in map.get(&tile).unwrap().iter() {
                paths.push_back((*next, distance + delta))
            }
        }
        ret
    }

    fn calculate_day_a(&self) -> usize {
        let map = self.build_simplified_map_day_a();
        self.find_longest_path_day_a(&map)
    }

    fn explore_node_day_b(
        &self,
        node: &Point2D,
        possible_nodes: &[Point2D],
    ) -> HashMap<Point2D, usize> {
        let mut paths = VecDeque::new();
        let mut ret = HashMap::new();
        paths.push_back(vec![*node]);
        while let Some(path) = paths.pop_front() {
            let last_in_path = path[path.len() - 1];
            for neighbour_delta in MapTile::Path.get_neighbour_deltas().into_iter() {
                let neighbour = last_in_path + neighbour_delta;
                if !self.within_bounds(&neighbour) {
                    continue;
                }
                let neighbour_tile = self.get(&neighbour);
                if !path.contains(&neighbour) {
                    if possible_nodes.contains(&neighbour) {
                        ret.entry(neighbour)
                            .and_modify(|old: &mut usize| *old = (*old).max(path.len()))
                            .or_insert(path.len());
                    } else if neighbour_tile.is_path_or_slope_tile() {
                        let mut next_path = path.clone();
                        next_path.push(neighbour);
                        paths.push_back(next_path);
                    }
                }
            }
        }
        ret
    }

    fn is_day_b_node(&self, node: &Point2D) -> bool {
        self.get(node).is_path_or_slope_tile()
            && MapTile::Path
                .get_neighbour_deltas()
                .iter()
                .map(|n| n + node)
                .filter(|n| self.within_bounds(n) && self.get(n).is_path_or_slope_tile())
                .count()
                > 2
    }

    fn get_all_nodes_b(&self) -> Vec<Point2D> {
        let mut ret = vec![self.find_start_tile(), self.find_end_tile()];
        ret.extend(
            (0..self.map[0].len())
                .cartesian_product(0..self.map.len())
                .map(|(x, y)| Point2D::from_usize(x, y))
                .filter(|node| self.is_day_b_node(node)),
        );
        ret
    }

    #[allow(dead_code)]
    fn print_map_with_nodes(&self, nodes: &[Point2D]) -> String {
        let mut ret = String::new();
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                if nodes.contains(&Point2D::from_usize(x, y)) {
                    ret += "X";
                } else if self.get(&Point2D::from_usize(x, y)).is_path_or_slope_tile() {
                    ret += " ";
                } else {
                    ret += "#";
                }
            }
            ret += "\n";
        }
        ret
    }

    fn build_simplified_map_day_b(&self) -> HashMap<Point2D, HashMap<Point2D, usize>> {
        let nodes = self.get_all_nodes_b();
        let mut ret = HashMap::new();
        for node in nodes.iter() {
            ret.insert(*node, self.explore_node_day_b(node, &nodes));
        }
        ret
    }

    fn find_longest_path_day_b(&self, map: &HashMap<Point2D, HashMap<Point2D, usize>>) -> usize {
        let mut paths = VecDeque::new();
        let end_tile = self.find_end_tile();
        paths.push_back((vec![self.find_start_tile()], 0));
        let mut seen: HashMap<Point2D, usize> = HashMap::new();
        while let Some((path, distance)) = paths.pop_front() {
            let tile = path[path.len() - 1];
            seen.entry(tile)
                .and_modify(|old: &mut usize| *old = (*old).max(distance))
                .or_insert(distance);
            if tile == end_tile {
                continue;
            }
            for (next, delta) in map.get(&tile).unwrap().iter() {
                if !path.contains(next) {
                    let mut next_path = path.clone();
                    next_path.push(*next);
                    paths.push_back((next_path, distance + delta));
                }
            }
        }
        *seen.get(&end_tile).unwrap_or(&0)
    }

    fn calculate_day_b(&self) -> usize {
        let map = self.build_simplified_map_day_b();
        self.find_longest_path_day_b(&map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day23 = Day23::new("data/test_data.txt").unwrap();
        let expected = 94;
        let actual = day23.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day23 = Day23::new("data/test_data.txt").unwrap();
        let expected = 154;
        let actual = day23.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day23 = Day23::new("data/input_data.txt").unwrap();
        let expected = 1966;
        let actual = day23.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day23 = Day23::new("data/input_data.txt").unwrap();
        let expected = 6286;
        let actual = day23.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
