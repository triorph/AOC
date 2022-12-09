extern crate peg;
use crate::types::Point;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day9_parser() for str {
    rule number() -> isize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule up() -> Point
        = "U " n:number() { Point(0, -n) }
    rule down() -> Point
        = "D " n:number() { Point(0, n) }
    rule left() -> Point
        = "L " n:number() { Point(-n, 0) }
    rule right() -> Point
        = "R " n:number() { Point(n, 0) }
    rule direction() -> Point
        = p:(up() / down() / left() / right()) { p }
    pub rule parse() -> Vec<Point>
        = lines_of_directions:direction() ++ ("\n")  "\n" * {
             { lines_of_directions }
        }
}}

pub fn parse_data(input: &str) -> Result<Vec<Point>, AOCFileOrParseError> {
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
        assert_eq!(actual.len(), 8);
    }
}
