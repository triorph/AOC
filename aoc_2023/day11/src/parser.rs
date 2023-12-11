extern crate peg;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day11_parser() for str {
    rule empty() -> bool
        = "." { false }
    rule galaxy() -> bool
        = "#" { true }
    rule location() -> bool
        = location: (empty() / galaxy()) { location}
    rule line() -> Vec<bool>
        = line:location() ++ ""
    pub rule parse() -> Vec<Vec<bool>>
        = lines:line() ++ ("\n" +) "\n" * { lines }
}}

pub fn parse_data(input: &str) -> Result<Vec<Vec<bool>>, AOCFileOrParseError> {
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
        let expected: Vec<Vec<bool>> = vec![
            vec![
                false, false, false, true, false, false, false, false, false, false,
            ],
            vec![
                false, false, false, false, false, false, false, true, false, false,
            ],
            vec![
                true, false, false, false, false, false, false, false, false, false,
            ],
            vec![
                false, false, false, false, false, false, false, false, false, false,
            ],
            vec![
                false, false, false, false, false, false, true, false, false, false,
            ],
            vec![
                false, true, false, false, false, false, false, false, false, false,
            ],
            vec![
                false, false, false, false, false, false, false, false, false, true,
            ],
            vec![
                false, false, false, false, false, false, false, false, false, false,
            ],
            vec![
                false, false, false, false, false, false, false, true, false, false,
            ],
            vec![
                true, false, false, false, true, false, false, false, false, false,
            ],
        ];
        assert_eq!(expected, actual)
    }
}
