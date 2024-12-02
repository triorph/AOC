extern crate peg;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day2_parser() for str {
    rule number() -> isize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule line() -> Vec<isize>
        = line:number() ++ (" " +) " " * { line }
    pub rule parse() -> Vec<Vec<isize>>
        = lines:line() ++ ("\n" +) "\n" * { lines }
}}

pub fn parse_data(input: &str) -> Result<Vec<Vec<isize>>, AOCFileOrParseError> {
    if let Ok(ret) = day2_parser::parse(input) {
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
        let actual = day2_parser::parse(&input_str).expect("Should parse successfully");
        let expected: Vec<Vec<isize>> = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        assert_eq!(expected, actual)
    }
}
