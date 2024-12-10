extern crate peg;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day9_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    pub rule parse() -> Vec<usize>
        = numbers:number() ++ "" "\n" * { numbers }
}}

pub fn parse_data(input: &str) -> Result<Vec<usize>, AOCFileOrParseError> {
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
        let expected: Vec<usize> = vec![2, 3, 3, 3, 1, 3, 3, 1, 2, 1, 4, 1, 4, 1, 3, 1, 4, 0, 2];
        assert_eq!(expected, actual)
    }
}
