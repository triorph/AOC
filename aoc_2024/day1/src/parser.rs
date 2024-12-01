extern crate peg;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day01_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule number_pair() -> (usize, usize)
        = left:number() " " * right:number() { ( left, right ) }
    pub rule parse() -> Vec<(usize, usize)>
        = lines:number_pair()++ ("\n" +) "\n" * { lines }
}}

pub fn parse_data(input: &str) -> Result<Vec<(usize, usize)>, AOCFileOrParseError> {
    if let Ok(ret) = day01_parser::parse(input) {
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
        let actual = day01_parser::parse(&input_str).expect("Should parse successfully");
        let expected: Vec<(usize, usize)> = vec![(3, 4), (4, 3), (2, 5), (1, 3), (3, 9), (3, 3)];
        assert_eq!(expected, actual)
    }
}
