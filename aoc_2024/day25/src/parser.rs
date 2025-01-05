extern crate peg;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day25_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule hash() -> bool
        = "#"  { true }
    rule dot() -> bool
        = "."  { false }
    rule tile() -> bool
        = tile:(hash()/dot()) { tile }
    rule line() -> Vec<bool>
        = line:tile() ++ "" { line }
    rule lock_or_key() -> Vec<Vec<bool>>
        = lines:line() ++ "\n" { lines }
    pub rule parse() -> Vec<Vec<Vec<bool>>>
        = locks_or_keys:lock_or_key() ++ ("\n" +) "\n" * { locks_or_keys }
}}

pub fn parse_data(input: &str) -> Result<Vec<Vec<Vec<bool>>>, AOCFileOrParseError> {
    if let Ok(ret) = day25_parser::parse(input) {
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
        let actual = day25_parser::parse(&input_str).expect("Should parse successfully");
        let expected: Vec<Vec<Vec<bool>>> = vec![
            vec![
                vec![true, true, true, true, true],
                vec![false, true, true, true, true],
                vec![false, true, true, true, true],
                vec![false, true, true, true, true],
                vec![false, true, false, true, false],
                vec![false, true, false, false, false],
                vec![false, false, false, false, false],
            ],
            vec![
                vec![true, true, true, true, true],
                vec![true, true, false, true, true],
                vec![false, true, false, true, true],
                vec![false, false, false, true, true],
                vec![false, false, false, true, false],
                vec![false, false, false, true, false],
                vec![false, false, false, false, false],
            ],
            vec![
                vec![false, false, false, false, false],
                vec![true, false, false, false, false],
                vec![true, false, false, false, false],
                vec![true, false, false, false, true],
                vec![true, false, true, false, true],
                vec![true, false, true, true, true],
                vec![true, true, true, true, true],
            ],
            vec![
                vec![false, false, false, false, false],
                vec![false, false, false, false, false],
                vec![true, false, true, false, false],
                vec![true, true, true, false, false],
                vec![true, true, true, false, true],
                vec![true, true, true, false, true],
                vec![true, true, true, true, true],
            ],
            vec![
                vec![false, false, false, false, false],
                vec![false, false, false, false, false],
                vec![false, false, false, false, false],
                vec![true, false, false, false, false],
                vec![true, false, true, false, false],
                vec![true, false, true, false, true],
                vec![true, true, true, true, true],
            ],
        ];
        assert_eq!(expected, actual)
    }
}
