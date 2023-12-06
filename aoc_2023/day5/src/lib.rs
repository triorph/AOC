mod converter;
mod parser;
use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};
use converter::ConverterMap;
use std::ops::Range;

pub struct Day5 {
    seeds: Vec<usize>,
    converter_maps: Vec<ConverterMap>,
}

impl AOCCalculator for Day5 {
    fn new(filename: &str) -> Result<Day5, AOCFileOrParseError> {
        let (seeds, converter_maps) = parse_data(&read_input_file(filename)?)?;
        Ok(Day5 {
            seeds,
            converter_maps,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day5 {
    fn full_convert(&self, input: usize) -> usize {
        self.converter_maps
            .iter()
            .fold(input, |x, converter| converter.convert(x))
    }

    fn full_convert_range(&self, input: Vec<Range<usize>>) -> Vec<Range<usize>> {
        let ret = self
            .converter_maps
            .iter()
            .fold(input, |x, converter| converter.convert_ranges(&x));
        ret
    }

    fn calculate_day_a(&self) -> usize {
        self.seeds
            .iter()
            .map(|seed| self.full_convert(*seed))
            .min()
            .unwrap_or(0)
    }

    fn get_seeds_as_range(&self) -> Vec<Range<usize>> {
        (0..self.seeds.len() / 2)
            .map(|i| self.seeds[i * 2]..(self.seeds[i * 2] + self.seeds[i * 2 + 1]))
            .collect()
    }

    fn calculate_day_b(&self) -> usize {
        self.full_convert_range(self.get_seeds_as_range())
            .iter()
            .map(|range| range.start)
            .min()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day5 = Day5::new("data/test_data.txt").unwrap();
        let expected = 35;
        let actual = day5.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day5 = Day5::new("data/test_data.txt").unwrap();
        let expected = 46;
        let actual = day5.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day5 = Day5::new("data/input_data.txt").unwrap();
        let expected = 174137457;
        let actual = day5.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day5 = Day5::new("data/input_data.txt").unwrap();
        let expected = 1493866;
        let actual = day5.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
