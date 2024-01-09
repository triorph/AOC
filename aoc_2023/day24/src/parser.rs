extern crate peg;
use aoc_helpers::point3d::Point3D;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day24_parser() for str {
    rule positive_number() -> isize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule negative_number() -> isize
        = "-" positive_number:positive_number() { - positive_number }
    rule number() -> isize
        = number:(positive_number() / negative_number()) { number }
    rule point() -> Point3D
        = x:number() "," " "* y:number() "," " "* z:number() { Point3D { x, y, z } }
    rule line() -> (Point3D, Point3D)
        = position:point() " "* "@" " "* velocity:point() { (position, velocity) }
    pub rule parse() -> Vec<(Point3D, Point3D)>
        = lines:line() ++ ("\n" +) "\n" * { lines }
}}

pub fn parse_data(input: &str) -> Result<Vec<(Point3D, Point3D)>, AOCFileOrParseError> {
    if let Ok(ret) = day24_parser::parse(input) {
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
        let actual = day24_parser::parse(&input_str).expect("Should parse successfully");
        let expected: Vec<(Point3D, Point3D)> = vec![
            (
                Point3D {
                    x: 19,
                    y: 13,
                    z: 30,
                },
                Point3D { x: -2, y: 1, z: -2 },
            ),
            (
                Point3D {
                    x: 18,
                    y: 19,
                    z: 22,
                },
                Point3D {
                    x: -1,
                    y: -1,
                    z: -2,
                },
            ),
            (
                Point3D {
                    x: 20,
                    y: 25,
                    z: 34,
                },
                Point3D {
                    x: -2,
                    y: -2,
                    z: -4,
                },
            ),
            (
                Point3D {
                    x: 12,
                    y: 31,
                    z: 28,
                },
                Point3D {
                    x: -1,
                    y: -2,
                    z: -1,
                },
            ),
            (
                Point3D {
                    x: 20,
                    y: 19,
                    z: 15,
                },
                Point3D { x: 1, y: -5, z: -3 },
            ),
        ];
        assert_eq!(expected, actual)
    }
}
