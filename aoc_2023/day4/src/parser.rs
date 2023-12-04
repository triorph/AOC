extern crate peg;
use crate::card::Card;
use std::collections::HashSet;

use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day4_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule card() -> Card
        = "Card" " "* number() ":" " "* winners:number() ++ (" "+) " "* "|" " "* values:number() ++ (" "+) {
            (HashSet::from_iter(winners.into_iter()), HashSet::from_iter(values.into_iter()))
        }
    pub rule parse() -> Vec<Card>
        = lines_of_cards:card() ++ ("\n" +) "\n" * { lines_of_cards }
}}

pub fn parse_data(input: &str) -> Result<Vec<Card>, AOCFileOrParseError> {
    if let Ok(ret) = day4_parser::parse(input) {
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
        let actual = day4_parser::parse(&input_str).expect("Should parse successfully");
        assert_eq!(6, actual.len())
    }
}
