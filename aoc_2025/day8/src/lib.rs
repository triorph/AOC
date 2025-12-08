mod parser;
use std::collections::HashSet;

use crate::parser::parse_data;
use aoc_helpers::{point3d::Point3D, read_input_file, AOCCalculator, AOCFileOrParseError};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day8 {
    locations: Vec<Point3D>,
}

impl AOCCalculator for Day8 {
    fn new(filename: &str) -> Result<Day8, AOCFileOrParseError> {
        Ok(Day8 {
            locations: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a(1000));
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day8 {
    fn calculate_day_a(&self, iterations: usize) -> usize {
        let circuits = self
            .get_sorted_distances()
            .into_iter()
            .take(iterations)
            .fold(self.get_initial_circuits(), |circuits, next| {
                self.add_next_connection_to_circuits(circuits, &next)
            });
        self.top_3_circuits(&circuits)
    }

    fn calculate_day_b(&self) -> usize {
        let last_connection_err = self.get_sorted_distances().into_iter().try_fold(
            self.get_initial_circuits(),
            |circuits, next| {
                if self.is_connection_noop(&circuits, &next) {
                    Ok(circuits)
                } else if circuits.len() > 2 {
                    Ok(self.add_next_connection_to_circuits(circuits, &next))
                } else {
                    Err(next)
                }
            },
        );
        let last_connection = last_connection_err.unwrap_err();
        (last_connection.1.x * last_connection.2.x) as usize
    }

    fn get_sorted_distances(&self) -> Vec<(f64, Point3D, Point3D)> {
        let mut distances: Vec<_> = self
            .locations
            .iter()
            .cloned()
            .cartesian_product(self.locations.iter().cloned())
            .filter(|(a, b)| a != b)
            .map(|(a, b)| (a.get_euclidean_distance(&b), a, b))
            .collect();
        distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        // step_by_2 to account for (x,a,b) and (x,b,a) being the same (and we should be sorting on x)
        distances.into_iter().step_by(2).collect()
    }

    fn get_initial_circuits(&self) -> Vec<HashSet<Point3D>> {
        self.locations
            .iter()
            .cloned()
            .map(|location| HashSet::from([location]))
            .collect()
    }

    fn add_next_connection_to_circuits(
        &self,
        circuits: Vec<HashSet<Point3D>>,
        next_connection: &(f64, Point3D, Point3D),
    ) -> Vec<HashSet<Point3D>> {
        if self.is_connection_noop(&circuits, next_connection) {
            return circuits;
        }
        let (to_change, remaining): (Vec<_>, Vec<_>) = circuits
            .iter()
            .partition(|circuit| {
                circuit.contains(&next_connection.1) || circuit.contains(&next_connection.2)
            })
            .clone();

        let mut first = to_change[0].clone();
        to_change[1].iter().for_each(|p| {
            first.insert(*p);
        });
        [vec![first], remaining.into_iter().cloned().collect()].concat()
    }

    fn is_connection_noop(
        &self,
        circuits: &[HashSet<Point3D>],
        next: &(f64, Point3D, Point3D),
    ) -> bool {
        circuits
            .iter()
            .any(|circuit| circuit.contains(&next.1) && circuit.contains(&next.2))
    }

    fn top_3_circuits(&self, circuits: &[HashSet<Point3D>]) -> usize {
        let mut by_len: Vec<_> = circuits.iter().map(|points| points.len()).collect();
        by_len.sort();
        by_len
            .into_iter()
            .rev()
            .take(3)
            .reduce(|a, b| a * b)
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day8 = Day8::new("data/test_data.txt").unwrap();
        let expected = 40;
        let actual = day8.calculate_day_a(10);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day8 = Day8::new("data/test_data.txt").unwrap();
        let expected = 25272;
        let actual = day8.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day8 = Day8::new("data/input_data.txt").unwrap();
        let expected = 171503;
        let actual = day8.calculate_day_a(1000);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day8 = Day8::new("data/input_data.txt").unwrap();
        let expected = 9069509600;
        let actual = day8.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
