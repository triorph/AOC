extern crate peg;
use crate::point::Point;
use crate::wall::Wall;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day14_parser() for str {
    rule positive_number() -> isize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule negative_number() -> isize
        = "-" n:positive_number() { -n }
    rule number() -> isize
        = n:(negative_number() / positive_number()) { n }
    rule point() -> Point
        = x:number() "," y:number() { Point{x, y} }
    rule wall() -> Wall
        = points:point() ++ " -> " { Wall::new(&points) }
    pub rule parse() -> Vec<Wall>
        = walls:wall() ++ ("\n") "\n" * {
             { walls }
        }
}}

pub fn parse_data(input: &str) -> Result<Vec<Wall>, AOCFileOrParseError> {
    if let Ok(ret) = day14_parser::parse(input) {
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
        let actual = day14_parser::parse(&input_str).expect("Should parse successfully");
        assert_eq!(actual.len(), 2);
    }
}
