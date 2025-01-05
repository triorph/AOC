mod parser;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use crate::parser::parse_data;
use aoc_helpers::{point2d::Point2D, read_input_file, AOCCalculator, AOCFileOrParseError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day21 {
    data: Vec<Vec<usize>>,
}

impl AOCCalculator for Day21 {
    fn new(filename: &str) -> Result<Day21, AOCFileOrParseError> {
        Ok(Day21 {
            data: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

trait Location<T> {
    fn get_location(&self) -> Point2D;
    fn get_all() -> Vec<T>;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
    Activate,
}

impl Location<usize> for usize {
    fn get_location(&self) -> Point2D {
        match self {
            7 => Point2D { x: 0, y: 0 },
            8 => Point2D { x: 1, y: 0 },
            9 => Point2D { x: 2, y: 0 },
            4 => Point2D { x: 0, y: 1 },
            5 => Point2D { x: 1, y: 1 },
            6 => Point2D { x: 2, y: 1 },
            1 => Point2D { x: 0, y: 2 },
            2 => Point2D { x: 1, y: 2 },
            3 => Point2D { x: 2, y: 2 },
            0 => Point2D { x: 1, y: 3 },
            // magic number: 10 is Activate
            10 => Point2D { x: 2, y: 3 },
            _ => panic!("Invalid number"),
        }
    }

    fn get_all() -> Vec<usize> {
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    }
}

impl Location<Direction> for Direction {
    fn get_location(&self) -> Point2D {
        match self {
            Direction::Up => Point2D::from_usize(1, 0),
            Direction::Activate => Point2D::from_usize(2, 0),
            Direction::Left => Point2D::from_usize(0, 1),
            Direction::Down => Point2D::from_usize(1, 1),
            Direction::Right => Point2D::from_usize(2, 1),
        }
    }

    fn get_all() -> Vec<Direction> {
        vec![
            Direction::Right,
            Direction::Up,
            Direction::Left,
            Direction::Down,
            Direction::Activate,
        ]
    }
}

impl Direction {
    fn get_as_point(&self) -> Point2D {
        match self {
            Direction::Up => Point2D { x: 0, y: -1 },
            Direction::Left => Point2D { x: -1, y: 0 },
            Direction::Down => Point2D { x: 0, y: 1 },
            Direction::Right => Point2D { x: 1, y: 0 },
            _ => Point2D { x: 0, y: 0 },
        }
    }
}

impl Direction {}

trait ConcatVec<T> {
    fn concat(self, b: Vec<T>) -> Vec<T>;
}

impl<T> ConcatVec<T> for Vec<T>
where
    T: Clone,
{
    fn concat(self, b: Vec<T>) -> Vec<T> {
        let mut ret = self.clone();
        ret.extend(b);
        ret
    }
}

impl Day21 {
    fn find_direction_for_number(&self, source: usize, dest: usize) -> Vec<Vec<Direction>> {
        let source_point = source.get_location();
        let dest_point = dest.get_location();
        let all_locations = usize::get_all()
            .into_iter()
            .map(|d| d.get_location())
            .collect::<HashSet<Point2D>>();
        let mut to_explore: BinaryHeap<Reverse<(usize, Vec<Direction>, Point2D)>> =
            BinaryHeap::new();
        to_explore.push(Reverse((0, vec![], source_point)));
        let mut best_paths: HashMap<Point2D, (usize, Vec<Vec<Direction>>)> = HashMap::new();
        while let Some(Reverse((distance, path, this_point))) = to_explore.pop() {
            let previous_explore = best_paths.get(&this_point);
            if previous_explore.is_none()
                || previous_explore.is_some_and(|(prev_dist, _)| distance <= *prev_dist)
            {
                if previous_explore.is_none()
                    || previous_explore.is_some_and(|(prev_dist, _)| distance < *prev_dist)
                {
                    best_paths.insert(this_point, (distance, vec![path.clone()]));
                } else {
                    best_paths
                        .entry(this_point)
                        .or_insert((distance, Vec::new()))
                        .1
                        .push(path.clone());
                }
                for direction in Direction::get_all().into_iter() {
                    if direction == Direction::Activate {
                        continue;
                    }
                    let next_point = this_point + direction.get_as_point();
                    if !all_locations.contains(&next_point) {
                        continue;
                    }
                    // let cost = if path.is_empty() || path[path.len() - 1] == direction {
                    //     distance + 1
                    // } else {
                    //     distance + 1000
                    // };
                    let cost = distance + 1;
                    let mut neighbour_path = path.clone();
                    neighbour_path.push(direction);
                    to_explore.push(Reverse((cost, neighbour_path, next_point)))
                }
            }
        }
        best_paths
            .get(&dest_point)
            .unwrap()
            .1
            .clone()
            .into_iter()
            .map(|d| d.concat(vec![Direction::Activate]))
            .collect::<Vec<Vec<Direction>>>()
    }

    fn find_directions_for_each_number(&self) -> HashMap<(usize, usize), Vec<Vec<Direction>>> {
        let mut ret = HashMap::new();
        for source in usize::get_all().into_iter() {
            for dest in usize::get_all().into_iter() {
                let directions = self.find_direction_for_number(source, dest);
                ret.insert((source, dest), directions);
            }
        }
        ret
    }

    fn find_direction_to_direction(
        &self,
        source: &Direction,
        dest: &Direction,
    ) -> Vec<Vec<Direction>> {
        let source_point = source.get_location();
        let dest_point = dest.get_location();
        let all_locations = Direction::get_all()
            .into_iter()
            .map(|d| d.get_location())
            .collect::<HashSet<Point2D>>();
        let mut to_explore: BinaryHeap<Reverse<(usize, Vec<Direction>, Point2D)>> =
            BinaryHeap::new();
        to_explore.push(Reverse((0, vec![], source_point)));
        let mut best_paths: HashMap<Point2D, (usize, Vec<Vec<Direction>>)> = HashMap::new();
        while let Some(Reverse((distance, path, this_point))) = to_explore.pop() {
            let previous_explore = best_paths.get(&this_point);
            if previous_explore.is_none()
                || previous_explore.is_some_and(|(prev_dist, _)| distance <= *prev_dist)
            {
                if previous_explore.is_none()
                    || previous_explore.is_some_and(|(prev_dist, _)| distance < *prev_dist)
                {
                    best_paths.insert(this_point, (distance, vec![path.clone()]));
                } else {
                    best_paths
                        .entry(this_point)
                        .or_insert((distance, Vec::new()))
                        .1
                        .push(path.clone());
                }
                for direction in Direction::get_all().into_iter() {
                    if direction == Direction::Activate {
                        continue;
                    }
                    let next_point = this_point + direction.get_as_point();
                    if !all_locations.contains(&next_point) {
                        continue;
                    }
                    // let cost = if path.is_empty() || path[path.len() - 1] == direction {
                    //     distance + 1
                    // } else {
                    //     distance + 1000
                    // };
                    let cost = distance + 1;
                    let mut neighbour_path = path.clone();
                    neighbour_path.push(direction);
                    to_explore.push(Reverse((cost, neighbour_path, next_point)))
                }
            }
        }
        best_paths
            .get(&dest_point)
            .unwrap()
            .1
            .clone()
            .into_iter()
            .map(|d| d.concat(vec![Direction::Activate]))
            .collect()
    }

    fn find_directions_for_each_direction(
        &self,
    ) -> HashMap<(Direction, Direction), Vec<Vec<Direction>>> {
        let mut ret = HashMap::new();
        for source in Direction::get_all().into_iter() {
            for dest in Direction::get_all().into_iter() {
                let directions = self.find_direction_to_direction(&source, &dest);
                ret.insert((source, dest), directions);
            }
        }
        ret
    }

    fn calculate_line_number_to_directions(
        &self,
        number_map: &HashMap<(usize, usize), Vec<Vec<Direction>>>,
        line: &[usize],
    ) -> Vec<Vec<Direction>> {
        // Finds all possible permutations of directions for a line number
        let to_combine = line
            .windows(2)
            .map(|input| number_map.get(&(input[0], input[1])).unwrap().clone())
            .collect::<Vec<Vec<Vec<Direction>>>>();
        let mut indexer = vec![0; to_combine.len()];
        let mut ret = vec![];

        loop {
            let next = to_combine
                .iter()
                .enumerate()
                .flat_map(|(i, input)| input[indexer[i]].to_vec())
                .collect::<Vec<Direction>>();
            ret.push(vec![Direction::Activate].concat(next));
            let mut tier = to_combine.len() - 1;
            indexer[tier] += 1;

            while indexer[tier] == to_combine[tier].len() {
                if tier == 0 {
                    let min_len = ret.iter().map(|x| x.len()).min().unwrap();
                    return ret.into_iter().filter(|x| x.len() == min_len).collect();
                }
                indexer[tier] = 0;
                tier -= 1;
                indexer[tier] += 1;
            }
        }
    }

    fn get_direction_len(
        direction_map: &HashMap<(Direction, Direction), Vec<Vec<Direction>>>,
        cached_results: &mut HashMap<(Direction, Direction, usize), usize>,
        source: Direction,
        dest: Direction,
        depth: usize,
        total_depth: usize,
    ) -> usize {
        // Finds the smallest length of all the options for a direction.
        // Recursively goes deeper, and uses a hashmap to cache the results
        if cached_results.contains_key(&(source, dest, depth)) {
            return *cached_results.get(&(source, dest, depth)).unwrap();
        }
        let mut best_min = None;
        for result in direction_map.get(&(source, dest)).unwrap().iter() {
            let min = if depth == total_depth {
                result.len()
            } else {
                vec![Direction::Activate]
                    .concat(result.to_vec())
                    .windows(2)
                    .map(|vals| {
                        Day21::get_direction_len(
                            direction_map,
                            cached_results,
                            vals[0],
                            vals[1],
                            depth + 1,
                            total_depth,
                        )
                    })
                    .sum::<usize>()
            };
            if best_min.is_none() || best_min.is_some_and(|best_min| min < best_min) {
                best_min = Some(min);
            }
        }
        cached_results.insert((source, dest, depth), best_min.unwrap());
        best_min.unwrap()
    }

    fn calculate_line(
        &self,
        number_map: &HashMap<(usize, usize), Vec<Vec<Direction>>>,
        direction_map: &HashMap<(Direction, Direction), Vec<Vec<Direction>>>,
        line: &[usize],
        total_runs: usize,
    ) -> usize {
        let activated_line = vec![10].concat(line.to_vec()).concat(vec![10]);
        let possible_directions =
            self.calculate_line_number_to_directions(number_map, &activated_line);
        let mut cached_results = HashMap::new();
        let line_len = possible_directions
            .iter()
            .map(|directions| {
                directions
                    .windows(2)
                    .map(|vals| {
                        Day21::get_direction_len(
                            direction_map,
                            &mut cached_results,
                            vals[0],
                            vals[1],
                            1,
                            total_runs,
                        )
                    })
                    .sum::<usize>()
            })
            .min()
            .unwrap();
        let number = line.iter().fold(0, |acc, x| acc * 10 + x);
        line_len * number
    }

    fn calculate_day_a(&self) -> usize {
        let number_map = self.find_directions_for_each_number();
        let direction_map = self.find_directions_for_each_direction();
        self.data
            .iter()
            .map(|line| self.calculate_line(&number_map, &direction_map, line, 2))
            .sum()
    }

    fn calculate_day_b(&self) -> usize {
        let number_map = self.find_directions_for_each_number();
        let direction_map = self.find_directions_for_each_direction();
        self.data
            .iter()
            .map(|line| self.calculate_line(&number_map, &direction_map, line, 25))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_individual_lines() {
        let day21 = Day21::new("data/test_data.txt").unwrap();
        let number_map = day21.find_directions_for_each_number();
        let direction_map = day21.find_directions_for_each_direction();
        for (numbers, expected, path) in [
            (
                [0, 2, 9],
                68 * 29,
                "029A = <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A",
            ),
            (
                [9, 8, 0],
                60 * 980,
                "980A = Av<<A>>^AvA^Av<<A>>^AAv<A<A>>^AAvAA<^A>Av<A>^AA<A>Av<A<A>>^AAAvA<^A>A",
            ),
            ([1, 7, 9], 68 * 179, "179A = "),
            ([4, 5, 6], 64 * 456, "456A = "),
            (
                [3, 7, 9],
                64 * 379,
                "379A = <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A",
            ),
        ]
        .into_iter()
        {
            let actual = day21.calculate_line(&number_map, &direction_map, &numbers, 2);
            println!("Path is  {:?}", path);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_calculate_day_a() {
        let day21 = Day21::new("data/test_data.txt").unwrap();
        let expected = 126384;
        let actual = day21.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day21 = Day21::new("data/test_data.txt").unwrap();
        let expected = 154115708116294;
        let actual = day21.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day21 = Day21::new("data/input_data.txt").unwrap();
        let expected = 278568;
        let actual = day21.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day21 = Day21::new("data/input_data.txt").unwrap();
        let expected = 341460772681012;
        let actual = day21.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
