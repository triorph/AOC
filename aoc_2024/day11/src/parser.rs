extern crate peg;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day11_parser() for str {
    rule number() -> u64
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    pub rule parse() -> Vec<u64>
        = numbers:number() ++ (" " +) "\n" * { numbers }
}}

pub fn parse_data(input: &str) -> Result<Vec<u64>, AOCFileOrParseError> {
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
        let expected: Vec<u64> = vec![125, 17];
        assert_eq!(expected, actual)
    }
}
