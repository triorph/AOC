use std::cmp::{Ordering, PartialOrd};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub struct PokerHand {
    cards: [PokerCard; 5],
    value_a: PokerValue,
    value_b: PokerValue,
}

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum PokerValue {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPairs,
    OnePair,
    AllDifferent,
}

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
pub enum PokerCard {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

const ALL_POKER_VALUES_IN_ORDER: [PokerValue; 7] = [
    PokerValue::FiveOfAKind,
    PokerValue::FourOfAKind,
    PokerValue::FullHouse,
    PokerValue::ThreeOfAKind,
    PokerValue::TwoPairs,
    PokerValue::OnePair,
    PokerValue::AllDifferent,
];

const ALL_POKER_CARDS_IN_DAY_A_ORDER: [PokerCard; 13] = [
    PokerCard::Ace,
    PokerCard::King,
    PokerCard::Queen,
    PokerCard::Jack,
    PokerCard::Ten,
    PokerCard::Nine,
    PokerCard::Eight,
    PokerCard::Seven,
    PokerCard::Six,
    PokerCard::Five,
    PokerCard::Four,
    PokerCard::Three,
    PokerCard::Two,
];

const ALL_POKER_CARDS_IN_DAY_B_ORDER: [PokerCard; 13] = [
    PokerCard::Ace,
    PokerCard::King,
    PokerCard::Queen,
    PokerCard::Ten,
    PokerCard::Nine,
    PokerCard::Eight,
    PokerCard::Seven,
    PokerCard::Six,
    PokerCard::Five,
    PokerCard::Four,
    PokerCard::Three,
    PokerCard::Two,
    PokerCard::Jack,
];

impl PartialOrd for PokerValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }
        for biggest in ALL_POKER_VALUES_IN_ORDER.iter() {
            if self == biggest {
                return Some(Ordering::Greater);
            } else if other == biggest {
                return Some(Ordering::Less);
            }
        }
        None // Shouldn't happen
    }
}

impl PokerHand {
    pub fn new(cards: &[PokerCard]) -> PokerHand {
        PokerHand {
            cards: cards.try_into().expect("Wrong number of poker cards"),
            value_a: PokerValue::from_day_a(cards),
            value_b: PokerValue::from_day_b(cards),
        }
    }

    #[cfg(test)]
    pub fn from_string(cards: &str) -> PokerHand {
        let cards = cards
            .chars()
            .map(PokerCard::from_char)
            .collect::<Vec<PokerCard>>()
            .try_into()
            .expect("Will be of size 5");
        PokerHand {
            cards,
            value_a: PokerValue::from_day_a(&cards),
            value_b: PokerValue::from_day_b(&cards),
        }
    }

    #[cfg(test)]
    pub fn raw(cards: [PokerCard; 5], value_a: PokerValue, value_b: PokerValue) -> PokerHand {
        PokerHand {
            cards,
            value_a,
            value_b,
        }
    }
    pub fn compare_day_a(&self, other: &PokerHand) -> Ordering {
        match self.value_a.partial_cmp(&other.value_a) {
            Some(Ordering::Less) => Ordering::Less,
            Some(Ordering::Greater) => Ordering::Greater,
            _ => self.compare_cards_day_a(other),
        }
    }

    fn compare_cards_day_a(&self, other: &PokerHand) -> Ordering {
        for (card, other_card) in self.cards.iter().zip(other.cards.iter()) {
            let interim_cmp = card.compare_day_a(other_card);
            if interim_cmp != Ordering::Equal {
                return interim_cmp;
            }
        }
        Ordering::Equal
    }

    pub fn compare_day_b(&self, other: &PokerHand) -> Ordering {
        match self.value_b.partial_cmp(&other.value_b) {
            Some(Ordering::Less) => Ordering::Less,
            Some(Ordering::Greater) => Ordering::Greater,
            _ => self.compare_cards_day_b(other),
        }
    }

    fn compare_cards_day_b(&self, other: &PokerHand) -> Ordering {
        for (card, other_card) in self.cards.iter().zip(other.cards.iter()) {
            let interim_cmp = card.compare_day_b(other_card);
            if interim_cmp != Ordering::Equal {
                return interim_cmp;
            }
        }
        Ordering::Equal
    }
}

impl PokerValue {
    fn build_frequencies(cards: &[PokerCard]) -> HashMap<PokerCard, usize> {
        let mut frequencies: HashMap<PokerCard, usize> = HashMap::new();
        for card in cards.iter() {
            *(frequencies.entry(*card).or_insert(0)) += 1;
        }
        frequencies
    }

    fn build_frequencies_and_fix_jokers(cards: &[PokerCard]) -> HashMap<PokerCard, usize> {
        let mut frequencies = PokerValue::build_frequencies(cards);
        let joker_value = frequencies.remove(&PokerCard::Jack).unwrap_or(0);
        let key_to_increase_by_joker = *frequencies
            .iter_mut()
            .max_by(|(_, first), (_, second)| first.cmp(second))
            .map(|(key, _)| key)
            .unwrap_or(&PokerCard::Jack);
        *(frequencies.entry(key_to_increase_by_joker).or_insert(0)) += joker_value;
        frequencies
    }

