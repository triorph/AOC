extern crate peg;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day3_parser() for str {
    rule battery() -> usize
        = n:$(['0'..='9']) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule power_bank() -> Vec<usize>
        = batteries:battery() ++ "" { batteries }
    pub rule parse() -> Vec<Vec<usize>>
        = lines:power_bank() ++ ("\n" +) "\n" * { lines }
}}

pub fn parse_data(input: &str) -> Result<Vec<Vec<usize>>, AOCFileOrParseError> {
    if let Ok(ret) = day3_parser::parse(input) {
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
        let actual = day3_parser::parse(&input_str).expect("Should parse successfully");
        let expected: Vec<Vec<usize>> = vec![
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
            vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
            vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
            vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
        ];
        assert_eq!(expected, actual)
    }
}
