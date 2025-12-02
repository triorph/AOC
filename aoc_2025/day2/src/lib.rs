mod parser;
use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};
use fancy_regex::Regex;
use std::ops::Range;

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
    fn find_invalid_ids_in_range_a(&self, range: &Range<usize>) -> Vec<usize> {
        let re = Regex::new(r"^(.+)(\1)$").unwrap();
        (range.start..=range.end)
            .filter(|value| re.is_match(&value.to_string()).unwrap_or(false))
            .collect()
    }

    fn find_invalid_ids_in_range_b(&self, range: &Range<usize>) -> Vec<usize> {
        let re = Regex::new(r"^(.+)(\1+)$").unwrap();
        (range.start..=range.end)
            .filter(|value| re.is_match(&value.to_string()).unwrap_or(false))
            .collect()
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
        let actual = day2
            .find_invalid_ids_in_range_b(&(start..end))
            .into_iter()
            .sum();
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
