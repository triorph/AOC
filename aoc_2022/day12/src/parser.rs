extern crate peg;
use crate::map_point::MapPoint;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day12_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule other_type() -> MapPoint
        = c:$(['a'..='z']) { MapPoint::Other(u32::from(c.chars().next().unwrap()) - u32::from('a')) }
    rule start() -> MapPoint
        = "S" { MapPoint::Start }
    rule end() -> MapPoint
        = "E" { MapPoint::End }
    rule map_point() -> MapPoint
        = mp:(other_type() / start() / end() ) { mp }
    rule x_vals() -> Vec<MapPoint>
        = x_vals:map_point() ++ "" { x_vals }
    rule all_vals() -> Vec<Vec<MapPoint>>
        = all_vals: x_vals() ++ "\n" { all_vals }
    pub rule parse() -> Vec<Vec<MapPoint>>
        = all_vals:all_vals() "\n" * {
             { all_vals }
        }
}}

pub fn parse_data(input: &str) -> Result<Vec<Vec<MapPoint>>, AOCFileOrParseError> {
    if let Ok(ret) = day12_parser::parse(input) {
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
        let actual = day12_parser::parse(&input_str).expect("Should parse successfully");
        assert_eq!(actual.len(), 5);
        assert_eq!(actual[0].len(), 8);
    }

    #[test]
    fn test_real_data_parse() {
        let input_str = read_input_file("data/input_data.txt").unwrap();
        let actual = day12_parser::parse(&input_str).expect("Should parse successfully");
        assert_eq!(actual.len(), 41);
        assert_eq!(actual[0].len(), 143);
    }
}
