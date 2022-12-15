extern crate peg;
use crate::point::Point;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day15_parser() for str {
    rule positive_number() -> isize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule negative_number() -> isize
        = "-" n:positive_number() { -n }
    rule number() -> isize
        = n:(negative_number() / positive_number()) { n }
    rule line() -> (Point, Point)
        = "Sensor at x=" x1:number() ", y=" y1:number() ": closest beacon is at x=" x2:number() ", y=" y2:number() {
            (Point{x: x1, y: y1}, Point{x: x2, y: y2})
        }
    pub rule parse() -> Vec<(Point, Point)>
        = lines:line() ++ ("\n") "\n" * {
             { lines }
        }
}}

pub fn parse_data(input: &str) -> Result<Vec<(Point, Point)>, AOCFileOrParseError> {
    if let Ok(ret) = day15_parser::parse(input) {
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
        let actual = day15_parser::parse(&input_str).expect("Should parse successfully");
        assert_eq!(actual.len(), 14)
    }
}
