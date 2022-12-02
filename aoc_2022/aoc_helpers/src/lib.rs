use std::fs::read_to_string;

#[derive(Debug)]
pub struct AOCFileOrParseError;

pub trait AOCCalculator<Answer> {
    fn new(input_file: &str) -> Result<Self, AOCFileOrParseError>
    where
        Self: std::marker::Sized;
    fn calculate_day_a(&self) -> Answer;
    fn calculate_day_b(&self) -> Answer;
    fn print_results(&self, name: &str);
}

pub fn read_input_file(filename: &str) -> Result<String, AOCFileOrParseError> {
    if let Ok(ret) = read_to_string(filename) {
        Ok(ret)
    } else {
        Err(AOCFileOrParseError)
    }
}
