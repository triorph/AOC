extern crate peg;

use aoc_helpers::{point2d::Point2D, AOCFileOrParseError};

peg::parser! { pub grammar day18_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule point() -> Point2D
        = x:number() "," y:number() { Point2D::from_usize(x, y) }
    pub rule parse() -> Vec<Point2D>
        = points:point() ++ ("\n" +) "\n" * { points }
}}

pub fn parse_data(input: &str) -> Result<Vec<Point2D>, AOCFileOrParseError> {
    if let Ok(ret) = day18_parser::parse(input) {
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
        let actual = day18_parser::parse(&input_str).expect("Should parse successfully");
        let expected: Vec<Point2D> = vec![
            Point2D::from_usize(5, 4),
            Point2D::from_usize(4, 2),
            Point2D::from_usize(4, 5),
            Point2D::from_usize(3, 0),
            Point2D::from_usize(2, 1),
            Point2D::from_usize(6, 3),
            Point2D::from_usize(2, 4),
            Point2D::from_usize(1, 5),
            Point2D::from_usize(0, 6),
            Point2D::from_usize(3, 3),
            Point2D::from_usize(2, 6),
            Point2D::from_usize(5, 1),
            Point2D::from_usize(1, 2),
            Point2D::from_usize(5, 5),
            Point2D::from_usize(2, 5),
            Point2D::from_usize(6, 5),
            Point2D::from_usize(1, 4),
            Point2D::from_usize(0, 4),
            Point2D::from_usize(6, 4),
            Point2D::from_usize(1, 1),
            Point2D::from_usize(6, 1),
            Point2D::from_usize(1, 0),
            Point2D::from_usize(0, 5),
            Point2D::from_usize(1, 6),
            Point2D::from_usize(2, 0),
        ];
        assert_eq!(expected, actual)
    }
}
