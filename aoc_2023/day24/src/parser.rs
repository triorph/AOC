extern crate peg;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day24_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    pub rule parse() -> Vec<usize>
        = lines:number() ++ ("\n" +) "\n" * { lines }
}}

pub fn parse_data(input: &str) -> Result<Vec<usize>, AOCFileOrParseError> {
    if let Ok(ret) = day24_parser::parse(input) {
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
        let actual = day24_parser::parse(&input_str).expect("Should parse successfully");
        let expected: Vec<usize> = vec![];
        assert_eq!(expected, actual)
    }
}
