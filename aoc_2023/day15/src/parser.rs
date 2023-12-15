extern crate peg;
use crate::step::{Operation, Step};
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day15_parser() for str {
    rule equals() -> Operation
        = "=" n:number() { Operation::Equals(n) }
    rule subtract() -> Operation
        = "-" { Operation::Subtract }
    rule operation() -> Operation
        = operation:(equals() / subtract()) { operation }
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule step() -> Step
        = label:$([^','|'\n'|'='|'-'|'0'..='9']+) operation:operation() {
            Step { label: label.to_string(), operation }
        }
    pub rule parse() -> Vec<Step>
        = steps:step() ++ ("," +) "," * "\n" * { steps }
}}

pub fn parse_data(input: &str) -> Result<Vec<Step>, AOCFileOrParseError> {
    if let Ok(ret) = day15_parser::parse(input) {
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
        let actual = day15_parser::parse(&input_str).expect("Should parse successfully");
        let expected: Vec<Step> = vec![
            Step {
                label: "rn".to_string(),
                operation: Operation::Equals(1),
            },
            Step {
                label: "cm".to_string(),
                operation: Operation::Subtract,
            },
            Step {
                label: "qp".to_string(),
                operation: Operation::Equals(3),
            },
            Step {
                label: "cm".to_string(),
                operation: Operation::Equals(2),
            },
            Step {
                label: "qp".to_string(),
                operation: Operation::Subtract,
            },
            Step {
                label: "pc".to_string(),
                operation: Operation::Equals(4),
            },
            Step {
                label: "ot".to_string(),
                operation: Operation::Equals(9),
            },
            Step {
                label: "ab".to_string(),
                operation: Operation::Equals(5),
            },
            Step {
                label: "pc".to_string(),
                operation: Operation::Subtract,
            },
            Step {
                label: "pc".to_string(),
                operation: Operation::Equals(6),
            },
            Step {
                label: "ot".to_string(),
                operation: Operation::Equals(7),
            },
        ];
        assert_eq!(expected, actual)
    }
}
