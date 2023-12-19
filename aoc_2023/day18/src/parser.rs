extern crate peg;
use crate::direction::Direction;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day18_parser() for str {
    rule digit() -> usize
        = n:$(['0'..='9']) { n.parse().unwrap_or_else(|_| panic!("Was expecting a number string {}", n))}
    rule hex_letter() -> usize
        = n:$(['a'..='f']) { match n { "a" => 10, "b" => 11, "c" => 12, "d" => 13, "e" => 14, _ => 15} }
    rule hex_digit() -> usize
        = n:(digit() / hex_letter())
    rule hex_number() -> usize
        = digits:hex_digit() ++ "" { digits.into_iter().fold(0, |value, digit| value * 16 + digit) }
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().unwrap_or_else(|_|panic!("Was expecting a number string{}", n)) }
    rule up() -> Direction
        = "U" { Direction::Up }
    rule down() -> Direction
        = "D" { Direction::Down }
    rule left() -> Direction
        = "L" { Direction::Left }
    rule right() -> Direction
        = "R" { Direction::Right }
    rule direction() -> Direction
        = direction:(up() / down() / left() / right())
    rule line() -> (Direction, usize, usize)
        = direction:direction() " " length:number() " (#" colour:hex_number() ")" { (direction, length, colour) }
    pub rule parse() -> Vec<(Direction, usize, usize)>
        = lines:line() ++ ("\n" +) "\n" * { lines }
}}

pub fn parse_data(input: &str) -> Result<Vec<(Direction, usize, usize)>, AOCFileOrParseError> {
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
        let expected: Vec<(Direction, usize, usize)> = vec![
            (Direction::Right, 6, 0x70c710),
            (Direction::Down, 5, 0x0dc571),
            (Direction::Left, 2, 0x5713f0),
            (Direction::Down, 2, 0xd2c081),
            (Direction::Right, 2, 0x59c680),
            (Direction::Down, 2, 0x411b91),
            (Direction::Left, 5, 0x8ceee2),
            (Direction::Up, 2, 0xcaa173),
            (Direction::Left, 1, 0x1b58a2),
            (Direction::Up, 2, 0xcaa171),
            (Direction::Right, 2, 0x7807d2),
            (Direction::Up, 3, 0xa77fa3),
            (Direction::Left, 2, 0x015232),
            (Direction::Up, 2, 0x7a21e3),
        ];
        assert_eq!(expected, actual)
    }
}
