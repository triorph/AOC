extern crate peg;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day8_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule entry() -> char
        = !"\n" c:[_] { c }
    rule line() -> Vec<char>
        = line:entry() ++ "" { line }
    pub rule parse() -> Vec<Vec<char>>
        = lines:line() ++ ("\n") "\n" * { lines }
}}

pub fn parse_data(input: &str) -> Result<Vec<Vec<char>>, AOCFileOrParseError> {
    if let Ok(ret) = day8_parser::parse(input) {
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
        let actual = day8_parser::parse(&input_str).expect("Should parse successfully");
        let expected: Vec<Vec<char>> = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '0', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '0', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '0', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '0', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', 'A', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', 'A', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', 'A', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];
        assert_eq!(expected, actual)
    }
}
