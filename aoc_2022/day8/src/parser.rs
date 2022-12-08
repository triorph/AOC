extern crate peg;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day8_parser() for str {
    rule number() -> u8
        = n:$(['0'..='9']) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule number_line() -> Vec<u8>
        = line_of_numbers:number() ++ "" { line_of_numbers }
    pub rule parse() -> Vec<Vec<u8>>
        = lines_of_numbers:number_line() ++ ("\n") "\n" * {
             { lines_of_numbers }
        }
}}

pub fn parse_data(input: &str) -> Result<Vec<Vec<u8>>, AOCFileOrParseError> {
    if let Ok(ret) = day8_parser::parse(input) {
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
        let actual = day8_parser::parse(&input_str).expect("Should parse successfully");
        assert_eq!(actual[0].len(), 5);
    }
}
