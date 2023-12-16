extern crate peg;
use aoc_helpers::AOCFileOrParseError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Room {
    SplitterVertical,
    SplitterHorizontal,
    DiagonalForward,
    DiagonalBackward,
    Empty,
}

peg::parser! { pub grammar day16_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule splitter_vertical() -> Room
        = "|" { Room::SplitterVertical }
    rule splitter_horizontal() -> Room
        = "-" { Room::SplitterHorizontal }
    rule diagonal_forward() -> Room
        = "/" { Room::DiagonalForward }
    rule diagonal_backward() -> Room
        = "\\" { Room::DiagonalBackward }
    rule empty() -> Room
        = "." { Room::Empty }
    rule room() -> Room
        = room:(splitter_vertical() / splitter_horizontal() / diagonal_forward() / diagonal_backward() / empty()) { room }
    rule line() -> Vec<Room>
        = line:room() ++ "" { line }
    pub rule parse() -> Vec<Vec<Room>>
        = lines:line() ++ ("\n" +) "\n" * { lines }
}}

pub fn parse_data(input: &str) -> Result<Vec<Vec<Room>>, AOCFileOrParseError> {
    if let Ok(ret) = day16_parser::parse(input) {
        Ok(ret)
    } else {
        Err(AOCFileOrParseError)
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
        let actual = day16_parser::parse(&input_str).expect("Should parse successfully");
        let expected: Vec<Vec<Room>> = vec![
            vec![
                Room::Empty,
                Room::SplitterVertical,
                Room::Empty,
                Room::Empty,
                Room::Empty,
                Room::DiagonalBackward,
                Room::Empty,
                Room::Empty,
                Room::Empty,
                Room::Empty,
            ],
            vec![
                Room::SplitterVertical,
                Room::Empty,
                Room::SplitterHorizontal,
                Room::Empty,
                Room::DiagonalBackward,
                Room::Empty,
                Room::Empty,
                Room::Empty,
                Room::Empty,
                Room::Empty,
            ],
            vec![
                Room::Empty,
                Room::Empty,
                Room::Empty,
                Room::Empty,
                Room::Empty,
                Room::SplitterVertical,
                Room::SplitterHorizontal,
                Room::Empty,
                Room::Empty,
                Room::Empty,
            ],
            vec![
                Room::Empty,
                Room::Empty,
                Room::Empty,
                Room::Empty,
                Room::Empty,
                Room::Empty,
                Room::Empty,
                Room::Empty,
                Room::SplitterVertical,
                Room::Empty,
            ],
            vec![
                Room::Empty,
                Room::Empty,
                Room::Empty,
                Room::Empty,
                Room::Empty,
                Room::Empty,
                Room::Empty,
                Room::Empty,
                Room::Empty,
                Room::Empty,
            ],
            vec![
                Room::Empty,
                Room::Empty,
                Room::Empty,
                Room::Empty,
                Room::Empty,
                Room::Empty,
                Room::Empty,
                Room::Empty,
                Room::Empty,
                Room::DiagonalBackward,
            ],
            vec![
                Room::Empty,
                Room::Empty,
                Room::Empty,
                Room::Empty,
                Room::DiagonalForward,
                Room::Empty,
                Room::DiagonalBackward,
                Room::DiagonalBackward,
                Room::Empty,
                Room::Empty,
            ],
            vec![
                Room::Empty,
                Room::SplitterHorizontal,
                Room::Empty,
                Room::SplitterHorizontal,
                Room::DiagonalForward,
                Room::Empty,
                Room::Empty,
                Room::SplitterVertical,
                Room::Empty,
                Room::Empty,
            ],
            vec![
                Room::Empty,
                Room::SplitterVertical,
                Room::Empty,
                Room::Empty,
                Room::Empty,
                Room::Empty,
                Room::SplitterHorizontal,
                Room::SplitterVertical,
                Room::Empty,
                Room::DiagonalBackward,
            ],
            vec![
                Room::Empty,
                Room::Empty,
                Room::DiagonalForward,
                Room::DiagonalForward,
                Room::Empty,
                Room::SplitterVertical,
                Room::Empty,
                Room::Empty,
                Room::Empty,
                Room::Empty,
            ],
        ];
        assert_eq!(expected, actual)
    }
}