    fn get_value_from_frequencies(frequencies: HashMap<PokerCard, usize>) -> PokerValue {
        match frequencies
            .values()
            .filter(|value| value > &&1)
            .copied()
            .collect::<Vec<usize>>()
        {
            val if val == [5] => PokerValue::FiveOfAKind,
            val if val == [4] => PokerValue::FourOfAKind,
            val if val == [3, 2] => PokerValue::FullHouse,
            val if val == [2, 3] => PokerValue::FullHouse,
            val if val == [3] => PokerValue::ThreeOfAKind,
            val if val == [2, 2] => PokerValue::TwoPairs,
            val if val == [2] => PokerValue::OnePair,
            _ => PokerValue::AllDifferent,
        }
    }
    fn from_day_a(cards: &[PokerCard]) -> PokerValue {
        PokerValue::get_value_from_frequencies(PokerValue::build_frequencies(cards))
    }

    fn from_day_b(cards: &[PokerCard]) -> PokerValue {
        PokerValue::get_value_from_frequencies(PokerValue::build_frequencies_and_fix_jokers(cards))
    }
}

impl PokerCard {
    fn compare_day_a(&self, other: &PokerCard) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }
        for biggest in ALL_POKER_CARDS_IN_DAY_A_ORDER.iter() {
            if self == biggest {
                return Ordering::Greater;
            } else if other == biggest {
                return Ordering::Less;
            }
        }
        Ordering::Equal
    }

    fn compare_day_b(&self, other: &PokerCard) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }
        for biggest in ALL_POKER_CARDS_IN_DAY_B_ORDER.iter() {
            if self == biggest {
                return Ordering::Greater;
            } else if other == biggest {
                return Ordering::Less;
            }
        }
        Ordering::Equal
    }

    #[cfg(test)]
    fn from_char(c: char) -> PokerCard {
        match c {
            'A' => PokerCard::Ace,
            'K' => PokerCard::King,
            'Q' => PokerCard::Queen,
            'J' => PokerCard::Jack,
            'T' => PokerCard::Ten,
            '9' => PokerCard::Nine,
            '8' => PokerCard::Eight,
            '7' => PokerCard::Seven,
            '6' => PokerCard::Six,
            '5' => PokerCard::Five,
            '4' => PokerCard::Four,
            '3' => PokerCard::Three,
            '2' => PokerCard::Two,
            _ => panic!("Invalid input char"),
        }
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use crate::poker_hand::PokerCard;
    use crate::poker_hand::PokerHand;
    use crate::poker_hand::PokerValue;
    use std::cmp::Ordering;

    #[test]
    fn test_poker_value_derive_a() {
        for (input, result) in [
            ("22222", PokerValue::FiveOfAKind),
            ("22422", PokerValue::FourOfAKind),
            ("22424", PokerValue::FullHouse),
            ("24424", PokerValue::FullHouse),
            ("24434", PokerValue::ThreeOfAKind),
            ("24334", PokerValue::TwoPairs),
            ("24345", PokerValue::OnePair),
            ("A2354", PokerValue::AllDifferent),
        ]
        .into_iter()
        {
            assert_eq!(
                PokerValue::from_day_a(
                    &input
                        .chars()
                        .map(PokerCard::from_char)
                        .collect::<Vec<PokerCard>>()
                ),
                result
            );
        }
    }

    #[test]
    fn test_poker_value_derive_b() {
        for (input, result) in [
            ("22JJ2", PokerValue::FiveOfAKind),
            ("J24J2", PokerValue::FourOfAKind),
            ("J2424", PokerValue::FullHouse),
            ("24J24", PokerValue::FullHouse),
            ("24J34", PokerValue::ThreeOfAKind),
            ("24334", PokerValue::TwoPairs),
            ("2J345", PokerValue::OnePair),
            ("A2354", PokerValue::AllDifferent),
        ]
        .into_iter()
        {
            assert_eq!(
                PokerValue::from_day_b(
                    &input
                        .chars()
                        .map(PokerCard::from_char)
                        .collect::<Vec<PokerCard>>()
                ),
                result
            );
        }
    }

    #[test]
    fn test_day_a_comparisons() {
        for (first, second, result) in [
            ("22222", "AAAAA", Ordering::Less),
            ("33322", "22333", Ordering::Greater),
            ("23456", "23465", Ordering::Less),
            ("22354", "AKQJT", Ordering::Greater),
            ("22334", "AAKQJ", Ordering::Greater),
            ("22234", "AAKKQ", Ordering::Greater),
            ("22233", "AAAKQ", Ordering::Greater),
            ("22223", "AAAKK", Ordering::Greater),
            ("22222", "AAAAK", Ordering::Greater),
            ("AAAQQ", "AAAQQ", Ordering::Equal),
        ]
        .into_iter()
        {
            assert_eq!(
                PokerHand::from_string(first).compare_day_a(&PokerHand::from_string(second)),
                result,
                "{:?} was not {:?} to {:?}",
                first,
                result,
                second
            );
        }
    }

    #[test]
    fn test_day_b_comparisons() {
        for (first, second, result) in [
            ("22222", "AAAAJ", Ordering::Less),
            ("22222", "JAAAA", Ordering::Greater),
            ("33J22", "22333", Ordering::Greater),
            ("23456", "23465", Ordering::Less),
            ("J2354", "AKQ9T", Ordering::Greater),
            ("J2334", "AAKQ9", Ordering::Greater),
            ("J2234", "AAKKQ", Ordering::Greater),
            ("J2233", "AAAKQ", Ordering::Greater),
            ("J2223", "AAAKK", Ordering::Greater),
            ("J2222", "AAAAK", Ordering::Greater),
            ("JAAQQ", "JAAQQ", Ordering::Equal),
        ]
        .into_iter()
        {
            assert_eq!(
                PokerHand::from_string(first).compare_day_b(&PokerHand::from_string(second)),
                result,
                "{:?} was not {:?} to {:?}",
                first,
                result,
                second
            );
        }
    }
}
