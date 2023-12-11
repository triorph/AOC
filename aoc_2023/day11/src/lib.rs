mod parser;
mod point;
use crate::parser::parse_data;
use aoc_helpers::point2d::Point2D;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};
use itertools::Itertools;

pub struct Day11 {
    galaxies: Vec<Point2D>,
}

fn to_points(map: &[Vec<bool>]) -> Vec<Point2D> {
    let mut ret = Vec::new();
    for (y, line) in map.iter().enumerate() {
        for (x, value) in line.iter().enumerate() {
            if *value {
                ret.push(Point2D {
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
    fn expand_horizontally_by(&self, galaxies: &[Point2D], expansion_size: usize) -> Vec<Point2D> {
        let mut ret = Vec::new();
        let mut expansion_constant = 0;
        let max = galaxies.iter().map(|point| point.x).max().unwrap_or(0);
        for i in 0..=max {
            let filtered: Vec<Point2D> = galaxies
                .iter()
                .filter(|point| point.x == i)
                .map(|point| {
                    point
                        + &Point2D {
                            x: expansion_constant,
                            y: 0,
                        }
                })
                .collect();
            if filtered.is_empty() {
                expansion_constant += expansion_size as isize;
            } else {
                ret.extend(filtered);
            }
        }
        ret
    }

    fn expand_vertically_by(&self, galaxies: &[Point2D], expansion_size: usize) -> Vec<Point2D> {
        let mut ret = Vec::new();
        let mut expansion_constant = 0;
        let max = galaxies.iter().map(|point| point.y).max().unwrap_or(0);
        for i in 0..=max {
            let filtered: Vec<Point2D> = galaxies
                .iter()
                .filter(|point| point.y == i)
                .map(|point| {
                    point
                        + &Point2D {
                            x: 0,
                            y: expansion_constant,
                        }
                })
                .collect();
            if filtered.is_empty() {
                expansion_constant += expansion_size as isize;
            } else {
                ret.extend(filtered);
            }
        }
        ret
    }

    fn expand_galaxies_by(&self, expansion_size: usize) -> Vec<Point2D> {
        self.expand_horizontally_by(
            &self.expand_vertically_by(&self.galaxies, expansion_size),
            expansion_size,
        )
    }

    fn calculate_all_shortest_paths_for_expansion_size(&self, expansion_size: usize) -> usize {
        let expanded_galaxies = self.expand_galaxies_by(expansion_size);
        expanded_galaxies
            .iter()
            .combinations(2)
            .map(|galaxy_pair| galaxy_pair[0].get_manhattan_distance(galaxy_pair[1]))
            .sum::<usize>()
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
        let expected = 702152204842;
        let actual = day11.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
