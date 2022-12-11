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

pub fn greatest_common_divisor(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        greatest_common_divisor(b, a % b)
    }
}

pub fn least_common_multiple(a: usize, b: usize) -> usize {
    a * (b / greatest_common_divisor(a, b))
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
