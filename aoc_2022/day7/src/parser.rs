extern crate peg;
use crate::types::{SizeOrDir, TerminalLine};
use aoc_helpers::AOCFileOrParseError;
use std::collections::HashMap;

peg::parser! { pub grammar day7_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse::<usize>().unwrap() }
    rule up_one_level() -> Option<String>
        = ".." { None }
    rule directory_or_file() -> Option<String>
        = s:$(['a'..='z'|'.'|'/']+) { Some(s.to_string()) }
    rule change_dir() -> TerminalLine
        = "$ cd " directory:(up_one_level() / directory_or_file()) {
            TerminalLine::ChangeDir(directory)
        }
    rule call_ls() -> TerminalLine
        = "$ ls" { TerminalLine::NoOp }
    rule list_file() -> TerminalLine
        = n:number() ( " " ) f:directory_or_file() {
            TerminalLine::File(n, f.unwrap())
        }
    rule list_directory() -> TerminalLine
        = "dir " directory:directory_or_file() { TerminalLine::Directory(directory.unwrap()) }
    rule list() -> TerminalLine
        = file_or_directory:(list_file() / list_directory()) {
            file_or_directory
        }
    rule instruction() -> TerminalLine
        = instruction:(change_dir() / call_ls() / list()) { instruction }
    pub rule parse() -> Vec<TerminalLine>
        = line_of_instructions:instruction() ++ ("\n") "\n" * {
             { line_of_instructions }
        }
}}

pub fn parse_data(input: &str) -> Result<HashMap<String, Vec<SizeOrDir>>, AOCFileOrParseError> {
    if let Ok(ret) = day7_parser::parse(input) {
        Ok(build_directories(&ret))
    } else {
        Err(AOCFileOrParseError)
    }
}

pub fn build_directories(lines: &[TerminalLine]) -> HashMap<String, Vec<SizeOrDir>> {
    let mut directory_stack: Vec<String> = Vec::new();
    let mut dir_contents: HashMap<String, Vec<SizeOrDir>> = HashMap::new();
    for line in lines.iter() {
        match line {
            TerminalLine::NoOp => (),
            TerminalLine::ChangeDir(Some(dir)) => {
                directory_stack.push(dir.clone());
            }
            TerminalLine::ChangeDir(None) => {
                directory_stack.pop();
            }
            TerminalLine::Directory(dir) => {
                let current_dir = directory_stack.join(".");
                dir_contents
                    .entry(current_dir.clone())
                    .and_modify(|contents| contents.push(SizeOrDir::Directory(dir.clone())))
                    .or_insert_with(|| vec![SizeOrDir::Directory(dir.clone())]);
            }
            TerminalLine::File(size, _) => {
                let current_dir = directory_stack.join(".");
                dir_contents
                    .entry(current_dir.clone())
                    .and_modify(|contents| contents.push(SizeOrDir::Size(*size)))
                    .or_insert_with(|| vec![SizeOrDir::Size(*size)]);
            }
        }
    }
    dir_contents
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    use aoc_helpers::read_input_file;
    #[test]
    fn test_parse() {
        let input_str = read_input_file("data/test_data.txt").unwrap();
        let actual = day7_parser::parse(&input_str).expect("Should parse successfully");
        assert_eq!(actual[0], TerminalLine::ChangeDir(Some("/".to_string())));
    }
}
