extern crate peg;
use aoc_helpers::point2d::Point2D;
use aoc_helpers::AOCFileOrParseError;
use std::collections::HashSet;

peg::parser! {
    pub grammar day4_parser() for str {
        rule number() -> usize
            = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
        rule paper() -> bool
            = "@" { true }
        rule empty() -> bool
            = "." { false }
        rule location() -> bool
            = location:(paper() / empty()) { location }
        rule row() -> Vec<bool>
            = row:location() ++ "" { row }
        pub rule parse() -> HashSet<Point2D>
            = rows:row() ++ ("\n" +) "\n" {
                Point2D::iterate_x_y_usize(0..(rows[0].len()), 0..(rows.len()))
                    .into_iter()
                    .filter(|point| rows[point.y as usize][point.x as usize])
                    .collect()
            }
    }
}

pub fn parse_data(input: &str) -> Result<HashSet<Point2D>, AOCFileOrParseError> {
    if let Ok(ret) = day4_parser::parse(input) {
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
        let actual = day4_parser::parse(&input_str).expect("Should parse successfully");
        let expected: HashSet<Point2D> = HashSet::from_vec(&vec![
            Point2D::from_usize(2, 0),
            Point2D::from_usize(3, 0),
            Point2D::from_usize(5, 0),
            Point2D::from_usize(6, 0),
            Point2D::from_usize(7, 0),
            Point2D::from_usize(8, 0),
            Point2D::from_usize(0, 1),
            Point2D::from_usize(1, 1),
            Point2D::from_usize(2, 1),
            Point2D::from_usize(4, 1),
            Point2D::from_usize(6, 1),
            Point2D::from_usize(8, 1),
            Point2D::from_usize(9, 1),
            Point2D::from_usize(0, 2),
            Point2D::from_usize(1, 2),
            Point2D::from_usize(2, 2),
            Point2D::from_usize(3, 2),
            Point2D::from_usize(4, 2),
            Point2D::from_usize(6, 2),
            Point2D::from_usize(8, 2),
            Point2D::from_usize(9, 2),
            Point2D::from_usize(0, 3),
            Point2D::from_usize(2, 3),
            Point2D::from_usize(3, 3),
            Point2D::from_usize(4, 3),
            Point2D::from_usize(5, 3),
            Point2D::from_usize(8, 3),
            Point2D::from_usize(0, 4),
            Point2D::from_usize(1, 4),
            Point2D::from_usize(3, 4),
            Point2D::from_usize(4, 4),
            Point2D::from_usize(5, 4),
            Point2D::from_usize(6, 4),
            Point2D::from_usize(8, 4),
            Point2D::from_usize(9, 4),
            Point2D::from_usize(1, 5),
            Point2D::from_usize(2, 5),
            Point2D::from_usize(3, 5),
            Point2D::from_usize(4, 5),
            Point2D::from_usize(5, 5),
            Point2D::from_usize(6, 5),
            Point2D::from_usize(7, 5),
            Point2D::from_usize(9, 5),
            Point2D::from_usize(1, 6),
            Point2D::from_usize(3, 6),
            Point2D::from_usize(5, 6),
            Point2D::from_usize(7, 6),
            Point2D::from_usize(8, 6),
            Point2D::from_usize(9, 6),
            Point2D::from_usize(0, 7),
            Point2D::from_usize(2, 7),
            Point2D::from_usize(3, 7),
            Point2D::from_usize(4, 7),
            Point2D::from_usize(6, 7),
            Point2D::from_usize(7, 7),
            Point2D::from_usize(8, 7),
            Point2D::from_usize(9, 7),
            Point2D::from_usize(1, 8),
            Point2D::from_usize(2, 8),
            Point2D::from_usize(3, 8),
            Point2D::from_usize(4, 8),
            Point2D::from_usize(5, 8),
            Point2D::from_usize(6, 8),
            Point2D::from_usize(7, 8),
            Point2D::from_usize(8, 8),
            Point2D::from_usize(0, 9),
            Point2D::from_usize(2, 9),
            Point2D::from_usize(4, 9),
            Point2D::from_usize(5, 9),
            Point2D::from_usize(6, 9),
            Point2D::from_usize(8, 9),
        ]);
        assert_eq!(expected, actual)
    }
}
