extern crate peg;
use crate::monkey::{Monkey, Operation};
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day11_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) {
            n.parse().expect(&format!("Was expecting a number string {}", n)[..])
        }
    rule monkey_header() -> usize
        = "Monkey " n:number() ":" "\n" { n }
    rule starting_items() -> Vec<usize>
        = " "+ "Starting items: " items:number() ++ ", " "\n" { items }
    rule add() -> Operation
        = "+ " n:number() { Operation::Add(n) }
    rule add_self() -> Operation
        = "+ old" { Operation::AddSelf }
    rule multiply() -> Operation
        = "* " n:number() { Operation::Multiply(n) }
    rule multiply_self() -> Operation
        = "* old" { Operation::MultiplySelf }
    rule operation() -> Operation
        = " "+ "Operation: new = old "
            operation:(add() / add_self() / multiply() / multiply_self()) "\n" {
                operation
            }
    rule test_condition() -> usize
        = " "+ "Test: divisible by " n:number() "\n" { n }
    rule true_case() -> usize
        = " "+ "If true: throw to monkey " n: number() "\n" { n }
    rule false_case() -> usize
        = " "+ "If false: throw to monkey " n:number() "\n" { n }
    rule monkey() -> Monkey
        = index:monkey_header()
            starting_items:starting_items()
            operation:operation()
            test_condition:test_condition()
            true_case:true_case()
            false_case:false_case() {
                Monkey{
                    index,
                    starting_items,
                    operation,
                    test_condition,
                    true_case,
                    false_case,
                    items_processed:0
                }
            }
    pub rule parse() -> Vec<Monkey>
        = monkeys:monkey() ++ ("\n" +) "\n" * {
            monkeys
        }
}}

pub fn parse_data(input: &str) -> Result<Vec<Monkey>, AOCFileOrParseError> {
    if let Ok(ret) = day11_parser::parse(input) {
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
        let actual = day11_parser::parse(&input_str).expect("Should parse successfully");
        assert_eq!(actual.len(), 4)
    }
}
