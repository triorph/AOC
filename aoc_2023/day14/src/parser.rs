extern crate peg;
use aoc_helpers::AOCFileOrParseError;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Tile {
    Rock,
    Wall,
    Empty,
}

peg::parser! { pub grammar day14_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule rock() -> Tile
        = "O" { Tile::Rock }
    rule wall() -> Tile
        = "#" { Tile::Wall }
    rule empty() -> Tile
        = "." { Tile::Empty }
    rule tile() -> Tile
        = tile:(rock() / wall() / empty()) { tile }
    rule line() -> Vec<Tile>
        = line:tile() ++ "" { line }
    pub rule parse() -> Vec<Vec<Tile>>
        = lines:line() ++ ("\n" +) "\n" * { lines }
}}

pub fn parse_data(input: &str) -> Result<Vec<Vec<Tile>>, AOCFileOrParseError> {
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
        assert_eq!(10, actual.len())
    }
}
