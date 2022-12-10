mod parser;
mod pixel_map;
use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};
use pixel_map::PixelMap;

#[derive(Clone)]
pub struct Day10 {
    instructions: Vec<Option<isize>>,
    x: isize,
    pixel_map: PixelMap,
    target: usize,
    day_a_result: usize,
}

impl AOCCalculator for Day10 {
    fn new(filename: &str) -> Result<Day10, AOCFileOrParseError> {
        Ok(Day10 {
            instructions: parse_data(&read_input_file(filename)?)?,
            x: 1,
            pixel_map: PixelMap::new(),
            target: 20,
            day_a_result: 0,
        })
    }

    fn print_results(&self, name: &str) {
        let mut obj = self.clone();
        println!("{}a answer is {:?}", name, obj.calculate_day_a());
        println!("{}b answer is:\n{}", name, obj.pixel_map);
    }
}

impl Day10 {
    fn calculate_day_a(&mut self) -> usize {
        for instruction in self.instructions.clone().iter() {
            self.process_instruction(instruction);
        }

        self.day_a_result
    }

    fn calculate_day_b(&mut self) -> String {
        for instruction in self.instructions.clone().iter() {
            self.process_instruction(instruction);
        }
        format!("{}", self.pixel_map)
    }

    fn increase_cycle(&mut self) {
        self.pixel_map.draw_next_pixel(self.x);
        if self.pixel_map.cycle >= self.target && self.target <= 240 {
            self.day_a_result += self.x as usize * self.target;
            self.target += 40;
        }
    }

    fn process_instruction(&mut self, instruction: &Option<isize>) {
        match instruction {
            Some(x) => {
                self.increase_cycle();
                self.increase_cycle();
                self.x += x;
            }
            None => {
                self.increase_cycle();
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
        let mut expected: String = "".into();
        expected += "##..##..##..##..##..##..##..##..##..##..\n";
        expected += "###...###...###...###...###...###...###.\n";
        expected += "####....####....####....####....####....\n";
        expected += "#####.....#####.....#####.....#####.....\n";
        expected += "######......######......######......####\n";
        expected += "#######.......#######.......#######.....\n";
        let actual = day10.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
