mod parser;
use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};

pub struct Day6 {
    races: Vec<(usize, usize)>,
}

impl AOCCalculator for Day6 {
    fn new(filename: &str) -> Result<Day6, AOCFileOrParseError> {
        Ok(Day6 {
            races: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

/// convert ["23","51","46"] to something 235146
fn get_day_b_number(input: &[usize]) -> usize {
    input
        .iter()
        .map(|number| number.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<usize>()
        .unwrap()
}

/// How many ways you can use your time to beat the best distance
fn how_many_ways_to_beat(time: usize, best_distance: usize) -> usize {
    // x * (time - x) > best_distance
    // can be expressed as a quandratic equation
    //
    // -x * x + time * x - best_distance > 0
    //
    // quadratic formula is (-b +- sqrt(b*b - 4ac)) / 2a
    //
    // which is: (- time +- sqrt(time * time - 4 * best_distance)) / -2
    let middle = time as f64 / 2.0;
    let offset = ((time * time - 4 * best_distance) as f64).sqrt() / 2.0;
    // note: we want to be under the roots, as exactly equal is not
    // "beating" the best distance
    let upper = (middle + offset - 1.0).ceil() as usize;
    let lower = (middle - offset + 1.0).floor() as usize;
    upper - lower + 1
}

impl Day6 {
    fn calculate_day_a(&self) -> usize {
        self.races
            .iter()
            .copied()
            .map(|(time, best_distance)| how_many_ways_to_beat(time, best_distance))
            .product()
    }

    fn all_times(&self) -> Vec<usize> {
        self.races.iter().map(|(time, _)| time).copied().collect()
    }
    fn all_best_distances(&self) -> Vec<usize> {
        self.races.iter().map(|(_, dist)| dist).copied().collect()
    }

    fn get_day_b_time_and_distance(&self) -> (usize, usize) {
        (
            get_day_b_number(&self.all_times()),
            get_day_b_number(&self.all_best_distances()),
        )
    }

    fn calculate_day_b(&self) -> usize {
        let (time, best_distance) = self.get_day_b_time_and_distance();
        how_many_ways_to_beat(time, best_distance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day6 = Day6::new("data/test_data.txt").unwrap();
        let expected = 288;
        let actual = day6.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day6 = Day6::new("data/test_data.txt").unwrap();
        let expected = 71503;
        let actual = day6.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day6 = Day6::new("data/input_data.txt").unwrap();
        let expected = 138915;
        let actual = day6.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day6 = Day6::new("data/input_data.txt").unwrap();
        let expected = 27340847;
        let actual = day6.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_how_many_ways_to_beat() {
        assert_eq!(how_many_ways_to_beat(7, 9), 4);
        assert_eq!(how_many_ways_to_beat(15, 40), 8);
        assert_eq!(how_many_ways_to_beat(30, 200), 9);
        assert_eq!(how_many_ways_to_beat(71530, 940200), 71503);
    }
}
