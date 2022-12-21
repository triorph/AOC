extern crate peg;
use std::collections::HashMap;

use crate::types::Operation;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day21_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule name() -> String
        = n:$(['a'..='z']+) { n.to_string() }
    rule add() -> Operation
        = left:name() ( " + " ) right:name() { Operation::Add(left, right) }
    rule subtract() -> Operation
        = left:name() ( " - " ) right:name() { Operation::Subtract(left, right) }
    rule multiply() -> Operation
        = left:name() ( " * " ) right:name() { Operation::Multiply(left, right) }
    rule divide() -> Operation
        = left:name() ( " / " ) right:name() { Operation::Divide(left, right) }
    rule operation_number() -> Operation
        = n:number() { Operation::Literal(n as isize) }
    rule operation() -> Operation
        = operation:(add() / multiply() / divide() / subtract() / operation_number()) { operation }
    rule monkey() -> (String, Operation)
        = name:name() ": " operation:operation() { (name, operation) }
    pub rule parse() -> HashMap<String, Operation>
        = monkeys:monkey() ++ ("\n") "\n" * {
            let mut ret = HashMap::new();
            for monkey in monkeys.into_iter() {
                ret.insert(monkey.0, monkey.1);
            }
            ret
        }
}}

pub fn parse_data(input: &str) -> Result<HashMap<String, Operation>, AOCFileOrParseError> {
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
        assert_eq!(actual.len(), 15);
    }
}
