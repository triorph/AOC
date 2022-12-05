extern crate peg;
use crate::hand::Hand;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day2_parser() for str {
    rule Rock() -> Hand
        = ("A" / "X") { Hand::Rock }
    rule Paper() -> Hand
        = ("B" / "Y") { Hand::Paper }
    rule Scissors() -> Hand
        = ("C" / "Z") { Hand::Scissors }
    rule hand() -> Hand
        = hand:(Rock() / Scissors() / Paper()) { hand }
    rule hand_pair() -> (Hand, Hand)
        = h1:hand() (" ") h2:hand() { (h1, h2) }
    pub rule parse() -> Vec<(Hand, Hand)>
        = hand_pair:hand_pair() ++ ("\n" +) "\n" * {
             { hand_pair }
        }
}}

pub fn parse_data(input: &str) -> Result<Vec<(Hand, Hand)>, AOCFileOrParseError> {
    if let Ok(ret) = day2_parser::parse(input) {
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
        let actual = day2_parser::parse(&input_str).expect("Should parse successfully");
        let expected = vec![
            (Hand::Rock, Hand::Paper),
            (Hand::Paper, Hand::Rock),
            (Hand::Scissors, Hand::Scissors),
        ];
        assert_eq!(expected, actual)
    }
}
