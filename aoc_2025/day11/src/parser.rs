extern crate peg;
use aoc_helpers::AOCFileOrParseError;
use std::collections::HashMap;

peg::parser! { pub grammar day11_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().unwrap_or_else(|_| panic!("Was expecting a number string {}", n))}
    rule node() -> String
        = node:$(['a'..='z']+) { node.to_string() }
    rule targets() -> Vec<String>
        = targets:node() ++ " " { targets }
    rule line() -> (String, Vec<String>)
        = source:node() ": " targets:targets() { (source, targets)}
    pub rule parse() -> HashMap<String, Vec<String>>
        = lines:line() ++ ("\n" +) "\n" * { HashMap::from_iter(lines.into_iter()) }
}}

pub fn parse_data(input: &str) -> Result<HashMap<String, Vec<String>>, AOCFileOrParseError> {
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
        let expected: HashMap<String, Vec<String>> = HashMap::from([
            (
                "aaa".to_string(),
                vec!["you".to_string(), "hhh".to_string()],
            ),
            (
                "you".to_string(),
                vec!["bbb".to_string(), "ccc".to_string()],
            ),
            (
                "bbb".to_string(),
                vec!["ddd".to_string(), "eee".to_string()],
            ),
            (
                "ccc".to_string(),
                vec!["ddd".to_string(), "eee".to_string(), "fff".to_string()],
            ),
            ("ddd".to_string(), vec!["ggg".to_string()]),
            ("eee".to_string(), vec!["out".to_string()]),
            ("fff".to_string(), vec!["out".to_string()]),
            ("ggg".to_string(), vec!["out".to_string()]),
            (
                "hhh".to_string(),
                vec!["ccc".to_string(), "fff".to_string(), "iii".to_string()],
            ),
            ("iii".to_string(), vec!["out".to_string()]),
        ]);
        assert_eq!(expected, actual)
    }
}
