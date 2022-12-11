extern crate peg;
use aoc_helpers::AOCFileOrParseError;

#[derive(Debug, PartialEq)]
enum Operation {
    Add(usize),
    Multiply(usize),
}

peg::parser! { pub grammar day1_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule starting_items() -> Vec<usize>
        = " "+ "Starting items: " items:number() ++ ", " "\n"+ { items }
    rule add() -> Operation
        = "+ " n:number() { Operation::Add(n) }
    rule multiply() -> Operation
        = "* " n:number() { Operation::Multiply(n) }
    rule operation() -> Operation
        = " "+ "Operation: new = old " operation:(add() / multiply()) "\n"+ { operation }
    rule test_condition() -> usize
        = " "+ "Test: divisible by " n:number() { n }
    rule true_case() -> usize
        = " "+ "If true: throw to monkey " n: number() { n }
    rule false_case() -> usize
        = " "+ "If false: throw to monkey " n:number() { n }
    rule calory_package() -> Vec<usize>
        = calories:number() ++ ("\n")
    pub rule parse() -> Vec<Vec<usize>>
        = all_elf_calories:calory_package() ++ ("\n" +) "\n" * {
             { all_elf_calories }
        }
}}

pub fn parse_data(input: &str) -> Result<Vec<Vec<usize>>, AOCFileOrParseError> {
    if let Ok(ret) = day1_parser::parse(input) {
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
        let actual = day1_parser::parse(&input_str).expect("Should parse successfully");
        let expected = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];
        assert_eq!(expected, actual)
    }
}
