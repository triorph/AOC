extern crate peg;
use crate::blizzard::{AllBlizzards, Direction, Tile};
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day24_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule floor() -> Tile
        = "." { Tile::Floor }
    rule wall() -> Tile
        = "#" { Tile::Wall }
    rule up() -> Direction
        = "^" { Direction::Up }
    rule down() -> Direction
        = "v" { Direction::Down }
    rule left() -> Direction
        = "<" { Direction::Left }
    rule right() -> Direction
        = ">" { Direction::Right }
    rule blizzard() -> Tile
        = direction:(up() / left() / down() / right()) { Tile::Blizzard(direction) }
    rule tile() -> Tile
        = tile:(floor() / wall() / blizzard()) { tile }
    rule line() -> Vec<Tile>
        = tiles:tile() ++ "" ( " "* ) { tiles }
    pub rule parse() -> AllBlizzards
        = tiles_lines:line() ++ ("\n") "\n" * {
            AllBlizzards::new(&tiles_lines)
        }
}}

pub fn parse_data(input: &str) -> Result<AllBlizzards, AOCFileOrParseError> {
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
    }
}
