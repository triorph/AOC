extern crate peg;
use crate::assignments::*;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day4_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule range() -> Range
        = start:number() "-" end:number() { Range{start, end} }
    rule assignment_pair() -> Assignment
        = l:range() "," r:range() { (l, r) }
    pub rule parse() -> Vec<Assignment>
        = lines:assignment_pair() ++ ("\n" +) "\n" * {
             { lines }
        }
}}

pub fn parse_data(input: &str) -> Result<Vec<Assignment>, AOCFileOrParseError> {
    if let Ok(ret) = day4_parser::parse(input) {
        Ok(ret)
    } else {
        Err(AOCFileOrParseError)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use aoc_helpers::read_input_file;
    #[test]
    fn test_parse() {
        let input_str = read_input_file("data/test_data.txt").unwrap();
        let actual = day4_parser::parse(&input_str).expect("Should parse successfully");
        assert_eq!(
            (Range { start: 2, end: 4 }, Range { start: 6, end: 8 }),
            actual[0]
        );
        assert_eq!(actual.len(), 6);
    }
}
