extern crate peg;
use crate::types::{Direction, MapTile};
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day15_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule wall() -> MapTile
        = "#" { MapTile::Wall }
    rule box_object() -> MapTile
        = "O" { MapTile::Box }
    rule character() -> MapTile
        = "@" { MapTile::Character }
    rule empty() -> MapTile
        = "." { MapTile::Empty }
    rule map_tile() -> MapTile
        = map_tile:(wall() / box_object() / character() / empty()) { map_tile }
    rule map_line() -> Vec<MapTile>
        = map_line:map_tile() ++ "" { map_line }
    rule map() -> Vec<Vec<MapTile>>
        = map:map_line() ++ "\n" { map }
    rule up() -> Direction
        = "^" { Direction::Up }
    rule down() -> Direction
        = "v" { Direction::Down }
    rule left() -> Direction
        = "<" { Direction::Left }
    rule right() -> Direction
        = ">" { Direction::Right }
    rule direction() -> Direction
        = direction:(up() / down() / left() / right()) { direction }
    rule directions() -> Vec<Direction>
        = directions:direction() ++ ("\n"?) { directions }
    pub rule parse() -> (Vec<Vec<MapTile>>, Vec<Direction>)
        = map:map() "\n"* directions:directions() "\n"* { (map, directions) }
}}

pub fn parse_data(input: &str) -> Result<(Vec<Vec<MapTile>>, Vec<Direction>), AOCFileOrParseError> {
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
        let input_str = read_input_file("data/test_data_2.txt").unwrap();
        let (actual_map, actual_directions) =
            day15_parser::parse(&input_str).expect("Should parse successfully");
        let expected_map: Vec<Vec<MapTile>> = vec![
            vec![
                MapTile::Wall,
                MapTile::Wall,
                MapTile::Wall,
                MapTile::Wall,
                MapTile::Wall,
                MapTile::Wall,
                MapTile::Wall,
                MapTile::Wall,
            ],
            vec![
                MapTile::Wall,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Box,
                MapTile::Empty,
                MapTile::Box,
                MapTile::Empty,
                MapTile::Wall,
            ],
            vec![
                MapTile::Wall,
                MapTile::Wall,
                MapTile::Character,
                MapTile::Empty,
                MapTile::Box,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Wall,
            ],
            vec![
                MapTile::Wall,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Box,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Wall,
            ],
            vec![
                MapTile::Wall,
                MapTile::Empty,
                MapTile::Wall,
                MapTile::Empty,
                MapTile::Box,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Wall,
            ],
            vec![
                MapTile::Wall,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Box,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Wall,
            ],
            vec![
                MapTile::Wall,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Empty,
                MapTile::Wall,
            ],
            vec![
                MapTile::Wall,
                MapTile::Wall,
                MapTile::Wall,
                MapTile::Wall,
                MapTile::Wall,
                MapTile::Wall,
                MapTile::Wall,
                MapTile::Wall,
            ],
        ];
        let expected_directions = vec![
            Direction::Left,
            Direction::Up,
            Direction::Up,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Down,
            Direction::Down,
            Direction::Left,
            Direction::Down,
            Direction::Right,
            Direction::Right,
            Direction::Down,
            Direction::Left,
            Direction::Left,
        ];
        assert_eq!(expected_map, actual_map);
        assert_eq!(expected_directions, actual_directions);
    }
}
