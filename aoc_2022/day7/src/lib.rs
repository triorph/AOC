mod parser;
mod types;
use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};

pub struct Day7 {
    directory_sizes: Vec<usize>,
}

impl AOCCalculator for Day7 {
    fn new(filename: &str) -> Result<Day7, AOCFileOrParseError> {
        Ok(Day7 {
            directory_sizes: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day7 {
    fn calculate_day_a(&self) -> usize {
        self.directory_sizes
            .iter()
            .filter(|&&size| size <= 100000)
            .sum()
    }

    fn calculate_day_b(&self) -> usize {
        let currently_left: usize = 70000000 - *self.directory_sizes.iter().max().unwrap();
        *self
            .directory_sizes
            .iter()
            .filter(|&&size| currently_left + size >= 30000000)
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day7 = Day7::new("data/test_data.txt").unwrap();
        let expected = 95437;
        let actual = day7.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day7 = Day7::new("data/test_data.txt").unwrap();
        let expected = 24933642;
        let actual = day7.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sizes() {
        let day7 = Day7::new("data/test_data.txt").unwrap();
        assert!(day7.directory_sizes.contains(&48381165));
        assert!(day7.directory_sizes.contains(&584));
        assert!(day7.directory_sizes.contains(&24933642));
        assert!(day7.directory_sizes.contains(&94853));
    }
}
