extern crate peg;
use crate::poker_hand::{PokerCard, PokerHand};
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day6_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule poker_card_ace() -> PokerCard
        = "A" { PokerCard::Ace }
    rule poker_card_king() -> PokerCard
        = "K" { PokerCard::King }
    rule poker_card_queen() -> PokerCard
        = "Q" { PokerCard::Queen }
    rule poker_card_jack() -> PokerCard
        = "J" { PokerCard::Jack }
    rule poker_card_ten() -> PokerCard
        = "T" { PokerCard::Ten }
    rule poker_card_nine() -> PokerCard
        = "9" { PokerCard::Nine }
    rule poker_card_eight() -> PokerCard
        = "8" { PokerCard::Eight }
    rule poker_card_seven() -> PokerCard
        = "7" { PokerCard::Seven }
    rule poker_card_six() -> PokerCard
        = "6" { PokerCard::Six }
    rule poker_card_five() -> PokerCard
        = "5" { PokerCard::Five }
    rule poker_card_four() -> PokerCard
        = "4" { PokerCard::Four }
    rule poker_card_three() -> PokerCard
        = "3" { PokerCard::Three }
    rule poker_card_two() -> PokerCard
        = "2" { PokerCard::Two }
    rule poker_card() -> PokerCard
        = card:(
            poker_card_ace() / poker_card_king() / poker_card_queen() /
            poker_card_jack() / poker_card_ten() / poker_card_nine() /
            poker_card_eight() / poker_card_seven() / poker_card_six() /
            poker_card_five() / poker_card_four() / poker_card_three() /
            poker_card_two())
        {
            card
        }
    rule poker_hand() -> PokerHand
        = poker_hand:poker_card() **<5> "" {
            PokerHand::new(&poker_hand)
    }
    rule line() -> (PokerHand, usize)
        = poker_hand:poker_hand() " " bet_value:number() { (poker_hand, bet_value) }
    pub rule parse() -> Vec<(PokerHand, usize)>
        = lines:line() ++ ("\n" +) "\n" * { lines }
}}

pub fn parse_data(input: &str) -> Result<Vec<(PokerHand, usize)>, AOCFileOrParseError> {
    if let Ok(ret) = day6_parser::parse(input) {
        Ok(ret)
    } else {
        Err(AOCFileOrParseError)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::poker_hand::PokerValue;
    use pretty_assertions::assert_eq;

    use aoc_helpers::read_input_file;

    #[test]
    fn test_parse() {
        let input_str = read_input_file("data/test_data.txt").unwrap();
        let actual = day6_parser::parse(&input_str).expect("Should parse successfully");
        let expected: Vec<(PokerHand, usize)> = vec![
            (
                PokerHand::raw(
                    [
                        PokerCard::Three,
                        PokerCard::Two,
                        PokerCard::Ten,
                        PokerCard::Three,
                        PokerCard::King,
                    ],
                    PokerValue::OnePair,
                    PokerValue::OnePair,
                ),
                765,
            ),
            (
                PokerHand::raw(
                    [
                        PokerCard::Ten,
                        PokerCard::Five,
                        PokerCard::Five,
                        PokerCard::Jack,
                        PokerCard::Five,
                    ],
                    PokerValue::ThreeOfAKind,
                    PokerValue::FourOfAKind,
                ),
                684,
            ),
            (
                PokerHand::raw(
                    [
                        PokerCard::King,
                        PokerCard::King,
                        PokerCard::Six,
                        PokerCard::Seven,
                        PokerCard::Seven,
                    ],
                    PokerValue::TwoPairs,
                    PokerValue::TwoPairs,
                ),
                28,
            ),
            (
                PokerHand::raw(
                    [
                        PokerCard::King,
                        PokerCard::Ten,
                        PokerCard::Jack,
                        PokerCard::Jack,
                        PokerCard::Ten,
                    ],
                    PokerValue::TwoPairs,
                    PokerValue::FourOfAKind,
                ),
                220,
            ),
            (
                PokerHand::raw(
                    [
                        PokerCard::Queen,
                        PokerCard::Queen,
                        PokerCard::Queen,
                        PokerCard::Jack,
                        PokerCard::Ace,
                    ],
                    PokerValue::ThreeOfAKind,
                    PokerValue::FourOfAKind,
                ),
                483,
            ),
        ];
        assert_eq!(expected, actual)
    }
}
