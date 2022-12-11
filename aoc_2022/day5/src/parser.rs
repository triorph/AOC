extern crate peg;
use crate::stack_set::Instruction;
use aoc_helpers::hash_utils::HashVec;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day5_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule some_value() -> Option<char>
        = "[" v:$(['A'..='Z']) "]" { Some(v.chars().next().expect("Will have a value")) }
    rule not_value() -> Option<char>
        = "   "  { None }
    rule value() -> Option<char>
        = value:(some_value() / not_value()) { value }
    rule horizontal_values() -> Vec<Option<char>>
        = line_of_values:value() ++ " " {line_of_values}
    rule names() -> Vec<usize>
        = " " * names:number() ++ (" " +) " " * { names }
    rule build_vertical_values() -> HashVec<usize, char>
        = lines_of_horizontals:horizontal_values() ++ "\n" ("\n") names:names() "\n" {
            convert_horizontal_to_vertical(names, lines_of_horizontals)
        }
    rule move() -> Instruction
        = "move " quantity:number() " from " source:number() " to " destination:number() {
            (quantity, source, destination)
        }
    rule moves() -> Vec<Instruction>
        = moves:move() ++ "\n" { moves }
    pub rule parse() -> (HashVec<usize, char>, Vec<Instruction>)
        = stack_set:build_vertical_values() "\n" move_list:moves() "\n"* {
            (stack_set, move_list)
        }
}}

fn convert_horizontal_to_vertical(
    names: Vec<usize>,
    lines_of_horizontals: Vec<Vec<Option<char>>>,
) -> HashVec<usize, char> {
    let mut ret: HashVec<usize, char> = HashVec::new();
    for line in lines_of_horizontals.iter().rev() {
        for (name, value) in names.iter().zip(line) {
            if let Some(value) = value {
                ret.push(*name, *value);
            }
        }
    }
    ret
}

pub fn parse_data(
    input: &str,
) -> Result<(HashVec<usize, char>, Vec<Instruction>), AOCFileOrParseError> {
    if let Ok(ret) = day5_parser::parse(input) {
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
        let actual = day5_parser::parse(&input_str).expect("Should parse successfully");
        assert_eq!(actual.0.len(), 3);
        assert_eq!(actual.0.get(&1).expect("1 is in dict"), &vec!['Z', 'N']);
        assert_eq!(
            actual.0.get(&2).expect("2 is in dict"),
            &vec!['M', 'C', 'D']
        );
        assert_eq!(actual.0.get(&3).expect("3 is in dict"), &vec!['P']);
        assert_eq!(actual.1.len(), 4);
        // move 1 from 2 to 1
        assert_eq!(actual.1[0], (1, 2, 1));
        // move 3 from 1 to 3
        assert_eq!(actual.1[1], (3, 1, 3));
        // move 2 from 2 to 1
        assert_eq!(actual.1[2], (2, 2, 1));
        // move 1 from 1 to 2
        assert_eq!(actual.1[3], (1, 1, 2));
    }
}
