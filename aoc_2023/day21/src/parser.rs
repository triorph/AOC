extern crate peg;
use aoc_helpers::AOCFileOrParseError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GardenTile {
    Plot,
    Rock,
    Start,
}
peg::parser! { pub grammar day21_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule plot() -> GardenTile
        = "." { GardenTile::Plot }
    rule rock() -> GardenTile
        = "#" { GardenTile::Rock }
    rule start() -> GardenTile
        = "S" { GardenTile::Start }
    rule garden_tile() -> GardenTile
        = garden_tile:(plot() / rock() / start()) { garden_tile }
    rule line() -> Vec<GardenTile>
        = line:garden_tile() ++ "" { line }
    pub rule parse() -> Vec<Vec<GardenTile>>
        = lines:line() ++ ("\n" +) "\n" * { lines }
}}

pub fn parse_data(input: &str) -> Result<Vec<Vec<GardenTile>>, AOCFileOrParseError> {
    if let Ok(ret) = day21_parser::parse(input) {
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
        let actual = day21_parser::parse(&input_str).expect("Should parse successfully");
        let expected: Vec<Vec<GardenTile>> = vec![
            vec![
                GardenTile::Plot,
                GardenTile::Plot,
                GardenTile::Plot,
                GardenTile::Plot,
                GardenTile::Plot,
                GardenTile::Plot,
                GardenTile::Plot,
                GardenTile::Plot,
                GardenTile::Plot,
                GardenTile::Plot,
                GardenTile::Plot,
            ],
            vec![
                GardenTile::Plot,
                GardenTile::Plot,
                GardenTile::Plot,
                GardenTile::Plot,
                GardenTile::Plot,
                GardenTile::Rock,
                GardenTile::Rock,
                GardenTile::Rock,
                GardenTile::Plot,
                GardenTile::Rock,
                GardenTile::Plot,
            ],
            vec![
                GardenTile::Plot,
                GardenTile::Rock,
                GardenTile::Rock,
                GardenTile::Rock,
                GardenTile::Plot,
                GardenTile::Rock,
                GardenTile::Rock,
                GardenTile::Plot,
                GardenTile::Plot,
                GardenTile::Rock,
                GardenTile::Plot,
            ],
            vec![
                GardenTile::Plot,
                GardenTile::Plot,
                GardenTile::Rock,
                GardenTile::Plot,
                GardenTile::Rock,
                GardenTile::Plot,
                GardenTile::Plot,
                GardenTile::Plot,
                GardenTile::Rock,
                GardenTile::Plot,
                GardenTile::Plot,
            ],
            vec![
                GardenTile::Plot,
                GardenTile::Plot,
                GardenTile::Plot,
                GardenTile::Plot,
                GardenTile::Rock,
                GardenTile::Plot,
                GardenTile::Rock,
                GardenTile::Plot,
                GardenTile::Plot,
                GardenTile::Plot,
                GardenTile::Plot,
            ],
            vec![
                GardenTile::Plot,
                GardenTile::Rock,
                GardenTile::Rock,
                GardenTile::Plot,
                GardenTile::Plot,
                GardenTile::Start,
                GardenTile::Rock,
                GardenTile::Rock,
                GardenTile::Rock,
                GardenTile::Rock,
                GardenTile::Plot,
            ],
            vec![
                GardenTile::Plot,
                GardenTile::Rock,
                GardenTile::Rock,
                GardenTile::Plot,
                GardenTile::Plot,
                GardenTile::Rock,
                GardenTile::Plot,
                GardenTile::Plot,
                GardenTile::Plot,
                GardenTile::Rock,
                GardenTile::Plot,
            ],
            vec![
                GardenTile::Plot,
                GardenTile::Plot,
                GardenTile::Plot,
                GardenTile::Plot,
                GardenTile::Plot,
                GardenTile::Plot,
                GardenTile::Plot,
                GardenTile::Rock,
                GardenTile::Rock,
                GardenTile::Plot,
                GardenTile::Plot,
            ],
            vec![
                GardenTile::Plot,
                GardenTile::Rock,
                GardenTile::Rock,
                GardenTile::Plot,
                GardenTile::Rock,
                GardenTile::Plot,
                GardenTile::Rock,
                GardenTile::Rock,
                GardenTile::Rock,
                GardenTile::Rock,
                GardenTile::Plot,
            ],
            vec![
                GardenTile::Plot,
                GardenTile::Rock,
                GardenTile::Rock,
                GardenTile::Plot,
                GardenTile::Plot,
                GardenTile::Rock,
                GardenTile::Rock,
                GardenTile::Plot,
                GardenTile::Rock,
                GardenTile::Rock,
                GardenTile::Plot,
            ],
            vec![
                GardenTile::Plot,
                GardenTile::Plot,
                GardenTile::Plot,
                GardenTile::Plot,
                GardenTile::Plot,
                GardenTile::Plot,
                GardenTile::Plot,
                GardenTile::Plot,
                GardenTile::Plot,
                GardenTile::Plot,
                GardenTile::Plot,
            ],
        ];
        assert_eq!(expected, actual)
    }
}
