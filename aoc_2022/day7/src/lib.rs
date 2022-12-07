mod parser;
mod types;
use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};
use std::collections::HashMap;
use types::SizeOrDir;

pub struct Day7 {
    directory_contents: HashMap<String, Vec<SizeOrDir>>,
}

impl AOCCalculator<usize> for Day7 {
    fn new(filename: &str) -> Result<Day7, AOCFileOrParseError> {
        Ok(Day7 {
            directory_contents: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn calculate_day_a(&self) -> usize {
        self.directory_contents
            .keys()
            .map(|directory| self.calculate_size(directory))
            .filter(|size| size <= &100000)
            .sum()
    }

    fn calculate_day_b(&self) -> usize {
        let total_size = self.calculate_size("/");
        self.directory_contents
            .keys()
            .map(|directory| self.calculate_size(directory))
            .filter(|size| 70000000 - total_size + size >= 30000000)
            .min()
            .unwrap()
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}
impl Day7 {
    fn calculate_size(&self, directory: &str) -> usize {
        let contents = self.directory_contents.get(directory).unwrap();
        contents
            .iter()
            .map(|content| match content {
                SizeOrDir::Size(size) => *size,
                SizeOrDir::Directory(next_directory) => {
                    let directory = directory.to_string() + "." + next_directory;
                    self.calculate_size(&directory)
                }
            })
            .sum::<usize>()
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
    fn test_build_directories() {
        let day7 = Day7::new("data/test_data.txt").unwrap();
        assert_eq!(day7.calculate_size("/"), 48381165);
        assert_eq!(day7.calculate_size("/.d"), 24933642);
        assert_eq!(day7.calculate_size("/.a"), 94853);
        assert_eq!(day7.calculate_size("/.a.e"), 584);
    }
}
