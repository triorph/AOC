extern crate peg;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day3_parser() for str {
    rule letter() -> char
        = n:$(['a'..='z' | 'A'..='Z']) { n.chars().next().expect("Must be exactly length 1") }
    rule rucksacks() -> Vec<char>
        = vals:letter() ++ "" {vals}
    pub rule parse() -> Vec<Vec<char>>
        = line_of_rucksacks:rucksacks() ++ ("\n" +) "\n" * {
             { line_of_rucksacks }
        }
}}

pub fn parse_data(input: &str) -> Result<Vec<Vec<char>>, AOCFileOrParseError> {
    if let Ok(ret) = day3_parser::parse(input) {
        Ok(ret)
    } else {
        Err(AOCFileOrParseError)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use aoc_helpers::read_input_file;
    #[test]
    fn test_parse() {
        let input_str = read_input_file("data/test_data.txt").unwrap();
        let actual = day3_parser::parse(&input_str).expect("Should parse successfully");
        assert_eq!(actual.len(), 6)
    }
}
