mod parser;
mod stack_set;
use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};
use stack_set::{Instruction, StackSet};

pub struct Day5 {
    stack_set: StackSet,
    instructions: Vec<Instruction>,
}

impl AOCCalculator for Day5 {
    fn new(filename: &str) -> Result<Day5, AOCFileOrParseError> {
        let (stack_set, move_list) = parse_data(&read_input_file(filename)?)?;
        Ok(Day5 {
            stack_set: StackSet::new(stack_set),
            instructions: move_list,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {}", name, self.calculate_day_a());
        println!("{}b answer is {}", name, self.calculate_day_b());
    }
}

impl Day5 {
    fn calculate_day_a(&self) -> String {
        let mut stack_set = self.stack_set.clone();
        stack_set.process_moves_a(&self.instructions);
        stack_set.show_top_values()
    }

    fn calculate_day_b(&self) -> String {
        let mut stack_set = self.stack_set.clone();
        stack_set.process_moves_b(&self.instructions);
        stack_set.show_top_values()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day5 = Day5::new("data/test_data.txt").unwrap();
        let expected: String = "CMZ".into();
        let actual = day5.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day5 = Day5::new("data/test_data.txt").unwrap();
        let expected: String = "MCD".into();
        let actual = day5.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
