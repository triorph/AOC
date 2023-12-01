use std::fs::read_to_string;
pub mod hash_utils;
pub mod modular_math;

#[derive(Debug)]
pub struct AOCFileOrParseError;

impl std::error::Error for AOCFileOrParseError {}

impl std::fmt::Display for AOCFileOrParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

pub trait AOCCalculator {
    fn new(input_file: &str) -> Result<Self, AOCFileOrParseError>
    where
        Self: std::marker::Sized;
    fn print_results(&self, name: &str);
}

pub fn read_input_file(filename: &str) -> Result<String, AOCFileOrParseError> {
    if let Ok(ret) = read_to_string(filename) {
        Ok(ret)
    } else {
        Err(AOCFileOrParseError)
    }
}
