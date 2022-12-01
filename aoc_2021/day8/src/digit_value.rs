use crate::digit_segment::DigitSegment;
#[derive(Clone, Debug, Hash, PartialEq)]
pub enum DigitValue {
    Unknown,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
}

impl DigitValue {
    pub fn get_outputs_for_type(self: &DigitValue) -> Vec<DigitSegment> {
        match self {
            DigitValue::Unknown => vec![
                DigitSegment::A,
                DigitSegment::B,
                DigitSegment::C,
                DigitSegment::D,
                DigitSegment::E,
                DigitSegment::F,
                DigitSegment::G,
            ],
            DigitValue::One => vec![DigitSegment::C, DigitSegment::F],
            DigitValue::Two => vec![
                DigitSegment::A,
                DigitSegment::C,
                DigitSegment::D,
                DigitSegment::E,
                DigitSegment::G,
            ],
            DigitValue::Three => vec![
                DigitSegment::A,
                DigitSegment::C,
                DigitSegment::D,
                DigitSegment::F,
                DigitSegment::G,
            ],
            DigitValue::Four => vec![
                DigitSegment::B,
                DigitSegment::C,
                DigitSegment::D,
                DigitSegment::F,
            ],
            DigitValue::Five => vec![
                DigitSegment::A,
                DigitSegment::B,
                DigitSegment::D,
                DigitSegment::F,
                DigitSegment::G,
            ],
            DigitValue::Six => vec![
                DigitSegment::A,
                DigitSegment::B,
                DigitSegment::D,
                DigitSegment::E,
                DigitSegment::F,
                DigitSegment::G,
            ],
            DigitValue::Seven => vec![DigitSegment::A, DigitSegment::C, DigitSegment::F],
            DigitValue::Eight => vec![
                DigitSegment::A,
                DigitSegment::B,
                DigitSegment::C,
                DigitSegment::D,
                DigitSegment::E,
                DigitSegment::F,
                DigitSegment::G,
            ],
            DigitValue::Nine => vec![
                DigitSegment::A,
                DigitSegment::B,
                DigitSegment::C,
                DigitSegment::D,
                DigitSegment::F,
                DigitSegment::G,
            ],
            DigitValue::Zero => vec![
                DigitSegment::A,
                DigitSegment::B,
                DigitSegment::C,
                DigitSegment::E,
                DigitSegment::F,
                DigitSegment::G,
            ],
        }
    }

    pub fn to_usize(self: &DigitValue) -> usize {
        match self {
            DigitValue::Zero => 0,
            DigitValue::One => 1,
            DigitValue::Two => 2,
            DigitValue::Three => 3,
            DigitValue::Four => 4,
            DigitValue::Five => 5,
            DigitValue::Six => 6,
            DigitValue::Seven => 7,
            DigitValue::Eight => 8,
            DigitValue::Nine => 9,
            DigitValue::Unknown => panic!("Cannot find value"),
        }
    }
}
