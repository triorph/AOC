extern crate peg;
use crate::map_tile::MapTile;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day23_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule forest() -> MapTile
        = "#" { MapTile::Forest }
    rule path() -> MapTile
        = "." { MapTile::Path }
    rule slope_top_bottom() -> MapTile
        = "v" { MapTile::SlopeTopBottom }
    rule slope_bottom_top() -> MapTile
        = "^" { MapTile::SlopeBottomTop }
    rule slope_left_right() -> MapTile
        = ">" { MapTile::SlopeLeftRight }
    rule slope_right_left() -> MapTile
        = "<" { MapTile::SlopeRightLeft }
    rule map_tile() -> MapTile
        = tile:(forest() / path() / slope_top_bottom() / slope_bottom_top() / slope_left_right() / slope_right_left()) { tile }
    rule line() -> Vec<MapTile>
        = line:map_tile() ++ ""
    pub rule parse() -> Vec<Vec<MapTile>>
        = lines:line() ++ ("\n" +) "\n" * { lines }
}}

pub fn parse_data(input: &str) -> Result<Vec<Vec<MapTile>>, AOCFileOrParseError> {
    if let Ok(ret) = day23_parser::parse(input) {
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
        let actual = day23_parser::parse(&input_str).expect("Should parse successfully");
        assert_eq!(actual.len(), 23);
        assert_eq!(actual[0].len(), 23);
    }
}
