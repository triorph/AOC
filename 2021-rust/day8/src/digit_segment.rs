#[derive(Clone, Debug, Hash, Eq, PartialEq, Copy)]
pub enum DigitSegment {
    A, // top-horizontal
    B, // upper-left
    C, // upper-right
    D, // middle-horizontal
    E, // lower-left
    F, // lower-right
    G, // bottom-horizontal
}

impl DigitSegment {
    pub fn from(c: char) -> DigitSegment {
        match c {
            'a' => DigitSegment::A,
            'b' => DigitSegment::B,
            'c' => DigitSegment::C,
            'd' => DigitSegment::D,
            'e' => DigitSegment::E,
            'f' => DigitSegment::F,
            'g' => DigitSegment::G,
            _ => panic!("Not a valid digit segment input"),
        }
    }

    pub fn all_segments() -> [DigitSegment; 7] {
        [
            DigitSegment::A,
            DigitSegment::B,
            DigitSegment::C,
            DigitSegment::D,
            DigitSegment::E,
            DigitSegment::F,
            DigitSegment::G,
        ]
    }
}
