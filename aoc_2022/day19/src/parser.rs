extern crate peg;
use aoc_helpers::AOCFileOrParseError;

use crate::robot::{Blueprint, RobotType};

peg::parser! { pub grammar day19_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule robot_type_ore() -> RobotType
        = "ore" { RobotType::Ore }
    rule robot_type_clay() -> RobotType
        = "clay" { RobotType::Clay }
    rule robot_type_obsidian() -> RobotType
        = "obsidian" { RobotType::Obsidian }
    rule robot_type_geode() -> RobotType
        = "geode" { RobotType::Geode }
    rule robot_type() -> RobotType
        = t:(robot_type_ore() / robot_type_clay() / robot_type_obsidian() / robot_type_geode()) { t }
    rule one_type_cost() -> (usize, RobotType)
        = n:number() ( " " ) t:robot_type() { (n, t) }
    rule robot_cost() -> (RobotType, Vec<(usize, RobotType)>)
        = "Each " t:robot_type() " robot costs " costs:one_type_cost() ++ ( " and " ) ( "." ) {
            (t, costs)
        }
    rule blueprint() -> Blueprint
        = "Blueprint " n:number() ": " robot_costs:robot_cost() ++ " " {
            Blueprint::new(n, &robot_costs)
        }
    pub rule parse() -> Vec<Blueprint>
        = blueprints:blueprint() ++ ("\n") "\n" * {
             { blueprints }
        }
}}

pub fn parse_data(input: &str) -> Result<Vec<Blueprint>, AOCFileOrParseError> {
    if let Ok(ret) = day19_parser::parse(input) {
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
        let actual = day19_parser::parse(&input_str).expect("Should parse successfully");
        assert_eq!(actual.len(), 2);
        assert_eq!(
            actual[0].costs,
            [[4, 0, 0], [2, 0, 0], [3, 14, 0], [2, 0, 7]]
        );
        assert_eq!(
            actual[1].costs,
            [[2, 0, 0], [3, 0, 0], [3, 8, 0], [3, 0, 12]]
        );
    }

    #[test]
    fn test_parse_real_data() {
        let input_str = read_input_file("data/input_data.txt").unwrap();
        let actual = day19_parser::parse(&input_str).expect("Should parse successfully");
        assert_eq!(actual.len(), 30);
        assert_eq!(actual[29].index, 30);
    }
}
