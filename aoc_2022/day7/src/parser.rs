extern crate peg;
use crate::types::TerminalLine;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day7_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse::<usize>().unwrap() }
    rule up_one_level() -> TerminalLine
        = ".." { TerminalLine::DecreaseDir }
    rule directory_or_file() -> TerminalLine
        = s:$(['a'..='z'|'.'|'/']+) { TerminalLine::IncreaseDir }
    rule change_dir() -> TerminalLine
        = "$ cd " directory:(up_one_level() / directory_or_file()) {
            directory
        }
    rule call_ls() -> TerminalLine
        = "$ ls" { TerminalLine::NoOp }
    rule list_file() -> TerminalLine
        = n:number() ( " " ) f:directory_or_file() { TerminalLine::File(n) }
    rule list_directory() -> TerminalLine
        = "dir " directory:directory_or_file() { TerminalLine::NoOp }
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

pub fn parse_data(input: &str) -> Result<Vec<usize>, AOCFileOrParseError> {
    if let Ok(terminal_lines) = day7_parser::parse(input) {
        Ok(build_sizes(&terminal_lines))
    } else {
        Err(AOCFileOrParseError)
    }
}

fn build_sizes(lines: &[TerminalLine]) -> Vec<usize> {
    let mut sizes_stack: Vec<usize> = vec![];
    let mut ret: Vec<usize> = Vec::new();
    for line in lines.iter() {
        match line {
            TerminalLine::NoOp => (),
            TerminalLine::IncreaseDir => sizes_stack.push(0),
            TerminalLine::DecreaseDir => {
                if let Some(current_size) = pop_and_merge_with_new_end(&mut sizes_stack) {
                    ret.push(current_size);
                }
            }
            TerminalLine::File(size) => {
                let len = sizes_stack.len();
                sizes_stack[len - 1] += size;
            }
        }
    }
    while let Some(size) = pop_and_merge_with_new_end(&mut sizes_stack) {
        ret.push(size)
    }
    ret
}

fn pop_and_merge_with_new_end(sizes_stack: &mut Vec<usize>) -> Option<usize> {
    if let Some(current_size) = sizes_stack.pop() {
        let len = sizes_stack.len();
        if len > 0 {
            sizes_stack[len - 1] += current_size;
        }
        Some(current_size)
    } else {
        None
    }
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
        assert_eq!(actual[0], TerminalLine::IncreaseDir);
    }
}
