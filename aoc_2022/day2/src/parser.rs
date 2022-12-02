extern crate peg;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day2_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    pub rule parse() -> Vec<usize>
        = lines_of_numbers:number() ++ ("\n" +) "\n" * {
             { lines_of_numbers }
        }
}}

pub fn parse_data(input: &str) -> Result<(), AOCFileOrParseError> {
    if let Ok(_ret) = day2_parser::parse(input) {
        Ok(())
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
        let actual = day2_parser::parse(&input_str).expect("Should parse successfully");
        let expected: Vec<usize> = vec![];
        assert_eq!(expected, actual)
    }
}
