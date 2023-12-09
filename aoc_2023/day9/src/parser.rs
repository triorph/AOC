extern crate peg;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day9_parser() for str {
    rule positive_number() -> isize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule negative_number() -> isize
        = "-" number: positive_number() { - number }
    rule number() -> isize
        = number: (positive_number() / negative_number()) { number }
    rule sequence() -> Vec<isize>
        = sequence:number() ++ " " { sequence }
    pub rule parse() -> Vec<Vec<isize>>
        = sequences:sequence() ++ ("\n" +) "\n" * { sequences }
}}

pub fn parse_data(input: &str) -> Result<Vec<Vec<isize>>, AOCFileOrParseError> {
    if let Ok(ret) = day9_parser::parse(input) {
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
        let actual = day9_parser::parse(&input_str).expect("Should parse successfully");
        let expected: Vec<Vec<isize>> = vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![1, 3, 6, 10, 15, 21],
            vec![10, 13, 16, 21, 30, 45],
        ];
        assert_eq!(expected, actual)
    }
}
