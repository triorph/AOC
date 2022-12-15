mod parser;
mod point;
mod range;

use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};
use point::Point;
use range::RangeCollection;

pub struct Day15 {
    sensors: Vec<(Point, isize)>,
}

impl AOCCalculator for Day15 {
    fn new(filename: &str) -> Result<Day15, AOCFileOrParseError> {
        Ok(Day15 {
            sensors: parse_data(&read_input_file(filename)?)?
                .iter()
                .copied()
                .map(|(p1, p2)| (p1, p1.manhattan_distance(&p2)))
                .collect(),
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a(2000000));
        println!("{}b answer is {:?}", name, self.calculate_day_b(4000000));
    }
}

impl Day15 {
    fn calculate_day_a(&self, y: isize) -> usize {
        let mut ranges = RangeCollection::new();
        for (sensor, distance) in self.sensors.iter() {
            let y_diff = (sensor.y - y).abs();
            let min = sensor.x - (distance - y_diff);
            let max = sensor.x + (distance - y_diff);
            if min < max {
                ranges.add(min, max);
            }
        }
        ranges.get_size()
    }

    fn calculate_day_b(&self, max_size: isize) -> isize {
        for (sensor, distance) in self.sensors.iter() {
            for point in sensor
                .get_all_points_at_manhattan_distance(distance + 1)
                .iter()
            {
                if point.outside_max(max_size) {
                    continue;
                }
                if !self.would_be_invalid(point) {
                    return point.calculate_tuning_frequency();
                }
            }
        }
        panic!("No point found")
    }

    fn would_be_invalid(&self, point: &Point) -> bool {
        self.sensors
            .iter()
            .any(|(sensor, distance)| sensor.manhattan_distance(point) <= *distance)
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
        let actual = day15.calculate_day_a(20);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day15 = Day15::new("data/test_data.txt").unwrap();
        let expected = 56000011;
        let actual = day15.calculate_day_b(20);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_a_real_input() {
        let day15 = Day15::new("data/input_data.txt").unwrap();
        let expected = 4873353;
        let actual = day15.calculate_day_a(2000000);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b_real_input() {
        let day15 = Day15::new("data/input_data.txt").unwrap();
        let expected = 11600823139120;
        let actual = day15.calculate_day_b(4000000);
        assert_eq!(expected, actual);
    }
}
