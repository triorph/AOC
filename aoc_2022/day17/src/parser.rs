extern crate peg;
use crate::types::Direction;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day17_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule left() -> Direction
        = "<" { Direction::Left }
    rule right() -> Direction
        = ">" { Direction::Right }
    rule direction() -> Direction
        = d:(left() / right()) { d }
    pub rule parse() -> Vec<Direction>
        = directions:direction() ++ ("") "\n" * {
             { directions }
        }
}}

pub fn parse_data(input: &str) -> Result<Vec<Direction>, AOCFileOrParseError> {
    if let Ok(ret) = day17_parser::parse(input) {
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
        let actual = day17_parser::parse(&input_str).expect("Should parse successfully");
        assert_eq!(actual.len(), 40);
    }
}
