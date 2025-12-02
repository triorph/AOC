mod parser;
use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};
use std::{collections::HashSet, ops::Range};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day2 {
    ranges: Vec<Range<usize>>,
}

impl AOCCalculator for Day2 {
    fn new(filename: &str) -> Result<Day2, AOCFileOrParseError> {
        Ok(Day2 {
            ranges: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day2 {
    fn find_invalid_ids_in_range_a(&self, range: &Range<usize>) -> HashSet<usize> {
        self.find_invalid_ids_with_divisor(&range.start.to_string(), &range.end.to_string(), 2)
            .into_iter()
            .filter(|value| (range.start..=range.end).contains(value))
            .collect()
    }

    fn build_segments(
        &self,
        start_segment: &str,
        end_segment: &str,
        divisor: usize,
    ) -> HashSet<usize> {
        (start_segment.parse::<usize>().unwrap()..=end_segment.parse::<usize>().unwrap())
            .map(|segment| segment.to_string().repeat(divisor))
            .map(|value_str| value_str.parse::<usize>().unwrap())
            .collect::<HashSet<usize>>()
    }

    fn find_invalid_ids_with_divisor(
        &self,
        range_start_str: &str,
        range_end_str: &str,
        divisor: usize,
    ) -> HashSet<usize> {
        if range_end_str.len() % divisor != 0 && range_start_str.len() % divisor != 0 {
            HashSet::new()
        } else if range_start_str.len() % divisor != 0 {
            let start_segment = if range_start_str.len() == range_end_str.len() {
                range_start_str[0..range_start_str.len() / divisor].to_string()
            } else {
                // Set to "100" if start value is "98" and end value is "110"
                "1".to_string() + &"0".to_string().repeat(range_end_str.len() / divisor - 1)
            };
            let end_segment = range_end_str[0..range_end_str.len() / divisor].to_string();
            self.build_segments(&start_segment, &end_segment, divisor)
        } else {
            let start_segment = range_start_str[0..range_start_str.len() / divisor].to_string();
            let end_segment = if range_start_str.len() == range_end_str.len() {
                range_end_str[0..range_end_str.len() / divisor].to_string()
            } else {
                // set end to "999" start value is "920" and end value is "1100"
                "9".to_string().repeat(range_start_str.len() / divisor)
            };
            self.build_segments(&start_segment, &end_segment, divisor)
        }
    }

    fn find_invalid_ids_in_range_b(&self, range: &Range<usize>) -> HashSet<usize> {
        let range_start_str = range.start.to_string();
        let range_end_str = range.end.to_string();
        (2..=(range_end_str.len()))
            .flat_map(|divisor| {
                self.find_invalid_ids_with_divisor(&range_start_str, &range_end_str, divisor)
            })
            .filter(|value| (range.start..=range.end).contains(value))
            .collect::<HashSet<usize>>()
    }

    fn calculate_day_a(&self) -> usize {
        self.ranges
            .iter()
            .map(|range| {
                self.find_invalid_ids_in_range_a(range)
                    .into_iter()
                    .sum::<usize>()
            })
            .sum()
    }

    fn calculate_day_b(&self) -> usize {
        self.ranges
            .iter()
            .map(|range| {
                self.find_invalid_ids_in_range_b(range)
                    .into_iter()
                    .sum::<usize>()
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[test]
    fn test_calculate_day_a() {
        let day2 = Day2::new("data/test_data.txt").unwrap();
        let expected = 1227775554;
        let actual = day2.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day2 = Day2::new("data/test_data.txt").unwrap();
        let expected = 4174379265;
        let actual = day2.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day2 = Day2::new("data/input_data.txt").unwrap();
        let expected = 38158151648;
        let actual = day2.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(11, 22, 33)]
    #[case(95, 115, 210)]
    #[case(998, 1012, 2009)]
    #[case(1188511880, 1188511890, 1188511885)]
    #[case(222220, 222224, 222222)]
    #[case(1698522, 1698528, 0)]
    #[case(446443, 446449, 446446)]
    #[case(38593856, 38593862, 38593859)]
    #[case(565653, 565659, 565656)]
    #[case(824824821, 824824827, 824824824)]
    #[case(2121212118, 2121212124, 2121212121)]
    fn test_find_invalid_ids_day_b(
        #[case] start: usize,
        #[case] end: usize,
        #[case] expected: usize,
    ) {
        let day2 = Day2::new("data/test_data.txt").unwrap();
        let actual_ids = day2.find_invalid_ids_in_range_b(&(start..end));
        println!("Returned IDs {:?}", actual_ids);
        let actual = actual_ids.into_iter().sum();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day2 = Day2::new("data/input_data.txt").unwrap();
        let expected = 45283684555;
        let actual = day2.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
