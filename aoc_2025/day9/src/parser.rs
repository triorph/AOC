extern crate peg;
use aoc_helpers::point2d::Point2D;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day9_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().unwrap_or_else(|_| panic!("Was expecting a number string {}", n))}
    rule point() -> Point2D
        = x:number() "," y:number() { Point2D::from_usize(x, y) }
    pub rule parse() -> Vec<Point2D>
        = lines:point() ++ ("\n" +) "\n" * { lines }
}}

pub fn parse_data(input: &str) -> Result<Vec<Point2D>, AOCFileOrParseError> {
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
        let expected: Vec<Point2D> = vec![
            Point2D { x: 7, y: 1 },
            Point2D { x: 11, y: 1 },
            Point2D { x: 11, y: 7 },
            Point2D { x: 9, y: 7 },
            Point2D { x: 9, y: 5 },
            Point2D { x: 2, y: 5 },
            Point2D { x: 2, y: 3 },
            Point2D { x: 7, y: 3 },
        ];
        assert_eq!(expected, actual)
    }
}
