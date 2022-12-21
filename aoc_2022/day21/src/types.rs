#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Operation {
    Add(String, String),
    Subtract(String, String),
    Multiply(String, String),
    Divide(String, String),
    Literal(isize),
}
