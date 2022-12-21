extern crate peg;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day20_parser() for str {
    rule positive_number() -> isize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule negative_number() -> isize
        = "-" n:positive_number() { - n }
    rule number() -> isize
        = n:(positive_number() / negative_number()) { n }
    pub rule parse() -> Vec<isize>
        = numbers:number() ++ ("\n") "\n" * {
             numbers
        }
}}

pub fn parse_data(input: &str) -> Result<Vec<isize>, AOCFileOrParseError> {
    if let Ok(ret) = day20_parser::parse(input) {
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
        let actual = day20_parser::parse(&input_str).expect("Should parse successfully");
        let expected: Vec<isize> = vec![1, 2, -3, 3, -2, 0, 4];
        assert_eq!(expected, actual)
    }
}
