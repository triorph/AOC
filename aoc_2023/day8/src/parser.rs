extern crate peg;
use std::collections::HashMap;

use crate::direction::Direction;
use aoc_helpers::AOCFileOrParseError;

pub type NodeMap = HashMap<String, (String, String)>;

peg::parser! { pub grammar day8_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule left() -> Direction
        = "L" { Direction::Left }
    rule right() -> Direction
        = "R" { Direction::Right }
    rule direction() -> Direction
        = direction: (left() / right()) { direction }
    rule directions() -> Vec<Direction>
        = directions:direction() ++ "" " "* { directions }
    rule node() -> String
        = node:$(['A'..='Z'|'1'..='9']*<3>) { node.to_string() }
    rule node_edges() -> (String, (String, String))
        = input:node() " = (" left:node() ", " right:node() ")" " "*{ (input, (left, right))}
    rule all_nodes() -> NodeMap
        = all_nodes:node_edges() ++ ("\n"+)  "\n"* {
            let mut ret = HashMap::new();
            for (node, (left, right)) in all_nodes.into_iter() {
                ret.insert(node, (left, right));
            }
            ret
        }
    pub rule parse() -> (Vec<Direction>, NodeMap)
        = directions:directions() "\n"+ all_nodes:all_nodes() "\n"* { (directions, all_nodes) }
}}

pub fn parse_data(input: &str) -> Result<(Vec<Direction>, NodeMap), AOCFileOrParseError> {
    if let Ok(ret) = day8_parser::parse(input) {
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
        let (directions, _) = day8_parser::parse(&input_str).expect("Should parse successfully");
        let expected: Vec<Direction> = vec![Direction::Left, Direction::Left, Direction::Right];
        assert_eq!(expected, directions);
    }
}
