#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Operator {
    Add,
    Multiply,
}

impl Operator {
    pub fn reduce(&self, a: usize, b: usize) -> usize {
        match self {
            Operator::Add => a + b,
            Operator::Multiply => a * b,
        }
    }
}
