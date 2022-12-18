mod droplet;
mod parser;
use crate::parser::parse_data;
use aoc_helpers::hash_utils::FromVec;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};
use std::collections::{HashSet, VecDeque};

use self::droplet::Droplet;

pub struct Day18 {
    droplets: Vec<Droplet>,
}

impl AOCCalculator for Day18 {
    fn new(filename: &str) -> Result<Day18, AOCFileOrParseError> {
        Ok(Day18 {
            droplets: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day18 {
    fn calculate_day_a(&self) -> usize {
        self.droplets
            .iter()
            .map(|droplet| droplet.calculate_exposed_surface(&self.droplets))
            .sum()
    }

    fn is_point_outside(&self, starting_droplet: &Droplet) -> bool {
        let body = HashSet::from_vec(&self.droplets);
        let mut seen: HashSet<Droplet> = HashSet::new();
        let mut stack = VecDeque::from(vec![*starting_droplet]);
        let bounds = self.get_bounds();
        while let Some(droplet) = stack.pop_front() {
            if seen.contains(&droplet) || body.contains(&droplet) {
                continue;
            } else if self.droplet_out_of_bounds(&droplet, bounds) {
                return true;
            } else {
                seen.insert(droplet);
                stack.extend(droplet.get_neighbours())
            }
        }
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

    fn calculate_day_b(&self) -> usize {
        self.droplets
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
        let day18 = Day18::new("data/test_data.txt").unwrap();
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
        let day18 = Day18::new("data/input_data.txt").unwrap();
        let expected = 0;
        let actual = day18.calculate_day_b();
        assert_ne!(actual, 2070);
        assert_ne!(actual, 3374);
        assert_eq!(expected, actual);
    }
}
