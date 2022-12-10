extern crate peg;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day10_parser() for str {
    rule positive_number() -> isize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule negative_number() -> isize
        = "-" n:positive_number() { - n }
    rule number() -> isize
        = n:(positive_number() / negative_number()) { n }
    rule noop() -> Option<isize>
        = "noop" { None }
    rule addx() -> Option<isize>
        = "addx " n:number() { Some(n) }
    rule instruction() -> Option<isize>
        = instruction:(noop() / addx()) { instruction }
    pub rule parse() -> Vec<Option<isize>>
        = line_of_instructions:instruction() ++ ("\n" +) "\n" * {
             { line_of_instructions }
        }
}}

pub fn parse_data(input: &str) -> Result<Vec<Option<isize>>, AOCFileOrParseError> {
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
        // let expected: Vec<usize> = vec![];
        // assert_eq!(expected, actual)
    }
}
