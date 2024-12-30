extern crate peg;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day21_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule line() -> Vec<usize>
        = line:number() ++ "" "A" { line }
    pub rule parse() -> Vec<Vec<usize>>
        = lines:line() ++ ("\n" +) "\n" * { lines }
}}

pub fn parse_data(input: &str) -> Result<Vec<Vec<usize>>, AOCFileOrParseError> {
    if let Ok(ret) = day21_parser::parse(input) {
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
        let actual = day21_parser::parse(&input_str).expect("Should parse successfully");
        let expected: Vec<Vec<usize>> = vec![
            vec![0, 2, 9],
            vec![9, 8, 0],
            vec![1, 7, 9],
            vec![4, 5, 6],
            vec![3, 7, 9],
        ];
        assert_eq!(expected, actual)
    }
}
