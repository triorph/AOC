mod parser;
use std::collections::{HashSet, VecDeque};

use crate::parser::parse_data;
use aoc_helpers::{
    point2d::{Neighbours, Point2D},
    read_input_file, AOCCalculator, AOCFileOrParseError,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day12 {
    garden: Vec<Vec<char>>,
}

impl AOCCalculator for Day12 {
    fn new(filename: &str) -> Result<Day12, AOCFileOrParseError> {
        Ok(Day12 {
            garden: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day12 {
    fn within_bounds(&self, plant_location: &Point2D) -> bool {
        (0..self.garden.len()).contains(&(plant_location.y as usize))
            && (0..self.garden[0].len()).contains(&(plant_location.x as usize))
    }
    fn get_plant(&self, plant_location: &Point2D) -> Option<char> {
        if self.within_bounds(plant_location) {
            Some(self.garden[plant_location.y as usize][plant_location.x as usize])
        } else {
            None
        }
    }

    fn separate_garden(&self) -> Vec<HashSet<Point2D>> {
        let mut ret: Vec<HashSet<Point2D>> = Vec::new();
        let mut explore = VecDeque::new();
        let mut explored = HashSet::new();
        explore.push_front(Point2D::from_usize(0, 0));
        while let Some(plant_location) = explore.pop_front() {
            if explored.contains(&plant_location) {
                continue;
            }
            explored.insert(plant_location);
            if let Some(plant_tile) = self.get_plant(&plant_location) {
                let neighbours: HashSet<Point2D> =
                    HashSet::from_iter(plant_location.get_neighbours());
                let plot_index = ret
                    .iter()
                    .enumerate()
                    .find(|(_, plot)| {
                        neighbours.iter().any(|neighbour| {
                            plot.contains(neighbour)
                                && self.get_plant(neighbour) == Some(plant_tile)
                        })
                    })
                    .map(|(index, _)| index);
                if let Some(plot_index) = plot_index {
                    ret[plot_index].insert(plant_location);
                } else {
                    ret.push(HashSet::from([plant_location]));
                }
                for neighbour in plant_location.get_neighbours().into_iter() {
                    if self.within_bounds(&neighbour) {
                        if self.get_plant(&neighbour) == Some(plant_tile) {
                            explore.push_front(neighbour);
                        } else {
                            explore.push_back(neighbour);
                        }
                    }
                }
            }
        }
        ret
    }

    fn calculate_perimeter(&self, plot: &HashSet<Point2D>) -> usize {
        let ret = plot
            .iter()
            .map(|point| {
                HashSet::from_iter(point.get_neighbours())
                    .difference(plot)
                    .count()
            })
            .sum();
        ret
    }

    fn corner_checks(
        &self,
        plot: &HashSet<Point2D>,
        location: &Point2D,
        corners: &[Point2D],
    ) -> bool {
        let corner_plants: Vec<Point2D> = corners
            .iter()
            .map(|corner_location| (location + corner_location))
            .collect();

        // ##  or #.
        // #.     ..
        // the corner piece must be different, and the two pieces next to it
        // must be either the same as your section (inner corner) or different
        // to the corner (outer corner)
        // also technically #.
        //                  .#
        // is a corner
        !plot.contains(&corner_plants[0])
            && (plot.contains(&corner_plants[1]) && plot.contains(&corner_plants[2])
                || (!plot.contains(&corner_plants[1]) && !plot.contains(&corner_plants[2])))
            || plot.contains(&corner_plants[0])
                && !plot.contains(&corner_plants[1])
                && !plot.contains(&corner_plants[2])
    }

    fn get_corners(&self) -> Vec<Vec<Point2D>> {
        vec![
            vec![
                Point2D { x: -1, y: -1 },
                Point2D { x: -1, y: 0 },
                Point2D { x: 0, y: -1 },
            ],
            vec![
                Point2D { x: 1, y: -1 },
                Point2D { x: 1, y: 0 },
                Point2D { x: 0, y: -1 },
            ],
            vec![
                Point2D { x: -1, y: 1 },
                Point2D { x: -1, y: 0 },
                Point2D { x: 0, y: 1 },
            ],
            vec![
                Point2D { x: 1, y: 1 },
                Point2D { x: 1, y: 0 },
                Point2D { x: 0, y: 1 },
            ],
        ]
    }

    fn calculate_number_sides(&self, plot: &HashSet<Point2D>) -> usize {
        // hack: num sides = num corners
        let ret = plot
            .iter()
            .map(|plot_point| {
                self.get_corners()
                    .into_iter()
                    .filter(|corners| self.corner_checks(plot, plot_point, corners))
                    .count()
            })
            .sum();
        println!(
            "Plot {:?} {:?} has corners {:?}",
            self.get_plant(plot.iter().next().unwrap()),
            plot,
            ret
        );
        ret
    }

    fn calculate_area(&self, plot: &HashSet<Point2D>) -> usize {
        plot.len()
    }

    fn calculate_day_a(&self) -> usize {
        let plots = self.separate_garden();
        plots
            .iter()
            .map(|plot| self.calculate_perimeter(plot) * self.calculate_area(plot))
            .sum()
    }

    fn calculate_day_b(&self) -> usize {
        let plots = self.separate_garden();
        plots
            .iter()
            .map(|plot| self.calculate_number_sides(plot) * self.calculate_area(plot))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day12 = Day12::new("data/test_data.txt").unwrap();
        let expected = 1930;
        let actual = day12.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day12 = Day12::new("data/test_data.txt").unwrap();
        let expected = 1206;
        let actual = day12.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b_2() {
        let day12 = Day12::new("data/test_data_2.txt").unwrap();
        let expected = 368;
        let actual = day12.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day12 = Day12::new("data/input_data.txt").unwrap();
        let expected = 1449902;
        let actual = day12.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day12 = Day12::new("data/input_data.txt").unwrap();
        let expected = 908042;
        let actual = day12.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
