mod parser;
use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day01 {
    dials: Vec<isize>,
}

impl AOCCalculator for Day01 {
    fn new(filename: &str) -> Result<Day01, AOCFileOrParseError> {
        Ok(Day01 {
            dials: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day01 {
    fn get_dial_positions(&self) -> Vec<isize> {
        self.dials.iter().fold(vec![50], |mut acc, x| {
            acc.push(acc[acc.len() - 1] + x);
            acc
        })
    }

    fn calculate_day_a(&self) -> usize {
        self.get_dial_positions()
            .iter()
            .filter(|x| **x % 100 == 0)
            .count()
    }

    fn calculate_zeros_in_rotation(&self, start: isize, end: isize) -> usize {
        let start_bucket = (start as f32 / 100.0).floor() as isize;
        let end_bucket = (end as f32 / 100.0).floor() as isize;
        (end_bucket - start_bucket).unsigned_abs()
        // make sure the 1 -> 0 case counts
            + if end % 100 == 0 && end < start { 1 } else { 0 }
        // exclude the 100 -> 99 case from counting
            - if start % 100 == 0 && end < start {
                1
            } else {
                0
            }
    }

    fn calculate_day_b(&self) -> usize {
        self.get_dial_positions()
            .windows(2)
            .map(|x| self.calculate_zeros_in_rotation(x[0], x[1]))
            .sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[test]
    fn test_calculate_day_a() {
        let day01 = Day01::new("data/test_data.txt").unwrap();
        let expected = 3;
        let actual = day01.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day01 = Day01::new("data/test_data.txt").unwrap();
        let expected = 6;
        let actual = day01.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day01 = Day01::new("data/input_data.txt").unwrap();
        let expected = 1139;
        let actual = day01.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day01 = Day01::new("data/input_data.txt").unwrap();
        let expected = 6684;
        let actual = day01.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[rstest]
    // Inputs from test day b
    #[case(50, -18, 1)]
    #[case(-18, -48,0)]
    #[case(-48,0,1)]
    #[case(0,-5,0)]
    #[case(-5, 55,1)]
    #[case(55, 0, 1)]
    #[case(0, -1 ,0)]
    #[case(-1, -100 ,1)]
    #[case(-100, -86,0)]
    #[case(-86, -168,1)]
    // some test cases I thought up
    #[case(0, 100, 1)]
    #[case(0, 1000, 10)]
    #[case(50, 55, 0)]
    #[case(0, 1, 0)]
    #[case(0, 99, 0)]
    #[case(1, 0, 1)]
    #[case(99, 0, 1)]
    #[case(199, 0, 2)]
    #[case(-1, 0, 1)]
    #[case(-1, 1, 1)]
    #[case(-99, 199, 2)]
    #[case(-101, 202, 4)]
    fn test_calculate_zeros_in_spin(
        #[case] start: isize,
        #[case] end: isize,
        #[case] expected: usize,
    ) {
        let day01 = Day01::new("data/input_data.txt").unwrap();
        let actual = day01.calculate_zeros_in_rotation(start, end);
        assert_eq!(expected, actual);
    }
}
