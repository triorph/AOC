extern crate peg;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day6_parser() for str {
    pub rule parse() -> String
        = line:$(['a'..='z']+) "\n" * {
             { line.to_string() }
        }
}}

pub fn parse_data(input: &str) -> Result<String, AOCFileOrParseError> {
    if let Ok(ret) = day6_parser::parse(input) {
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
        let actual = day6_parser::parse(&input_str).expect("Should parse successfully");
        let expected = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string();
        assert_eq!(expected, actual)
    }
}
