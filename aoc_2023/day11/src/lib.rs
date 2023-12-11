mod parser;
mod point;
use crate::parser::parse_data;
use crate::point::Point;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};
use std::collections::HashSet;

pub struct Day11 {
    galaxies: HashSet<Point>,
}

fn to_points(map: &[Vec<bool>]) -> HashSet<Point> {
    let mut ret = HashSet::new();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] {
                ret.insert(Point {
                    x: x as isize,
                    y: y as isize,
                });
            }
        }
    }
    ret
}

impl AOCCalculator for Day11 {
    fn new(filename: &str) -> Result<Day11, AOCFileOrParseError> {
        Ok(Day11 {
            galaxies: to_points(&parse_data(&read_input_file(filename)?)?),
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day11 {
    fn expand_horizontally_by(
        &self,
        galaxies: &HashSet<Point>,
        expansion_size: usize,
    ) -> HashSet<Point> {
        let mut ret = HashSet::new();
        let mut expansion_constant = 0;
        let max = galaxies.iter().map(|point| point.x).max().unwrap_or(0);
        for i in 0..=max {
            let filtered: Vec<Point> = galaxies
                .iter()
                .filter(|point| point.x == i)
                .map(|point| {
                    point
                        + &Point {
                            x: expansion_constant,
                            y: 0,
                        }
                })
                .collect();
            if filtered.len() == 0 {
                expansion_constant += expansion_size as isize;
            } else {
                ret.extend(filtered);
            }
        }
        ret
    }

    fn expand_vertically_by(
        &self,
        galaxies: &HashSet<Point>,
        expansion_size: usize,
    ) -> HashSet<Point> {
        let mut ret = HashSet::new();
        let mut expansion_constant = 0;
        let max = galaxies.iter().map(|point| point.y).max().unwrap_or(0);
        for i in 0..=max {
            let filtered: Vec<Point> = galaxies
                .iter()
                .filter(|point| point.y == i)
                .map(|point| {
                    point
                        + &Point {
                            x: 0,
                            y: expansion_constant,
                        }
                })
                .collect();
            if filtered.len() == 0 {
                expansion_constant += expansion_size as isize;
            } else {
                ret.extend(filtered);
            }
        }
        ret
    }

    fn expand_galaxies_by(&self, expansion_size: usize) -> HashSet<Point> {
        self.expand_horizontally_by(
            &self.expand_vertically_by(&self.galaxies, expansion_size),
            expansion_size,
        )
    }

    fn calculate_all_shortest_paths_for_expansion_size(&self, expansion_size: usize) -> usize {
        let expanded_galaxies = self.expand_galaxies_by(expansion_size);
        expanded_galaxies
            .iter()
            .map(|galaxy| {
                expanded_galaxies
                    .iter()
                    .filter(|other_galaxy| other_galaxy != &galaxy)
                    .map(|other_galaxy| galaxy.distance_to(&other_galaxy))
                    .sum::<usize>()
            })
            .sum::<usize>()
            / 2
    }

    fn calculate_day_a(&self) -> usize {
        self.calculate_all_shortest_paths_for_expansion_size(1)
    }

    fn calculate_day_b(&self) -> usize {
        self.calculate_all_shortest_paths_for_expansion_size(999999)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day11 = Day11::new("data/test_data.txt").unwrap();
        let expected = 374;
        let actual = day11.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day11 = Day11::new("data/test_data.txt").unwrap();
        let actual = day11.calculate_all_shortest_paths_for_expansion_size(9);
        assert_eq!(1030, actual);
        let day11 = Day11::new("data/test_data.txt").unwrap();
        let actual = day11.calculate_all_shortest_paths_for_expansion_size(99);
        assert_eq!(8410, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day11 = Day11::new("data/input_data.txt").unwrap();
        let expected = 9609130;
        let actual = day11.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day11 = Day11::new("data/input_data.txt").unwrap();
        let expected = 0;
        let actual = day11.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
