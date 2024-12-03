extern crate peg;
use aoc_helpers::AOCFileOrParseError;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Instruction {
    Do,
    DoNot,
    Multiply(usize, usize),
}

peg::parser! { pub grammar day3_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule mul_instruction() -> Instruction
        = "mul(" left:number() "," right:number() ")" { Instruction::Multiply(left, right) }
    rule do() -> Instruction
        = "do()" { Instruction::Do }
    rule do_not() -> Instruction
        = "don't()" { Instruction::DoNot }
    rule instruction() -> Instruction
        = instruction:(mul_instruction() / do() / do_not()) { instruction }
    rule blank_space() -> ()
        = (!instruction() [_])* {}
    pub rule parse() -> Vec<Instruction>
        = blank_space() instructions:instruction() ++ (blank_space()) blank_space()  { instructions }
}}

pub fn parse_data(input: &str) -> Result<Vec<Instruction>, AOCFileOrParseError> {
    if let Ok(ret) = day3_parser::parse(input) {
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
        let actual = day3_parser::parse(&input_str).expect("Should parse successfully");
        let expected: Vec<Instruction> = vec![
            Instruction::Multiply(2, 4),
            Instruction::DoNot,
            Instruction::Multiply(5, 5),
            Instruction::Multiply(11, 8),
            Instruction::Do,
            Instruction::Multiply(8, 5),
        ];
        assert_eq!(expected, actual)
    }
}
