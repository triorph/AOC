mod parser;
use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};

pub struct Day6 {
    data: String,
}

impl AOCCalculator<usize> for Day6 {
    fn new(filename: &str) -> Result<Day6, AOCFileOrParseError> {
        Ok(Day6 {
            data: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn calculate_day_a(&self) -> usize {
        find_time_until_n_different(&self.data, 4)
    }

    fn calculate_day_b(&self) -> usize {
        find_time_until_n_different(&self.data, 14)
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

fn find_time_until_n_different(data: &str, n: usize) -> usize {
    let mut count = n;
    let mut chars = data.chars();
    while chars.clone().count() >= n {
        let testable = chars.clone().take(n).collect::<Vec<char>>();
        // println!("testable: {:?}", testable);
        if test_all_different(&testable, n) {
            return count;
        } else {
            count += 1;
            chars.next();
        }
    }
    panic!("Shouldn't get here");
}

fn test_all_different(vals: &[char], n: usize) -> bool {
    for val in vals.iter() {
        if vals.iter().filter(|v| v == &val).count() != 1 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day6 = Day6::new("data/test_data.txt").unwrap();
        let expected = 7;
        let actual = day6.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day6 = Day6::new("data/test_data.txt").unwrap();
        let expected = 0;
        let actual = day6.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
