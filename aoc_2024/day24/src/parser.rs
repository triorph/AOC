extern crate peg;
use aoc_helpers::AOCFileOrParseError;
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub enum Operation {
    Or,
    Xor,
    And,
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Or => write!(f, "OR"),
            Operation::Xor => write!(f, "XOR"),
            Operation::And => write!(f, "AND"),
        }
    }
}

pub type OperationLine = (String, Operation, String, String);

peg::parser! { pub grammar day24_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule variable() -> String
        = v:$(['a'..='z'|'0'..='9']+) { v.to_string() }
    rule initial_value() -> (String, usize)
        = name:variable() ": " value:number() { (name, value) }
    rule initial_values() -> HashMap<String, usize>
        = initial_values:initial_value() ++ "\n" { HashMap::from_iter(initial_values) }
    rule or() -> Operation
        = "OR" { Operation::Or }
    rule xor() -> Operation
        = "XOR" { Operation::Xor }
    rule and() -> Operation
        = "AND" { Operation::And }
    rule operation() -> Operation
        = operation:(or() / xor() / and()) { operation }
    rule operation_line() -> OperationLine
        = in1:variable() " " op:operation() " " in2:variable() " -> " out:variable() {
            (in1, op, in2, out)
        }
    rule operation_lines() -> Vec<OperationLine>
        = operation_lines:operation_line() ++ "\n" { operation_lines }
    pub rule parse() -> (HashMap<String, usize>, Vec<OperationLine>)
        = initial_values:initial_values() "\n"+ operation_lines:operation_lines() "\n"* {
            (initial_values, operation_lines)
        }
}}

pub fn parse_data(
    input: &str,
) -> Result<(HashMap<String, usize>, Vec<OperationLine>), AOCFileOrParseError> {
    if let Ok(ret) = day24_parser::parse(input) {
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
        let actual = day24_parser::parse(&input_str).expect("Should parse successfully");
        let expected: (HashMap<String, usize>, Vec<OperationLine>) = (
            HashMap::from_iter([
                ("x00".to_string(), 1),
                ("x01".to_string(), 0),
                ("x02".to_string(), 1),
                ("x03".to_string(), 1),
                ("x04".to_string(), 0),
                ("y00".to_string(), 1),
                ("y01".to_string(), 1),
                ("y02".to_string(), 1),
                ("y03".to_string(), 1),
                ("y04".to_string(), 1),
            ]),
            vec![
                (
                    "ntg".to_string(),
                    Operation::Xor,
                    "fgs".to_string(),
                    "mjb".to_string(),
                ),
                (
                    "y02".to_string(),
                    Operation::Or,
                    "x01".to_string(),
                    "tnw".to_string(),
                ),
                (
                    "kwq".to_string(),
                    Operation::Or,
                    "kpj".to_string(),
                    "z05".to_string(),
                ),
                (
                    "x00".to_string(),
                    Operation::Or,
                    "x03".to_string(),
                    "fst".to_string(),
                ),
                (
                    "tgd".to_string(),
                    Operation::Xor,
                    "rvg".to_string(),
                    "z01".to_string(),
                ),
                (
                    "vdt".to_string(),
                    Operation::Or,
                    "tnw".to_string(),
                    "bfw".to_string(),
                ),
                (
                    "bfw".to_string(),
                    Operation::And,
                    "frj".to_string(),
                    "z10".to_string(),
                ),
                (
                    "ffh".to_string(),
                    Operation::Or,
                    "nrd".to_string(),
                    "bqk".to_string(),
                ),
                (
                    "y00".to_string(),
                    Operation::And,
                    "y03".to_string(),
                    "djm".to_string(),
                ),
                (
                    "y03".to_string(),
                    Operation::Or,
                    "y00".to_string(),
                    "psh".to_string(),
                ),
                (
                    "bqk".to_string(),
                    Operation::Or,
                    "frj".to_string(),
                    "z08".to_string(),
                ),
                (
                    "tnw".to_string(),
                    Operation::Or,
                    "fst".to_string(),
                    "frj".to_string(),
                ),
                (
                    "gnj".to_string(),
                    Operation::And,
                    "tgd".to_string(),
                    "z11".to_string(),
                ),
                (
                    "bfw".to_string(),
                    Operation::Xor,
                    "mjb".to_string(),
                    "z00".to_string(),
                ),
                (
                    "x03".to_string(),
                    Operation::Or,
                    "x00".to_string(),
                    "vdt".to_string(),
                ),
                (
                    "gnj".to_string(),
                    Operation::And,
                    "wpb".to_string(),
                    "z02".to_string(),
                ),
                (
                    "x04".to_string(),
                    Operation::And,
                    "y00".to_string(),
                    "kjc".to_string(),
                ),
                (
                    "djm".to_string(),
                    Operation::Or,
                    "pbm".to_string(),
                    "qhw".to_string(),
                ),
                (
                    "nrd".to_string(),
                    Operation::And,
                    "vdt".to_string(),
                    "hwm".to_string(),
                ),
                (
                    "kjc".to_string(),
                    Operation::And,
                    "fst".to_string(),
                    "rvg".to_string(),
                ),
                (
                    "y04".to_string(),
                    Operation::Or,
                    "y02".to_string(),
                    "fgs".to_string(),
                ),
                (
                    "y01".to_string(),
                    Operation::And,
                    "x02".to_string(),
                    "pbm".to_string(),
                ),
                (
                    "ntg".to_string(),
                    Operation::Or,
                    "kjc".to_string(),
                    "kwq".to_string(),
                ),
                (
                    "psh".to_string(),
                    Operation::Xor,
                    "fgs".to_string(),
                    "tgd".to_string(),
                ),
                (
                    "qhw".to_string(),
                    Operation::Xor,
                    "tgd".to_string(),
                    "z09".to_string(),
                ),
                (
                    "pbm".to_string(),
                    Operation::Or,
                    "djm".to_string(),
                    "kpj".to_string(),
                ),
                (
                    "x03".to_string(),
                    Operation::Xor,
                    "y03".to_string(),
                    "ffh".to_string(),
                ),
                (
                    "x00".to_string(),
                    Operation::Xor,
                    "y04".to_string(),
                    "ntg".to_string(),
                ),
                (
                    "bfw".to_string(),
                    Operation::Or,
                    "bqk".to_string(),
                    "z06".to_string(),
                ),
                (
                    "nrd".to_string(),
                    Operation::Xor,
                    "fgs".to_string(),
                    "wpb".to_string(),
                ),
                (
                    "frj".to_string(),
                    Operation::Xor,
                    "qhw".to_string(),
                    "z04".to_string(),
                ),
                (
                    "bqk".to_string(),
                    Operation::Or,
                    "frj".to_string(),
                    "z07".to_string(),
                ),
                (
                    "y03".to_string(),
                    Operation::Or,
                    "x01".to_string(),
                    "nrd".to_string(),
                ),
                (
                    "hwm".to_string(),
                    Operation::And,
                    "bqk".to_string(),
                    "z03".to_string(),
                ),
                (
                    "tgd".to_string(),
                    Operation::Xor,
                    "rvg".to_string(),
                    "z12".to_string(),
                ),
                (
                    "tnw".to_string(),
                    Operation::Or,
                    "pbm".to_string(),
                    "gnj".to_string(),
                ),
            ],
        );
        assert_eq!(expected, actual)
    }
}
