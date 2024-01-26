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
const CYCLE_INDICES: [Point2D; 9] = [
    Point2D { x: -1, y: -1 },
    Point2D { x: -1, y: 0 },
    Point2D { x: -1, y: 1 },
    Point2D { x: 0, y: -1 },
    Point2D { x: 0, y: 0 },
    Point2D { x: 0, y: 1 },
    Point2D { x: 1, y: -1 },
    Point2D { x: 1, y: 0 },
    Point2D { x: 1, y: 1 },
];

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

    #[allow(dead_code)]
    fn print_garden_sizes(&self, garden: &HashMap<Point2D, HashSet<Point2D>>) -> String {
        let min_x = garden
            .iter()
            .map(|(_, wraps)| wraps.iter().map(|w| w.x).min().unwrap_or(0))
            .min()
            .unwrap_or(0);
        let max_x = garden
            .iter()
            .map(|(_, wraps)| wraps.iter().map(|w| w.x).max().unwrap_or(0))
            .max()
            .unwrap_or(0);
        let min_y = garden
            .iter()
            .map(|(_, wraps)| wraps.iter().map(|w| w.y).min().unwrap_or(0))
            .min()
            .unwrap_or(0);
        let max_y = garden
            .iter()
            .map(|(_, wraps)| wraps.iter().map(|w| w.y).max().unwrap_or(0))
            .max()
            .unwrap_or(0);
        let mut ret = String::new();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                ret += &format!(
                    "{: >3}",
                    &garden
                        .iter()
                        .map(|(_, wraps)| wraps.iter().filter(|w| w.x == x && w.y == y).count())
                        .sum::<usize>()
                        .to_string()
                );
                ret += ","
            }
            ret += "\n";
        }
        ret
    }

    fn print_plot_counts(&self, garden: &HashMap<Point2D, usize>) -> String {
        let min_x = garden.keys().map(|key| key.x).min().unwrap_or(0);
        let max_x = garden.keys().map(|key| key.x).max().unwrap_or(0);
        let min_y = garden.keys().map(|key| key.y).min().unwrap_or(0);
        let max_y = garden.keys().map(|key| key.y).max().unwrap_or(0);
        let mut ret = String::new();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                ret += &format!(
                    "{: >3}",
                    &garden.get(&Point2D { x, y }).unwrap_or(&0).to_string()
                );
                ret += ","
            }
            ret += "\n";
        }
        ret
    }

    fn build_plot_counts(
        &self,
        garden: &HashMap<Point2D, HashSet<Point2D>>,
    ) -> HashMap<Point2D, usize> {
        let mut ret: HashMap<Point2D, usize> = HashMap::new();
        for point_plots in garden.values() {
            for plot in point_plots.iter() {
                ret.entry(*plot).and_modify(|old| *old += 1).or_insert(1);
            }
        }
        ret
    }

    fn find_cycle(&self, counts: &[usize]) -> Option<usize> {
        let latest = counts[counts.len() - 1];
        if latest != 0 && counts.iter().filter(|c| **c == latest).count() > 1 {
            Some(
                counts
                    .iter()
                    .enumerate()
                    .find(|(_, c)| **c == latest)
                    .map(|(i, _)| i)
                    .unwrap()
                    - counts
                        .iter()
                        .enumerate()
                        .find(|(_, c)| **c != 0)
                        .map(|(i, _)| i)
                        .unwrap(),
            )
        } else {
            None
        }
    }

    fn normalise_expansion_point(&self, point: &Point2D) -> Point2D {
        Point2D {
            x: if point.x == 0 { 0 } else { point.x.signum() },
            y: if point.y == 0 { 0 } else { point.y.signum() },
        }
    }

    fn find_wrapped_plots_after_steps(&self, steps: usize) -> usize {
        let mut found: HashMap<Point2D, HashSet<Point2D>> = HashMap::new();
        found.insert(self.start, HashSet::from_iter([Point2D { x: 0, y: 0 }]));
        let mut plot_counts = self.build_plot_counts(&found);
        let mut cycles = vec![vec![]; 9];
        let mut expansion_times = vec![vec![]; 9];
        let mut cycle_indices_map: HashMap<Point2D, usize> = HashMap::new();
        for (i, index) in CYCLE_INDICES.iter().enumerate() {
            cycle_indices_map.insert(*index, i);
        }
        let mut cycles_found = false;
        for step in 0..100 {
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
            let next_plot_counts = self.build_plot_counts(&next_found);
            if next_plot_counts.len() != plot_counts.len() {
                println!("Opened up a new plot at iteration {:?} with {:?} plots open and {:?} steps total",
                    step, next_plot_counts.len(), next_plot_counts.values().sum::<usize>()
                );
                let all_new_keys: HashSet<Point2D> =
                    HashSet::from_iter(next_plot_counts.keys().copied());
                let all_old_keys: HashSet<Point2D> =
                    HashSet::from_iter(plot_counts.keys().copied());

                let new_keys = all_new_keys.difference(&all_old_keys);
                let normalised_new_keys = new_keys
                    .into_iter()
                    .map(|point| self.normalise_expansion_point(point))
                    .collect::<HashSet<Point2D>>();
                for key in normalised_new_keys.into_iter() {
                    let index = cycle_indices_map.get(&key).unwrap();
                    expansion_times[*index].push(step);
                }
            }
            if !cycles_found {
                for i in 0..9 {
                    cycles[i].push(*next_plot_counts.get(&CYCLE_INDICES[i]).unwrap_or(&0));
                }
            }
            if !cycles_found && cycles.iter().all(|cycle| self.find_cycle(cycle).is_some()) {
                cycles_found = true;
                println!(
                    "at time {} found all cycle for plot {:?} -\n {}",
                    step,
                    cycles,
                    self.print_garden_sizes(&next_found)
                );
            }
            plot_counts = next_plot_counts;
            found = next_found;
        }
        let plots = self.predict_plots(100, cycles.clone(), expansion_times.clone());
        for plot in plot_counts.keys() {
            assert_eq!(
                plots.get(plot).unwrap_or(&0),
                plot_counts.get(plot).unwrap_or(&0)
            )
        }

        found
            .values()
            .map(|wrap_locations| wrap_locations.len())
            .sum()
    }

    fn get_all_points_at_distance(&self, point: &Point2D, distance: usize) -> Vec<Point2D> {
        if point.x.abs() + point.y.abs() == 1 {
            vec![point * distance as isize]
        } else {
            // assume diagonal
            let ret = (0..(distance as isize))
                .map(|offset| Point2D {
                    x: point.x * distance as isize - offset * point.x.signum(),
                    y: point.y * distance as isize
                        - (distance as isize - 1 - offset) * point.y.signum(),
                })
                .collect();
            println!(
                "Diagonal points at distance {:?} for point {:?} are {:?}",
                distance, point, ret
            );
            ret
        }
    }

    fn predict_plots(
        &self,
        step: usize,
        cycles: Vec<Vec<usize>>,
        expansion_times: Vec<Vec<usize>>,
    ) -> HashMap<Point2D, usize> {
        let mut ret = HashMap::new();
        let origin = Point2D { x: 0, y: 0 };
        for ((cycle_point, cycle), expansions) in CYCLE_INDICES
            .iter()
            .zip(cycles.iter())
            .zip(expansion_times.iter())
        {
            println!("cycle {:?}: {:?}", cycle_point, cycle);
            println!("Expansion times for this type: {:?}", expansions);
            let time_until_zero = cycle
                .iter()
                .enumerate()
                .find(|(_, c)| **c != 0)
                .map(|(i, _)| i)
                .unwrap();
            let mut distance = 1;
            let cycle_len = self.find_cycle(cycle).unwrap();
            let mut count = cycle[cycle_len + time_until_zero];
            while count != 0 {
                for point in self
                    .get_all_points_at_distance(cycle_point, distance)
                    .into_iter()
                {
                    ret.insert(point, count);
                }
                distance += 1;
                if *cycle_point == origin {
                    break;
                }

                if time_until_zero * distance > step + cycle_len {
                    count = 0;
                } else if time_until_zero * distance < time_until_zero + cycle_len {
                    let cycle_index = if step > time_until_zero * distance + cycle_len {
                        time_until_zero + cycle_len
                    } else {
                        1
                    };
                    count = cycle[cycle_index];
                } else {
                    count = 0;
                }
            }
        }
        println!("Worked out cycles: {:?}", ret);
        println!("Looks like:\n{}", self.print_plot_counts(&ret));
        ret
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
