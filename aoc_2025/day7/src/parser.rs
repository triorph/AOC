extern crate peg;
use aoc_helpers::{point2d::Point2D, AOCFileOrParseError};
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum RoomTile {
    Start,
    Empty,
    Splitter,
}

peg::parser! { pub grammar day7_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().unwrap_or_else(|_| panic!("Was expecting a number string {}", n))}
    rule start() -> RoomTile
        = "S" { RoomTile::Start }
    rule empty() -> RoomTile
        = "." { RoomTile::Empty }
    rule splitter() -> RoomTile
        = "^" { RoomTile::Splitter }
    rule room_tile() -> RoomTile
        = room_tile:(start() / empty() / splitter())
    rule line() -> Vec<RoomTile>
        = line:room_tile() ++ "" { line }
    pub rule parse() -> (Point2D, HashSet<Point2D>)
        = lines:line() ++ ("\n" +) "\n" * {
        (
            Point2D::iterate_x_y_usize(0..lines[0].len(), 0..lines.len())
                .into_iter()
                .find(|p| lines[p.y as usize][p.x as usize] == RoomTile::Start)
                .unwrap(),
            Point2D::iterate_x_y_usize(0..lines[0].len(), 0..lines.len())
                .into_iter()
                .filter(|p| lines[p.y as usize][p.x as usize] == RoomTile::Splitter)
                .collect()
        )
    }
}}

pub fn parse_data(input: &str) -> Result<(Point2D, HashSet<Point2D>), AOCFileOrParseError> {
    if let Ok(ret) = day7_parser::parse(input) {
        Ok(ret)
    } else {
        Err(AOCFileOrParseError)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    use aoc_helpers::{hash_utils::FromVec, read_input_file};
    #[test]
    fn test_parse() {
        let input_str = read_input_file("data/test_data.txt").unwrap();
        let (actual_start, actual_splitters) =
            day7_parser::parse(&input_str).expect("Should parse successfully");
        let expected_start: Point2D = Point2D { x: 7, y: 0 };
        let expected_splitters: HashSet<Point2D> = HashSet::from_vec(&vec![
            Point2D { x: 7, y: 2 },
            Point2D { x: 6, y: 4 },
            Point2D { x: 8, y: 4 },
            Point2D { x: 5, y: 6 },
            Point2D { x: 7, y: 6 },
            Point2D { x: 9, y: 6 },
            Point2D { x: 4, y: 8 },
            Point2D { x: 6, y: 8 },
            Point2D { x: 10, y: 8 },
            Point2D { x: 3, y: 10 },
            Point2D { x: 5, y: 10 },
            Point2D { x: 9, y: 10 },
            Point2D { x: 11, y: 10 },
            Point2D { x: 2, y: 12 },
            Point2D { x: 6, y: 12 },
            Point2D { x: 12, y: 12 },
            Point2D { x: 1, y: 14 },
            Point2D { x: 3, y: 14 },
            Point2D { x: 5, y: 14 },
            Point2D { x: 7, y: 14 },
            Point2D { x: 9, y: 14 },
            Point2D { x: 13, y: 14 },
        ]);
        assert_eq!(expected_start, actual_start);
        assert_eq!(expected_splitters, actual_splitters);
    }
}
