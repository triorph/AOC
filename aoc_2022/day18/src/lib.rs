mod droplet;
mod parser;
use crate::parser::parse_data;
use aoc_helpers::hash_utils::FromVec;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};
use std::collections::{HashSet, VecDeque};

use self::droplet::Droplet;

#[derive(Clone)]
pub struct Day18 {
    droplets: HashSet<Droplet>,
    outside_cache: HashSet<Droplet>,
    inside_cache: HashSet<Droplet>,
}

impl AOCCalculator for Day18 {
    fn new(filename: &str) -> Result<Day18, AOCFileOrParseError> {
        Ok(Day18 {
            droplets: HashSet::from_vec(&parse_data(&read_input_file(filename)?)?),
            inside_cache: HashSet::new(),
            outside_cache: HashSet::new(),
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        let mut obj = self.clone();
        println!("{}b answer is {:?}", name, obj.calculate_day_b());
    }
}

impl Day18 {
    fn calculate_day_a(&self) -> usize {
        self.droplets
            .iter()
            .map(|droplet| droplet.calculate_exposed_surface(&self.droplets))
            .sum()
    }

    fn is_point_outside(&mut self, starting_droplet: &Droplet) -> bool {
        let mut seen: HashSet<Droplet> = HashSet::new();
        let mut stack = VecDeque::from(vec![*starting_droplet]);
        let bounds = self.get_bounds();
        while let Some(droplet) = stack.pop_front() {
            if seen.contains(&droplet) || self.droplets.contains(&droplet) {
                continue;
            } else if self.droplet_out_of_bounds(&droplet, bounds)
                || self.outside_cache.contains(&droplet)
            {
                self.outside_cache.extend(seen);
                return true;
            } else if self.inside_cache.contains(&droplet) {
                self.inside_cache.extend(seen);
                return false;
            } else {
                seen.insert(droplet);
                stack.extend(droplet.get_neighbours())
            }
        }
        self.inside_cache.extend(seen);
        false
    }

    fn get_bounds(&self) -> (isize, isize, isize, isize, isize, isize) {
        (
            self.droplets.iter().map(|droplet| droplet.x).min().unwrap(),
            self.droplets.iter().map(|droplet| droplet.x).max().unwrap(),
            self.droplets.iter().map(|droplet| droplet.y).min().unwrap(),
            self.droplets.iter().map(|droplet| droplet.y).max().unwrap(),
            self.droplets.iter().map(|droplet| droplet.z).min().unwrap(),
            self.droplets.iter().map(|droplet| droplet.z).max().unwrap(),
        )
    }

    fn droplet_out_of_bounds(
        &self,
        droplet: &Droplet,
        bounds: (isize, isize, isize, isize, isize, isize),
    ) -> bool {
        droplet.x < bounds.0
            || droplet.x > bounds.1
            || droplet.y < bounds.2
            || droplet.y > bounds.3
            || droplet.z < bounds.4
            || droplet.z > bounds.5
    }

    fn calculate_day_b(&mut self) -> usize {
        self.droplets
            .clone()
            .iter()
            .map(|droplet| {
                droplet
                    .get_neighbours()
                    .into_iter()
                    .filter(|neighbour| self.is_point_outside(neighbour))
                    .count()
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_calculate_day_a() {
        let day18 = Day18::new("data/test_data.txt").unwrap();
        let expected = 64;
        let actual = day18.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let mut day18 = Day18::new("data/test_data.txt").unwrap();
        let expected = 58;
        let actual = day18.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day18 = Day18::new("data/input_data.txt").unwrap();
        let expected = 3542;
        let actual = day18.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let mut day18 = Day18::new("data/input_data.txt").unwrap();
        let expected = 2080;
        let actual = day18.calculate_day_b();
        assert_ne!(actual, 2070);
        assert_ne!(actual, 3374);
        assert_eq!(expected, actual);
    }
}
