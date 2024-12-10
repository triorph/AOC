extern crate peg;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day10_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule line() -> Vec<usize>
        = line:number() ++ "" { line }
    pub rule parse() -> Vec<Vec<usize>>
        = lines:line() ++ "\n" "\n" * { lines }
}}

pub fn parse_data(input: &str) -> Result<Vec<Vec<usize>>, AOCFileOrParseError> {
    if let Ok(ret) = day10_parser::parse(input) {
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
        let actual = day10_parser::parse(&input_str).expect("Should parse successfully");
        let expected: Vec<Vec<usize>> = vec![
            vec![8, 9, 0, 1, 0, 1, 2, 3],
            vec![7, 8, 1, 2, 1, 8, 7, 4],
            vec![8, 7, 4, 3, 0, 9, 6, 5],
            vec![9, 6, 5, 4, 9, 8, 7, 4],
            vec![4, 5, 6, 7, 8, 9, 0, 3],
            vec![3, 2, 0, 1, 9, 0, 1, 2],
            vec![0, 1, 3, 2, 9, 8, 0, 1],
            vec![1, 0, 4, 5, 6, 7, 3, 2],
        ];
        assert_eq!(expected, actual)
    }
}
