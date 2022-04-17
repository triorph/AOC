use crate::digit_possibilities::DigitPossibilities;
use crate::digit_segment::DigitSegment;
use crate::digit_value::DigitValue;

#[derive(Clone, Debug)]
pub struct Digit {
    pub input_segments: Vec<DigitSegment>,
    pub digit_value: DigitValue,
}

impl Digit {
    pub fn new(digits: &str) -> Digit {
        let input_segments: Vec<DigitSegment> = digits.chars().map(DigitSegment::from).collect();
        Digit {
            input_segments,
            digit_value: DigitValue::Unknown,
        }
    }

    pub fn is_simple(self: &Digit) -> bool {
        [2_usize, 3_usize, 4_usize, 7_usize].contains(&self.input_segments.len())
    }

    fn determine_if_two_three_or_five(
        self: &Digit,
        possibilities: &DigitPossibilities,
    ) -> DigitValue {
        // Should only be called after determining that the length of the array is 5
        let possible_outputs = possibilities.build_possible_outputs(&self.input_segments);
        if !possible_outputs.contains(&DigitSegment::C) {
            DigitValue::Five
        } else if !possible_outputs.contains(&DigitSegment::B)
            && !possible_outputs.contains(&DigitSegment::E)
        {
            DigitValue::Three
        } else if !possible_outputs.contains(&DigitSegment::B)
            && !possible_outputs.contains(&DigitSegment::F)
        {
            DigitValue::Two
        } else {
            DigitValue::Unknown
        }
    }

    fn determine_if_zero_six_or_nine(
        self: &Digit,
        possibilities: &DigitPossibilities,
    ) -> DigitValue {
        // Should only be called after checking the input segments length is 6
        let possible_outputs = possibilities.build_possible_outputs(&self.input_segments);
        if !possible_outputs.contains(&DigitSegment::D) {
            DigitValue::Zero
        } else if !possible_outputs.contains(&DigitSegment::E) {
            DigitValue::Nine
        } else if !possible_outputs.contains(&DigitSegment::C) {
            DigitValue::Six
        } else {
            DigitValue::Unknown
        }
    }

    fn determine_digit_type(self: &mut Digit, possibilities: &DigitPossibilities) {
        if self.digit_value == DigitValue::Unknown {
            if self.input_segments.len() == 2 {
                self.digit_value = DigitValue::One;
                // Calculates which segment outputs are possible for a given input
                // segment choice (given what we have currently worked out)
            } else if self.input_segments.len() == 3 {
                self.digit_value = DigitValue::Seven;
            } else if self.input_segments.len() == 4 {
                self.digit_value = DigitValue::Four;
            } else if self.input_segments.len() == 7 {
                self.digit_value = DigitValue::Eight;
            } else if self.input_segments.len() == 5 {
                //possible options are 2, 3, 5
                self.digit_value = self.determine_if_two_three_or_five(possibilities);
            } else if self.input_segments.len() == 6 {
                // possible options are 0, 6, 9
                self.digit_value = self.determine_if_zero_six_or_nine(possibilities);
            }
        }
    }

    pub fn update_possibilities(self: &mut Digit, possibilities: &mut DigitPossibilities) {
        self.determine_digit_type(possibilities);
        let outputs = self.digit_value.get_outputs_for_type();
        possibilities.filter_down_possibilities(&self.input_segments, &outputs);
    }
}
