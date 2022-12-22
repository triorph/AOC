mod instruction;
mod map;
mod parser;
use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};

use self::{instruction::Instruction, map::Map};

pub struct Day22 {
    map: Map,
    instructions: Vec<Instruction>,
}

impl AOCCalculator for Day22 {
    fn new(filename: &str) -> Result<Day22, AOCFileOrParseError> {
        let (map, instructions) = parse_data(&read_input_file(filename)?)?;
        Ok(Day22 { map, instructions })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day22 {
    fn calculate_day_a(&self) -> usize {
        let mut position = self.map.get_starting_position();
        for instruction in self.instructions.iter() {
            position.run_instruction(&self.map, instruction, false);
        }
        position.get_password()
    }

    fn calculate_day_b(&self) -> usize {
        let mut position = self.map.get_starting_position();
        for instruction in self.instructions.iter() {
            position.run_instruction(&self.map, instruction, true);
        }
        position.get_password()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day22 = Day22::new("data/test_data.txt").unwrap();
        let expected = 6032;
        let actual = day22.calculate_day_a();
        assert_eq!(expected, actual);
    }

    // #[test]
    // fn test_calculate_day_b() {
    //     let day22 = Day22::new("data/test_data.txt").unwrap();
    //     let expected = 0;
    //     let actual = day22.calculate_day_b();
    //     assert_eq!(expected, actual);
    // }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day22 = Day22::new("data/input_data.txt").unwrap();
        let expected = 64256;
        let actual = day22.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day22 = Day22::new("data/input_data.txt").unwrap();
        let expected = 109224;
        let actual = day22.calculate_day_b();
        assert!(actual > 61360, "{} not greater than 61360", actual);
        assert_eq!(expected, actual);
    }
}
