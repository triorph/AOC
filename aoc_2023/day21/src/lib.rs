mod parser;
use std::collections::{HashMap, HashSet};

use crate::parser::parse_data;
use aoc_helpers::point2d::{Neighbours, Point2D};
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};
use parser::GardenTile;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day21 {
    rocks: HashSet<Point2D>,
    start: Point2D,
    bounds: (usize, usize),
}

impl AOCCalculator for Day21 {
    fn new(filename: &str) -> Result<Day21, AOCFileOrParseError> {
        let mut rocks = HashSet::new();
        let mut start = Point2D { x: 0, y: 0 };
        let tiles = parse_data(&read_input_file(filename)?)?;
        for (y, line) in tiles.iter().enumerate() {
            for (x, value) in line.iter().enumerate() {
                match value {
                    GardenTile::Rock => {
                        rocks.insert(Point2D::from_usize(x, y));
                    }
                    GardenTile::Start => start = Point2D::from_usize(x, y),
                    _ => (),
                }
            }
        }
        Ok(Day21 {
            rocks,
            start,
            bounds: (tiles[0].len(), tiles.len()),
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day21 {
    fn within_bounds(&self, point: &Point2D) -> bool {
        (0..(self.bounds.0 as isize)).contains(&point.x)
            && (0..(self.bounds.1 as isize)).contains(&point.y)
    }

    fn find_plots_after_steps(&self, steps: usize) -> usize {
        let mut found = HashSet::new();
        found.insert(self.start);
        for _ in 0..steps {
            found = found
                .into_iter()
                .flat_map(|plot| plot.get_neighbours())
                .filter(|neighbour| self.within_bounds(neighbour))
                .filter(|neighbour| !self.rocks.contains(neighbour))
                .collect::<HashSet<Point2D>>();
        }
        found.len()
    }

    fn wrap_plot(&self, plot: &Point2D) -> Point2D {
        let max_x = self.bounds.0 as isize;
        let max_y = self.bounds.1 as isize;
        Point2D {
            x: ((plot.x % max_x) + max_x) % max_x,
            y: ((plot.y % max_y) + max_y) % max_y,
        }
    }

    fn get_wrap_location(&self, plot: &Point2D) -> Point2D {
        let max_x = self.bounds.0 as isize;
        let max_y = self.bounds.1 as isize;
        let x = if plot.x < 0 {
            (plot.x - max_x) / max_x
        } else {
            plot.x / max_x
        };
        let y = if plot.y < 0 {
            (plot.y - max_y) / max_y
        } else {
            plot.y / max_y
        };
        Point2D { x, y }
    }

    #[allow(dead_code)]
    fn print_garden(&self, garden: &HashMap<Point2D, HashSet<Point2D>>) -> String {
        let min_x = garden
            .iter()
            .map(|(point, wraps)| {
                point.x + wraps.iter().map(|w| w.x).min().unwrap_or(0) * self.bounds.0 as isize
            })
            .min()
            .unwrap_or(0);
        let max_x = garden
            .iter()
            .map(|(point, wraps)| {
                point.x + wraps.iter().map(|w| w.x).max().unwrap_or(0) * self.bounds.0 as isize
            })
            .max()
            .unwrap_or(0);
        let min_y = garden
            .iter()
            .map(|(point, wraps)| {
                point.y + wraps.iter().map(|w| w.y).min().unwrap_or(0) * self.bounds.1 as isize
            })
            .min()
            .unwrap_or(0);
        let max_y = garden
            .iter()
            .map(|(point, wraps)| {
                point.y + wraps.iter().map(|w| w.y).max().unwrap_or(0) * self.bounds.1 as isize
            })
            .max()
            .unwrap_or(0);
        let mut ret = String::new();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.rocks.contains(&self.wrap_plot(&Point2D { x, y })) {
                    ret += "#";
                } else if garden
                    .get(&self.wrap_plot(&Point2D { x, y }))
                    .unwrap_or(&HashSet::new())
                    .contains(&self.get_wrap_location(&Point2D { x, y }))
                {
                    ret += "O";
                } else {
                    ret += " ";
                }
            }
            ret += "\n";
        }
        ret
    }

    fn find_wrapped_plots_after_steps(&self, steps: usize) -> usize {
        let mut found: HashMap<Point2D, HashSet<Point2D>> = HashMap::new();
        found.insert(self.start, HashSet::from_iter([Point2D { x: 0, y: 0 }]));
        for _ in 0..steps {
            let mut next_found = HashMap::new();
            for (plot, wrap_locations) in found.iter() {
                for (neighbour, neighbour_wrap_location) in plot
                    .get_neighbours()
                    .into_iter()
                    .map(|neighbour| {
                        (
                            self.wrap_plot(&neighbour),
                            self.get_wrap_location(&neighbour),
                        )
                    })
                    .filter(|(neighbour, _)| !self.rocks.contains(neighbour))
                {
                    let neighbour_wrap_locations: HashSet<Point2D> = wrap_locations
                        .iter()
                        .map(|wrap_location| wrap_location + &neighbour_wrap_location)
                        .collect();
                    (*next_found.entry(neighbour).or_insert(HashSet::new()))
                        .extend(neighbour_wrap_locations);
                }
            }
            found = next_found;
        }
        found
            .values()
            .map(|wrap_locations| wrap_locations.len())
            .sum()
    }

    fn calculate_day_a(&self) -> usize {
        self.find_plots_after_steps(64)
    }

    fn calculate_day_b(&self) -> usize {
        self.find_wrapped_plots_after_steps(26501365)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day21 = Day21::new("data/test_data.txt").unwrap();
        let expected = 16;
        let actual = day21.find_plots_after_steps(6);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b_1() {
        let day21 = Day21::new("data/test_data.txt").unwrap();
        assert_eq!(day21.find_wrapped_plots_after_steps(6), 16);
    }

    #[test]
    fn test_calculate_day_b_2() {
        let day21 = Day21::new("data/test_data.txt").unwrap();
        assert_eq!(day21.find_wrapped_plots_after_steps(10), 50);
    }

    #[test]
    fn test_calculate_day_b_3() {
        let day21 = Day21::new("data/test_data.txt").unwrap();
        assert_eq!(day21.find_wrapped_plots_after_steps(50), 1594);
    }

    #[test]
    fn test_calculate_day_b_4() {
        let day21 = Day21::new("data/test_data.txt").unwrap();
        assert_eq!(day21.find_wrapped_plots_after_steps(100), 6536);
    }

    #[test]
    fn test_calculate_day_b_5() {
        let day21 = Day21::new("data/test_data.txt").unwrap();
        assert_eq!(day21.find_wrapped_plots_after_steps(500), 167004);
    }

    #[test]
    fn test_calculate_day_b_6() {
        let day21 = Day21::new("data/test_data.txt").unwrap();
        assert_eq!(day21.find_wrapped_plots_after_steps(1000), 668697);
    }

    #[test]
    fn test_calculate_day_b_7() {
        let day21 = Day21::new("data/test_data.txt").unwrap();
        assert_eq!(day21.find_wrapped_plots_after_steps(5000), 16733044);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day21 = Day21::new("data/input_data.txt").unwrap();
        let expected = 3770;
        let actual = day21.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day21 = Day21::new("data/input_data.txt").unwrap();
        let expected = 0;
        let actual = day21.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
