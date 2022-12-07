#[derive(Debug, PartialEq, Eq)]
pub enum TerminalLine {
    IncreaseDir,
    DecreaseDir,
    File(usize),
    NoOp,
}
