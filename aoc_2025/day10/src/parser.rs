extern crate peg;
use aoc_helpers::AOCFileOrParseError;

pub type Machine = (Vec<bool>, Vec<Vec<usize>>, Vec<usize>);

peg::parser! { pub grammar day10_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().unwrap_or_else(|_| panic!("Was expecting a number string {}", n))}
    rule indicator_on() -> bool
        = "#" { true }
    rule indicator_off() -> bool
        = "." { false }
    rule indicator_light() -> bool
        = indicator_light:(indicator_on() / indicator_off()) { indicator_light }
    rule indicator_block() -> Vec<bool>
        = "[" indicator_block:indicator_light() ++ "" "]" { indicator_block }
    rule button_schematic() -> Vec<usize>
        = "(" button_schematic:number() ++ "," ")" { button_schematic }
    rule button_schematics() -> Vec<Vec<usize>>
        = button_schematics:button_schematic() ++ " " { button_schematics }
    rule joltage_requirements() -> Vec<usize>
        = "{" joltage_requirements:number() ++ "," "}" { joltage_requirements }
    rule line() -> Machine
        = indicator_block:indicator_block() " " button_schematics:button_schematics() " " joltage_requirements:joltage_requirements() {
            (indicator_block, button_schematics, joltage_requirements)
        }
    pub rule parse() -> Vec<Machine>
        = lines:line() ++ ("\n" +) "\n" * { lines }
}}

pub fn parse_data(input: &str) -> Result<Vec<Machine>, AOCFileOrParseError> {
    if let Ok(ret) = day10_parser::parse(input) {
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
        let actual = day10_parser::parse(&input_str).expect("Should parse successfully");
        let expected: Vec<Machine> = vec![
            (
                vec![false, true, true, false],
                vec![
                    vec![3],
                    vec![1, 3],
                    vec![2],
                    vec![2, 3],
                    vec![0, 2],
                    vec![0, 1],
                ],
                vec![3, 5, 4, 7],
            ),
            (
                vec![false, false, false, true, false],
                vec![
                    vec![0, 2, 3, 4],
                    vec![2, 3],
                    vec![0, 4],
                    vec![0, 1, 2],
                    vec![1, 2, 3, 4],
                ],
                vec![7, 5, 12, 7, 2],
            ),
            (
                vec![false, true, true, true, false, true],
                vec![
                    vec![0, 1, 2, 3, 4],
                    vec![0, 3, 4],
                    vec![0, 1, 2, 4, 5],
                    vec![1, 2],
                ],
                vec![10, 11, 11, 5, 10, 5],
            ),
        ];
        assert_eq!(expected, actual)
    }
}
