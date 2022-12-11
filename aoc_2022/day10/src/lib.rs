mod parser;
mod pixel_map;
use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};
use pixel_map::PixelMap;

#[derive(Clone)]
pub struct Day10 {
    instructions: Vec<Option<isize>>,
}

impl AOCCalculator for Day10 {
    fn new(filename: &str) -> Result<Day10, AOCFileOrParseError> {
        Ok(Day10 {
            instructions: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        let pixel_map = self.run_instructions();
        println!("{}a answer is {:?}", name, pixel_map.day_a_result);
        println!("{}b answer is:\n{}", name, pixel_map);
    }
}

impl Day10 {
    fn run_instructions(&self) -> PixelMap {
        let mut pixel_map = PixelMap::new();
        for instruction in self.instructions.iter() {
            pixel_map.process_instruction(instruction);
        }
        pixel_map
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day10 = Day10::new("data/test_data.txt").unwrap();
        let expected = 13140;
        let actual = day10.run_instructions();
        assert_eq!(expected, actual.day_a_result);
    }

    #[test]
    fn test_calculate_day_b() {
        let day10 = Day10::new("data/test_data.txt").unwrap();
        let mut expected: String = "".into();
        expected += "██  ██  ██  ██  ██  ██  ██  ██  ██  ██  \n";
        expected += "███   ███   ███   ███   ███   ███   ███ \n";
        expected += "████    ████    ████    ████    ████    \n";
        expected += "█████     █████     █████     █████     \n";
        expected += "██████      ██████      ██████      ████\n";
        expected += "███████       ███████       ███████     \n";
        let actual = day10.run_instructions();
        assert_eq!(expected, format!("{}", actual));
    }
}
