extern crate peg;

use crate::pipe::Pipe;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day9_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule top_bottom() -> Pipe
        = "|" { Pipe::TopBottom }
    rule left_right() -> Pipe
        = "-" { Pipe::LeftRight }
    rule top_left() -> Pipe
        = "J" { Pipe::TopLeft }
    rule bottom_left() -> Pipe
        = "7" { Pipe::BottomLeft }
    rule top_right() -> Pipe
        = "L" { Pipe::TopRight }
    rule bottom_right() -> Pipe
        = "F" { Pipe::BottomRight }
    rule empty() -> Pipe
        = "." { Pipe::Empty }
    rule start() -> Pipe
        = "S" { Pipe::Start }
    rule pipe() -> Pipe
        = pipe:(top_bottom() / left_right() / top_left() /
                bottom_left() / top_right() / bottom_right() /
                empty() / start()) {
            pipe
        }
    rule line() -> Vec<Pipe>
        = line:pipe() ++ ""
    pub rule parse() -> Vec<Vec<Pipe>>
        = lines:line() ++ ("\n" +) "\n" * { lines }
}}

pub fn parse_data(input: &str) -> Result<Vec<Vec<Pipe>>, AOCFileOrParseError> {
    if let Ok(ret) = day9_parser::parse(input) {
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
        let actual = day9_parser::parse(&input_str).expect("Should parse successfully");
        assert_eq!(5, actual.len())
    }
}
