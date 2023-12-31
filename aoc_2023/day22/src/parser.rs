extern crate peg;
use crate::block::Block;
use aoc_helpers::point3d::Point3D;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day22_parser() for str {
    rule positive_number() -> isize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule negative_number() -> isize
        = "-" positive_number:positive_number() { -positive_number }
    rule number() -> isize
        = number:(negative_number() / positive_number()) { number }
    rule point3d() -> Point3D
        = x:number() "," y:number() "," z:number() { Point3D {x, y, z} }
    rule block() -> Block
        = start:point3d() "~" end:point3d() { Block::new( start, end ) }
    pub rule parse() -> Vec<Block>
        = lines:block() ++ ("\n" +) "\n" * { lines }
}}

pub fn parse_data(input: &str) -> Result<Vec<Block>, AOCFileOrParseError> {
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
        let expected: Vec<Block> = vec![
            Block {
                start: Point3D { x: 1, y: 0, z: 1 },
                end: Point3D { x: 1, y: 2, z: 1 },
                num_moves: 0,
            },
            Block {
                start: Point3D { x: 0, y: 0, z: 2 },
                end: Point3D { x: 2, y: 0, z: 2 },
                num_moves: 0,
            },
            Block {
                start: Point3D { x: 0, y: 2, z: 3 },
                end: Point3D { x: 2, y: 2, z: 3 },
                num_moves: 0,
            },
            Block {
                start: Point3D { x: 0, y: 0, z: 4 },
                end: Point3D { x: 0, y: 2, z: 4 },
                num_moves: 0,
            },
            Block {
                start: Point3D { x: 2, y: 0, z: 5 },
                end: Point3D { x: 2, y: 2, z: 5 },
                num_moves: 0,
            },
            Block {
                start: Point3D { x: 0, y: 1, z: 6 },
                end: Point3D { x: 2, y: 1, z: 6 },
                num_moves: 0,
            },
            Block {
                start: Point3D { x: 1, y: 1, z: 8 },
                end: Point3D { x: 1, y: 1, z: 9 },
                num_moves: 0,
            },
        ];
        assert_eq!(expected, actual)
    }
}
