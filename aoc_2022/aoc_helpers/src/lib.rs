use std::collections::HashSet;
use std::fs::read_to_string;
use std::hash::Hash;

#[derive(Debug)]
pub struct AOCFileOrParseError;

pub fn hashset_from_vec<T>(v: &[T]) -> HashSet<T>
where
    T: Eq + Hash + Copy,
{
    HashSet::from_iter(v.iter().copied())
}

impl std::error::Error for AOCFileOrParseError {}

impl std::fmt::Display for AOCFileOrParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

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
