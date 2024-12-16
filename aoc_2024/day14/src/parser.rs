extern crate peg;
use aoc_helpers::{point2d::Point2D, AOCFileOrParseError};

peg::parser! { pub grammar day14_parser() for str {
    rule positive_number() -> isize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule negative_number() -> isize
        = "-" number:positive_number() { - number }
    rule number() -> isize
        = number:(positive_number() / negative_number()) { number }
    rule point() -> Point2D
        = x:number() "," y:number() { Point2D {x, y} }
    rule line() -> (Point2D, Point2D)
        = "p=" position:point() " v=" velocity:point() { (position, velocity) }
    pub rule parse() -> Vec<(Point2D, Point2D)>
        = lines:line() ++ ("\n" +) "\n" * { lines }
}}

pub fn parse_data(input: &str) -> Result<Vec<(Point2D, Point2D)>, AOCFileOrParseError> {
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
        let expected: Vec<(Point2D, Point2D)> = vec![
            (Point2D { x: 0, y: 4 }, Point2D { x: 3, y: -3 }),
            (Point2D { x: 6, y: 3 }, Point2D { x: -1, y: -3 }),
            (Point2D { x: 10, y: 3 }, Point2D { x: -1, y: 2 }),
            (Point2D { x: 2, y: 0 }, Point2D { x: 2, y: -1 }),
            (Point2D { x: 0, y: 0 }, Point2D { x: 1, y: 3 }),
            (Point2D { x: 3, y: 0 }, Point2D { x: -2, y: -2 }),
            (Point2D { x: 7, y: 6 }, Point2D { x: -1, y: -3 }),
            (Point2D { x: 3, y: 0 }, Point2D { x: -1, y: -2 }),
            (Point2D { x: 9, y: 3 }, Point2D { x: 2, y: 3 }),
            (Point2D { x: 7, y: 3 }, Point2D { x: -1, y: 2 }),
            (Point2D { x: 2, y: 4 }, Point2D { x: 2, y: -3 }),
            (Point2D { x: 9, y: 5 }, Point2D { x: -3, y: -3 }),
        ];
        assert_eq!(expected, actual)
    }
}
