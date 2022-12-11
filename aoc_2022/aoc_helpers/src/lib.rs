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

pub fn gcd_pair(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd_pair(b, a % b)
    }
}

fn gcd(vals: &[usize]) -> usize {
    vals.iter()
        .map(|x| *x)
        .reduce(|a: usize, b: usize| gcd_pair(a, b))
        .unwrap()
}

pub fn lcm(vals: &[usize]) -> usize {
    println!("vals: {:?}", vals);
    let gcd: usize = gcd(vals);
    println!("gcd: {:?}", gcd);
    let lcm = vals
        .iter()
        .copied()
        .reduce(|a: usize, b: usize| (a * b / gcd))
        .unwrap();
    println!("lcm: {:?}", lcm);
    lcm
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
