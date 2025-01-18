mod parser;
use std::{cmp::Reverse, collections::BinaryHeap};

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

    fn get_largest_a_that_prints_instructions(&self) -> usize {
        // we want to look backwards for this
        //
        // Best to focus on the real case, instead of anything generic
        //
        // 2,4, -- set B to A % 8
        // 1,2, -- XOR B by 2
        // 7,5, -- Set C to A >> B
        // 4,5, -- XOR B by C
        // 1,3, -- XOR B by 3
        // 5,5, -- output the contents of B % 8
        // 0,3, -- right-shift A by 3 (divide by 8)
        // 3,0  -- branch to start if A is not zero
        //
        // The combo operand for 3 is 3, so basically we just right-shift A by 3
        // for every comma we have (15)
        // so we only need to consider A around 1<<45 to 1<<51
        //
        // We have a 5,5 for out only output, which means
        // we need to print register B mod 8 each time.
        //
        // since deshift at the start will be 0, we can work out what it will be
        // at each time
        //
        // so basically, we need every outcome to be bits to be (A XOR 1) XOR (A >> (A XOR 2))
        // assuming that A is 8-bit except for A >> (A XOR 2) also ends up as an 8-bit result
        //
        // the easiest way to do this is to do an explore algorithm on possible A values,
        // matching the instruction_commands to the instructions (but in reverse).
        // Then when going to the next node, we shift up by 3 and try another 8 instructions
        // on all that match.
        //
        // note:
        // test instructions are:
        // 0,3, right-shift A by 3
        // 5,4, output the contents of A % 8
        // 3,0, jump back to A if not empty
        // so basically just the smallest number that satisfies the original numbers
        // concatenated together as 3-bit arrays
        let mut obj = self.clone();
        let mut to_explore: BinaryHeap<(usize, Reverse<usize>)> = BinaryHeap::new();
        to_explore.push((0, Reverse(0)));
        while let Some((depth, Reverse(value))) = to_explore.pop() {
            if depth == self.instructions.len() {
                return value;
            }
            for i in 0..8 {
                let new_a = (value << 3) + i;
                let ret = obj.run_all_instructions_and_get_output_b(new_a);
                let expect = self.instructions[self.instructions.len() - depth - 1];
                if ret == expect {
                    to_explore.push((depth + 1, Reverse(new_a)));
                }
            }
        }
        panic!("No day b value found")
    }

    fn calculate_day_b(&self) -> usize {
        self.get_largest_a_that_prints_instructions()
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

    fn run_all_instructions_and_get_output_b(&mut self, starting_a: usize) -> usize {
        self.registers = vec![starting_a, 0, 0];
        for operator_operand in self.instructions.chunks(2) {
            let operator = operator_operand[0];
            let operand = operator_operand[1];
            match operator {
                0 => self.registers[0] = self.get_adv_result(operand),
                1 => self.registers[1] ^= operand,
                2 => self.registers[1] = self.get_combo_operand(operand) % 8,
                3 => (),
                4 => self.registers[1] ^= self.registers[2],
                5 => return self.get_combo_operand(operand) % 8,
                6 => self.registers[1] = self.get_adv_result(operand),
                7 => self.registers[2] = self.get_adv_result(operand),
                _ => panic!("Invalid operator {:?}", operator),
            }
        }
        panic!("No output returned")
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
        let expected = 37221270076916;
        let actual = day17.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
