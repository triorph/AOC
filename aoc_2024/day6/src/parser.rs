extern crate peg;
use aoc_helpers::AOCFileOrParseError;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MapTile {
    Rock,
    Player,
    Empty,
}

peg::parser! { pub grammar day6_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule rock() -> MapTile
        = "#" { MapTile::Rock }
    rule player() -> MapTile
        = "^" { MapTile::Player }
    rule empty() -> MapTile
        = "." { MapTile::Empty }
    rule map_tile() -> MapTile
        = map_tile:(rock() / player() / empty()) { map_tile }
    rule line() -> Vec<MapTile>
        = line:map_tile() ++ "" { line }
    pub rule parse() -> Vec<Vec<MapTile>>
        = lines:line() ++ ("\n" +) "\n" * { lines }
}}

pub fn parse_data(input: &str) -> Result<Vec<Vec<MapTile>>, AOCFileOrParseError> {
    if let Ok(ret) = day6_parser::parse(input) {
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
        let actual = day6_parser::parse(&input_str).expect("Should parse successfully");
        let expected: Vec<Vec<MapTile>> = vec![
            vec![
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Rock,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
            ],
            vec![
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Rock,
            ],
            vec![
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
            ],
            vec![
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Rock,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
            ],
            vec![
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Rock,
                MapTile::Empty,
                MapTile::Empty,
            ],
            vec![
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
            ],
            vec![
                MapTile::Empty,
                MapTile::Rock,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Player,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
            ],
            vec![
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Rock,
                MapTile::Empty,
            ],
            vec![
                MapTile::Rock,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
            ],
            vec![
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Rock,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
            ],
        ];
        assert_eq!(expected, actual)
    }
}
