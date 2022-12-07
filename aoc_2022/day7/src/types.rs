#[derive(Debug, PartialEq, Eq)]
pub enum TerminalLine {
    ChangeDir(Option<String>),
    File(usize, String),
    Directory(String),
    NoOp,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SizeOrDir {
    Size(usize),
    Directory(String),
}
