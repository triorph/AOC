extern crate peg;
use aoc_helpers::AOCFileOrParseError;

use crate::operator::Operator;
pub type StringList = Vec<String>;
pub type OperatorAndSize = (Operator, usize);
pub type OperatorAndSizeList = Vec<OperatorAndSize>;

peg::parser! { pub grammar day6_parser() for str {
    rule add() -> Operator
        = "+" { Operator::Add }
    rule multiply() -> Operator
        = "*" { Operator::Multiply }
    rule operator() -> Operator
        = operator:(add() / multiply()) { operator }
    rule operator_and_spaces_count() -> OperatorAndSize
        = operator:operator() spaces:((!(" " operator()) " ")*) { (operator, spaces.len()) }
    rule number_line() -> String
        = n:$([' '|'0'..='9']+) { n.to_string() }
    rule operator_line() -> OperatorAndSizeList
        = " " * operators:operator_and_spaces_count() ++ (" ") " " * { operators }
    rule number_lines() -> Vec<String>
        = lines:number_line() ++ ("\n" +) "\n" { lines }
    pub rule parse() -> (StringList, OperatorAndSizeList)
        = number_lines:number_lines() "\n" * operator_line:operator_line() "\n" * { (number_lines, operator_line) }
}}

pub fn parse_data(input: &str) -> Result<(StringList, OperatorAndSizeList), AOCFileOrParseError> {
    if let Ok(ret) = day6_parser::parse(input) {
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
        let (actual_numbers, actual_operators) =
            day6_parser::parse(&input_str).expect("Should parse successfully");
        let expected_numbers: Vec<String> = vec![
            "123 328  51 64 ".to_string(),
            " 45 64  387 23 ".to_string(),
            "  6 98  215 314".to_string(),
        ];
        let expected_operators: Vec<(Operator, usize)> = vec![
            (Operator::Multiply, 2),
            (Operator::Add, 2),
            (Operator::Multiply, 2),
            (Operator::Add, 2),
        ];
        assert_eq!(expected_numbers, actual_numbers);
        assert_eq!(expected_operators, actual_operators);
    }
}
