extern crate peg;
use aoc_helpers::AOCFileOrParseError;
use std::ops::Range;

peg::parser! {
    pub grammar day2_parser() for str {
        rule number() -> usize
            = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
        rule range() -> Range<usize>
            = start:number() "-" end:number() { start..end }
        pub rule parse() -> Vec<Range<usize>>
            = ranges:range() ++ (['\n'|','] +) "\n" * { ranges }
    }
}

pub fn parse_data(input: &str) -> Result<Vec<Range<usize>>, AOCFileOrParseError> {
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
        let expected: Vec<Range<usize>> = vec![
            11..22,
            95..115,
            998..1012,
            1188511880..1188511890,
            222220..222224,
            1698522..1698528,
            446443..446449,
            38593856..38593862,
            565653..565659,
            824824821..824824827,
            2121212118..2121212124,
        ];
        assert_eq!(expected, actual)
    }
}
