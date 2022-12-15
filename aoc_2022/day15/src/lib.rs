mod parser;
mod point;
use std::collections::HashSet;

use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};
use point::Point;

pub struct Day15 {
    beacons: Vec<(Point, isize)>,
}

impl AOCCalculator for Day15 {
    fn new(filename: &str) -> Result<Day15, AOCFileOrParseError> {
        Ok(Day15 {
            beacons: parse_data(&read_input_file(filename)?)?
                .iter()
                .copied()
                .map(|(p1, p2)| (p1, p1.manhattan_distance(&p2)))
                .collect(),
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day15 {
    fn calculate_day_a(&self) -> usize {
        self.invalidate_at_y(2000000)
    }

    fn invalidate_at_y(&self, y: isize) -> usize {
        let mut ret = HashSet::new();
        for (beacon, distance) in self.beacons.iter() {
            let y_diff = (beacon.y - y).abs();
            ret.extend(
                ((beacon.x - (distance - y_diff))..(beacon.x + (distance - y_diff)))
                    .map(|x| Point { x, y }),
            )
        }
        ret.len()
    }

    fn calculate_day_b(&self) -> isize {
        for (beacon, distance) in self.beacons.iter() {
            for point in beacon
                .get_all_points_at_manhattan_distance(distance + 1)
                .iter()
            {
                if point.x >= 4000000 || point.y >= 4000000 || point.x < 0 || point.y < 0 {
                    continue;
                }
                if !self.would_be_invalid(point) {
                    println!("point: {:?}", point);
                    return point.x * 4000000 + point.y;
                }
            }
        }
        panic!("No point found")
    }

    fn would_be_invalid(&self, point: &Point) -> bool {
        self.beacons
            .iter()
            .any(|(p, distance)| p.manhattan_distance(point) < *distance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day15 = Day15::new("data/test_data.txt").unwrap();
        let expected = 26;
        let actual = day15.invalidate_at_y(20);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day15 = Day15::new("data/test_data.txt").unwrap();
        let expected = 0;
        let actual = day15.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_a_real_input() {
        let day15 = Day15::new("data/input_data.txt").unwrap();
        let expected = 0;
        let actual = day15.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b_real_input() {
        let day15 = Day15::new("data/input_data.txt").unwrap();
        let expected = 0;
        let actual = day15.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
