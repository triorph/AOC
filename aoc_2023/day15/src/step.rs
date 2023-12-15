use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Step {
    pub label: String,
    pub operation: Operation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Equals(usize),
    Subtract,
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Operation::Equals(val) => "=".to_string() + &val.to_string(),
                Operation::Subtract => "-".to_string(),
            }
        )
    }
}

impl Step {
    pub fn to_day_a_string(&self) -> String {
        self.label.clone() + &self.operation.to_string()
    }
}

pub trait AOCHash {
    fn get_aoc_hash(&self) -> usize;
}

impl AOCHash for String {
    fn get_aoc_hash(&self) -> usize {
        self.chars().fold(0, |value: usize, next: char| {
            ((value + (next as usize)) * 17) % 256
        })
    }
}
