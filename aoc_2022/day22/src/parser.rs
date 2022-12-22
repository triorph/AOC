extern crate peg;
use crate::instruction::Instruction;
use crate::map::{Map, MapTile};
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day22_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule rotate_left() -> Instruction
        = "L" { Instruction::RotateLeft }
    rule rotate_right() -> Instruction
        = "R" { Instruction::RotateRight }
    rule move_forward() -> Instruction
        = n:number() { Instruction::MoveForward(n) }
    rule instruction() -> Instruction
        = instruction:(rotate_left() / rotate_right() / move_forward()) { instruction }
    rule wrap() -> MapTile
        = " " { MapTile::Wrap }
    rule wall() -> MapTile
        = ("#") { MapTile::Wall }
    rule floor() -> MapTile
        = (".") { MapTile::Floor }
    rule map_point() -> MapTile
        = point:( wall() / floor() / wrap() ) { point }
    rule map_line() -> Vec<MapTile>
        = line:map_point() ++ "" { line }
    rule map() -> Map
        = points:map_line() ++ "\n" { Map::new( points ) }
    rule instructions() -> Vec<Instruction>
        = instructions:instruction() ++ "" { instructions }
    pub rule parse() -> (Map, Vec<Instruction>)
        = map:map() "\n"* instructions:instructions() "\n"* {
             (map, instructions)
        }
}}

pub fn parse_data(input: &str) -> Result<(Map, Vec<Instruction>), AOCFileOrParseError> {
    if let Ok(ret) = day22_parser::parse(input) {
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
        let actual = day22_parser::parse(&input_str).expect("Should parse successfully");
        assert_eq!(actual.1.len(), 13)
    }
}
