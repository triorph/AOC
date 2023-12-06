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

impl Day6 {
    fn calculate_day_a(&self) -> usize {
        self.races
            .iter()
            .map(|(time, best_distance)| {
                (1..*time)
                    .map(|speed| speed * (time - speed))
                    .filter(|distance| distance > best_distance)
                    .count()
            })
            .product()
    }

    fn get_day_b_time_and_distance(&self) -> (usize, usize) {
        let time = self
            .races
            .iter()
            .map(|(time, _)| time.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse::<usize>()
            .unwrap();
        let best_distance = self
            .races
            .iter()
            .map(|(_, distance)| distance.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse::<usize>()
            .unwrap();
        (time, best_distance)
    }

    fn calculate_day_b(&self) -> usize {
        let (time, best_distance) = self.get_day_b_time_and_distance();
        let start = time / 2;
        let mut count = 0;
        for i in start..time {
            if i * (time - i) > best_distance {
                count += 2;
            } else {
                break;
            }
        }
        if time % 2 == 0 {
            count -= 1;
        }
        count
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
}
