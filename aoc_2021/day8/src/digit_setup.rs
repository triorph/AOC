use crate::digit_line::DigitLine;
use crate::parser::day8_parser;

pub struct DigitSetup {
    pub digit_lines: Vec<DigitLine>,
}

impl DigitSetup {
    /// Creates a new DigitSetup object from the string values of the input test data
    pub fn new(digit_setup_input_str: &str) -> DigitSetup {
        day8_parser::parse(digit_setup_input_str).unwrap()
    }

    /// Calculate the quantity of digits in the outputs that can be derived on their
    /// segment length alone, e.g. 1, 4, 7 and 8 each uniquely only have 2, 4, 3 and 7 segments
    /// each.
    pub fn calculate_day_a(self: &DigitSetup) -> usize {
        self.digit_lines
            .iter()
            .map(|digit_line| digit_line.count_simple_digits())
            .sum()
    }

    /// For each digit line, work out what the output number is supposed to be, then sum all digit
    /// line outputs
    pub fn calculate_day_b(self: &mut DigitSetup) -> usize {
        self.digit_lines
            .iter_mut()
            .map(|digit_line| digit_line.calculate_output())
            .sum()
    }
}
