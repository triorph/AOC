use std::cmp::{Ordering, PartialOrd};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub struct PokerHand {
    cards: [PokerCard; 5],
    value_a: PokerValue,
    value_b: PokerValue,
}

impl PokerHand {
    pub fn compare_hand_day_a(&self, other: &PokerHand) -> Ordering {
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
        return Ordering::Equal;
    }

    pub fn compare_hand_day_b(&self, other: &PokerHand) -> Ordering {
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
        return Ordering::Equal;
    }
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

const ALL_POKER_VALUES_IN_ORDER: [PokerValue; 7] = [
    PokerValue::FiveOfAKind,
    PokerValue::FourOfAKind,
    PokerValue::FullHouse,
    PokerValue::ThreeOfAKind,
    PokerValue::TwoPairs,
    PokerValue::OnePair,
    PokerValue::AllDifferent,
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
    pub fn raw(cards: [PokerCard; 5], value_a: PokerValue, value_b: PokerValue) -> PokerHand {
        PokerHand {
            cards,
            value_a,
            value_b,
        }
    }
}

impl PokerValue {
    fn build_frequencies(cards: &[PokerCard]) -> HashMap<PokerCard, usize> {
        let mut frequencies: HashMap<PokerCard, usize> = HashMap::new();
        for card in cards.iter() {
            *(frequencies.entry(*card).or_insert(0)) += 1
        }
        frequencies
    }

    fn build_frequencies_and_fix_jokers(cards: &[PokerCard]) -> HashMap<PokerCard, usize> {
        let mut frequencies = PokerValue::build_frequencies(cards);
        let joker_value = frequencies.remove(&PokerCard::Jack).unwrap_or(0);
        let key_to_increase_by_joker = (*frequencies
            .iter_mut()
            .max_by(|(_, first), (_, second)| first.cmp(second))
            .map(|(key, _)| key)
            .unwrap_or(&PokerCard::Jack))
        .clone();
        frequencies
            .entry(key_to_increase_by_joker)
            .and_modify(|value| *value += joker_value)
            .or_insert(joker_value);
        frequencies
    }

    fn get_value_from_frequencies(frequencies: HashMap<PokerCard, usize>) -> PokerValue {
        match frequencies
            .values()
            .filter(|value| value > &&1)
            .copied()
            .collect::<Vec<usize>>()
        {
            val if &val == &[5] => PokerValue::FiveOfAKind,
            val if &val == &[4] => PokerValue::FourOfAKind,
            val if &val == &[3, 2] => PokerValue::FullHouse,
            val if &val == &[2, 3] => PokerValue::FullHouse,
            val if &val == &[3] => PokerValue::ThreeOfAKind,
            val if &val == &[2, 2] => PokerValue::TwoPairs,
            val if &val == &[2] => PokerValue::OnePair,
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
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use crate::poker_hand::PokerCard;
    use crate::poker_hand::PokerValue;

    #[test]
    fn test_poker_value_derive() {
        assert_eq!(
            PokerValue::from_day_a(&[
                PokerCard::Two,
                PokerCard::Two,
                PokerCard::Two,
                PokerCard::Two,
                PokerCard::Two
            ]),
            PokerValue::FiveOfAKind
        );
        assert_eq!(
            PokerValue::from_day_a(&[
                PokerCard::Two,
                PokerCard::Two,
                PokerCard::Four,
                PokerCard::Two,
                PokerCard::Two
            ]),
            PokerValue::FourOfAKind
        );
        assert_eq!(
            PokerValue::from_day_a(&[
                PokerCard::Two,
                PokerCard::Two,
                PokerCard::Four,
                PokerCard::Two,
                PokerCard::Four,
            ]),
            PokerValue::FullHouse
        );
        assert_eq!(
            PokerValue::from_day_a(&[
                PokerCard::Two,
                PokerCard::Four,
                PokerCard::Four,
                PokerCard::Two,
                PokerCard::Four,
            ]),
            PokerValue::FullHouse
        );
        assert_eq!(
            PokerValue::from_day_a(&[
                PokerCard::Two,
                PokerCard::Four,
                PokerCard::Four,
                PokerCard::Three,
                PokerCard::Four,
            ]),
            PokerValue::ThreeOfAKind
        );
        assert_eq!(
            PokerValue::from_day_a(&[
                PokerCard::Two,
                PokerCard::Four,
                PokerCard::Three,
                PokerCard::Three,
                PokerCard::Four,
            ]),
            PokerValue::TwoPairs
        );
        assert_eq!(
            PokerValue::from_day_a(&[
                PokerCard::Two,
                PokerCard::Five,
                PokerCard::Three,
                PokerCard::Three,
                PokerCard::Four,
            ]),
            PokerValue::OnePair
        );
        assert_eq!(
            PokerValue::from_day_a(&[
                PokerCard::Two,
                PokerCard::Five,
                PokerCard::Three,
                PokerCard::Six,
                PokerCard::Four,
            ]),
            PokerValue::AllDifferent
        );
    }
}
