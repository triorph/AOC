#[derive(Debug, Eq, PartialEq)]
pub enum Instruction {
    RotateLeft,
    RotateRight,
    MoveForward(usize),
}
