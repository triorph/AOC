extern crate peg;
use aoc_helpers::AOCFileOrParseError;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Tile {
    Ash,
    Volcano,
}

peg::parser! { pub grammar day13_parser() for str {
    rule volcano() -> Tile
        = "#" { Tile::Volcano }
    rule ash() -> Tile
        = "." { Tile::Ash }
    rule tile() -> Tile
        = tile:(ash() / volcano()) { tile }
    rule line() -> Vec<Tile>
        = line:tile() ++ ""
    rule location() -> Vec<Vec<Tile>>
        = location:line() ++ "\n"
    pub rule parse() -> Vec<Vec<Vec<Tile>>>
        = locations:location() ++ ("\n" +) "\n" * { locations }
}}

pub fn parse_data(input: &str) -> Result<Vec<Vec<Vec<Tile>>>, AOCFileOrParseError> {
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
        assert_eq!(2, actual.len());
    }
}
