extern crate peg;
use aoc_helpers::{point2d::Point2D, AOCFileOrParseError};
pub type ClawMachine = (Point2D, Point2D, Point2D);

peg::parser! { pub grammar day13_parser() for str {
    rule number() -> isize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule button() -> Point2D
        = "Button " [_] ": X+" x:number() ", Y+" y:number() { Point2D{ x, y } }
    rule prize() -> Point2D
        = "Prize: X=" x:number() ", Y=" y:number() { Point2D{ x, y } }
    rule claw_machine() -> ClawMachine
        = button_1:button() "\n" button_2:button() "\n" prize:prize() { (button_1, button_2, prize) }
    pub rule parse() -> Vec<ClawMachine>
        = machines:claw_machine() ++ ("\n" +) "\n" * { machines }
}}

pub fn parse_data(input: &str) -> Result<Vec<ClawMachine>, AOCFileOrParseError> {
    if let Ok(ret) = day13_parser::parse(input) {
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
        let actual = day13_parser::parse(&input_str).expect("Should parse successfully");
        let expected: Vec<ClawMachine> = vec![
            (
                Point2D::from_usize(94, 34),
                Point2D::from_usize(22, 67),
                Point2D::from_usize(8400, 5400),
            ),
            (
                Point2D::from_usize(26, 66),
                Point2D::from_usize(67, 21),
                Point2D::from_usize(12748, 12176),
            ),
            (
                Point2D::from_usize(17, 86),
                Point2D::from_usize(84, 37),
                Point2D::from_usize(7870, 6450),
            ),
            (
                Point2D::from_usize(69, 23),
                Point2D::from_usize(27, 71),
                Point2D::from_usize(18641, 10279),
            ),
        ];
        assert_eq!(expected, actual)
    }
}
