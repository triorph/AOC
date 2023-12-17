#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Room {
    SplitterVertical,
    SplitterHorizontal,
    DiagonalForward,
    DiagonalBackward,
    Empty,
}
