extern crate peg;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day17_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule register() -> usize
        = "Register " [_] ": " register:number() { register }
    rule registers() -> Vec<usize>
        = registers:register() ++ "\n" { registers }
    rule instructions() -> Vec<usize>
        = "Program: " instructions:number() ++ (",") { instructions }
    pub rule parse() -> (Vec<usize>, Vec<usize>)
        = registers:registers() "\n"* instructions:instructions() "\n"* { (registers, instructions) }
}}

pub fn parse_data(input: &str) -> Result<(Vec<usize>, Vec<usize>), AOCFileOrParseError> {
    if let Ok(ret) = day17_parser::parse(input) {
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
        let (actual_registers, actual_instructions) =
            day17_parser::parse(&input_str).expect("Should parse successfully");
        let expected_registers: Vec<usize> = vec![729, 0, 0];
        let expected_instructions: Vec<usize> = vec![0, 1, 5, 4, 3, 0];
        assert_eq!(expected_registers, actual_registers);
        assert_eq!(expected_instructions, actual_instructions);
    }
}
