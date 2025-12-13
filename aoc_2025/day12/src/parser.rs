extern crate peg;
use aoc_helpers::AOCFileOrParseError;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum PresentTile {
    Full,
    Empty,
}

pub type Present = (usize, Vec<Vec<PresentTile>>);
pub type Goal = (usize, usize, Vec<usize>);

peg::parser! { pub grammar day12_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().unwrap_or_else(|_| panic!("Was expecting a number string {}", n))}
    rule full_present() -> PresentTile
        = "#" { PresentTile::Full }
    rule empty_present() -> PresentTile
        = "." { PresentTile::Empty }
    rule present_tile() -> PresentTile
        = present_tile:(full_present() / empty_present()) { present_tile }
    rule present_line() -> Vec<PresentTile>
        = present_line:present_tile() ++ "" { present_line }
    rule present() -> Present
        = index:number() ":\n" present:present_line() ++ "\n" { (index, present) }
    rule presents() -> Vec<Present>
        = presents:present() ++ ("\n"+) { presents }
    rule goal() -> Goal
        = x:number() "x" y:number() ": " present_counts:number() ++ " " { (x, y, present_counts) }
    rule goals() -> Vec<(Goal)>
        = goals:goal() ++ "\n" { goals }
    pub rule parse() -> (Vec<Present>, Vec<Goal>)
        = presents:presents() "\n" * goals:goals() "\n"* { (presents, goals) }
}}

pub fn parse_data(input: &str) -> Result<(Vec<Present>, Vec<Goal>), AOCFileOrParseError> {
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
        let (actual_presents, actual_goals) =
            day12_parser::parse(&input_str).expect("Should parse successfully");
        let expected_presents: Vec<Present> = vec![];
        let expected_goals: Vec<Goal> = vec![];
        assert_eq!(expected_presents, actual_presents);
        assert_eq!(expected_goals, actual_goals);
    }
}
