mod parser;
use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day17 {
    instructions: Vec<usize>,
    registers: Vec<usize>,
    output: Vec<usize>,
}

impl AOCCalculator for Day17 {
    fn new(filename: &str) -> Result<Day17, AOCFileOrParseError> {
        let (registers, instructions) = parse_data(&read_input_file(filename)?)?;
        Ok(Day17 {
            instructions,
            registers,
            output: Vec::new(),
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.clone().calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day17 {
    fn calculate_day_a(&mut self) -> String {
        let mut instruction_pointer = 0;
        let instruction_chunks: Vec<Vec<usize>> = self
            .instructions
            .chunks(2)
            .map(|chunk| vec![chunk[0], chunk[1]])
            .collect();
        while instruction_pointer < instruction_chunks.len() {
            instruction_pointer = self.run_instruction(
                instruction_chunks[instruction_pointer][0],
                instruction_chunks[instruction_pointer][1],
                instruction_pointer,
            );
        }

        self.output
            .iter()
            .map(|val| val.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    fn calculate_day_b(&self) -> usize {
        let mut ret = 0;
        loop {
            let mut obj = self.clone();
            obj.registers[0] = ret;
            obj.calculate_day_a();
            if obj.output == self.instructions {
                return ret;
            } else {
                ret += 1;
            }
        }
    }

    fn get_combo_operand(&self, operand: usize) -> usize {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.registers[0],
            5 => self.registers[1],
            6 => self.registers[2],
            _ => panic!("Invalid operand"),
        }
    }

    fn get_adv_result(&self, operand: usize) -> usize {
        let numerator = self.registers[0];
        let combo_operand = self.get_combo_operand(operand);
        numerator >> combo_operand
    }

    fn run_instruction(
        &mut self,
        operator: usize,
        operand: usize,
        instruction_pointer: usize,
    ) -> usize {
        match operator {
            0 => self.registers[0] = self.get_adv_result(operand),
            1 => self.registers[1] ^= operand,
            2 => self.registers[1] = self.get_combo_operand(operand) % 8,
            3 => {
                if self.registers[0] != 0 {
                    return operand / 2;
                }
            }
            4 => self.registers[1] ^= self.registers[2],
            5 => self.output.push(self.get_combo_operand(operand) % 8),
            6 => self.registers[1] = self.get_adv_result(operand),
            7 => self.registers[2] = self.get_adv_result(operand),
            _ => panic!("Invalid operator {:?}", operator),
        }
        instruction_pointer + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let mut day17 = Day17::new("data/test_data.txt").unwrap();
        let expected = "4,6,3,5,6,3,5,2,1,0";
        let actual = day17.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day17 = Day17::new("data/test_data_2.txt").unwrap();
        let expected = 117440;
        let actual = day17.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn advanced_test_cases() {
        let mut obj = Day17 {
            registers: vec![0, 0, 9],
            instructions: vec![2, 6],
            output: vec![],
        };
        obj.calculate_day_a();
        assert_eq!(obj.registers, vec![0, 1, 9]);
        let mut obj = Day17 {
            registers: vec![10, 0, 0],
            instructions: vec![5, 0, 5, 1, 5, 4],
            output: vec![],
        };
        obj.calculate_day_a();
        assert_eq!(obj.output, vec![0, 1, 2]);
        let mut obj = Day17 {
            registers: vec![2024, 0, 0],
            instructions: vec![0, 1, 5, 4, 3, 0],
            output: vec![],
        };
        obj.calculate_day_a();
        assert_eq!(obj.output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(obj.registers, [0, 0, 0]);
        let mut obj = Day17 {
            registers: vec![0, 29, 0],
            instructions: vec![1, 7],
            output: vec![],
        };
        obj.calculate_day_a();
        assert_eq!(obj.registers, [0, 26, 0]);
        let mut obj = Day17 {
            registers: vec![0, 2024, 43690],
            instructions: vec![4, 0],
            output: vec![],
        };
        obj.calculate_day_a();
        assert_eq!(obj.registers, [0, 44354, 43690]);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let mut day17 = Day17::new("data/input_data.txt").unwrap();
        let expected = "3,1,4,3,1,7,1,6,3";
        let actual = day17.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day17 = Day17::new("data/input_data.txt").unwrap();
        let expected = 0;
        let actual = day17.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
