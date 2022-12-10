mod parser;
mod pixel_map;
use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};
use pixel_map::PixelMap;

#[derive(Clone)]
pub struct Day10 {
    instructions: Vec<Option<isize>>,
    x: isize,
    cycle: usize,
}

impl AOCCalculator<isize> for Day10 {
    fn new(filename: &str) -> Result<Day10, AOCFileOrParseError> {
        Ok(Day10 {
            instructions: parse_data(&read_input_file(filename)?)?,
            x: 1,
            cycle: 1,
        })
    }

    fn calculate_day_a(&self) -> isize {
        let mut obj = self.clone();
        let mut target = 20;
        let mut ret = 0;
        for instruction in self.instructions.iter() {
            let x = obj.process_instruction(instruction, target);
            if obj.cycle >= target && target <= 220 {
                ret += target as isize * x;
                target += 40;
            }
        }

        ret
    }

    fn calculate_day_b(&self) -> isize {
        let mut obj = self.clone();
        let mut pixel_map = PixelMap::new();
        for instruction in self.instructions.iter() {
            obj.process_instruction_day_b(instruction, &mut pixel_map);
            println!("obj.x: {:?}", obj.x);
        }
        println!("Day b drawing is:\n{}", pixel_map);
        0
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day10 {
    fn process_instruction(&mut self, instruction: &Option<isize>, target: usize) -> isize {
        match instruction {
            Some(x) => {
                self.x += x;
                self.cycle += 2;
                if self.cycle == target + 1 {
                    self.x - x
                } else {
                    self.x
                }
            }
            None => {
                self.cycle += 1;
                self.x
            }
        }
    }

    fn process_instruction_day_b(&mut self, instruction: &Option<isize>, pixel_map: &mut PixelMap) {
        match instruction {
            Some(x) => {
                pixel_map.draw_current(self.x);
                pixel_map.draw_current(self.x);
                self.x += x;
                self.cycle += 2;
            }
            None => {
                pixel_map.draw_current(self.x);
                self.cycle += 1;
            }
        }
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
        let actual = day10.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day10 = Day10::new("data/test_data.txt").unwrap();
        let expected = 0;
        let actual = day10.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
