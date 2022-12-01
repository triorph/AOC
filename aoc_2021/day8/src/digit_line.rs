use crate::digit::Digit;
use crate::digit_possibilities::DigitPossibilities;
use crate::digit_segment::DigitSegment;
use crate::digit_value::DigitValue;
pub struct DigitLine {
    pub input_digits: Vec<Digit>,
    pub output_digits: [Digit; 4],
}

impl DigitLine {
    pub fn count_simple_digits(self: &DigitLine) -> usize {
        self.output_digits
            .iter()
            .filter(|digit| digit.is_simple())
            .count()
    }

    fn output_not_found(self: &DigitLine) -> bool {
        self.output_digits
            .iter()
            .filter(|digit| digit.digit_value == DigitValue::Unknown)
            .count()
            > 0
    }

    fn build_output_number(self: &DigitLine) -> usize {
        self.output_digits
            .iter()
            .fold(0, |acc, digit| acc * 10 + digit.digit_value.to_usize())
    }

    fn update_possibilities_for_len_6_digits(
        self: &DigitLine,
        possibilities: &mut DigitPossibilities,
    ) {
        let len_6_digits = self
            .input_digits
            .clone()
            .into_iter()
            .filter(|digit| digit.input_segments.len() == 6)
            .collect::<Vec<Digit>>();
        let common_digits =
            len_6_digits
                .iter()
                .fold(DigitSegment::all_segments().to_vec(), |ret, next| {
                    ret.into_iter()
                        .filter(|val| next.input_segments.contains(val))
                        .collect()
                });

        possibilities.filter_down_possibilities(
            &common_digits,
            &[
                DigitSegment::A,
                DigitSegment::B,
                DigitSegment::F,
                DigitSegment::G,
            ],
        )
    }

    fn update_possibilities_for_len_5_digits(
        self: &DigitLine,
        possibilities: &mut DigitPossibilities,
    ) {
        let len_5_digits = self
            .input_digits
            .clone()
            .into_iter()
            .filter(|digit| digit.input_segments.len() == 5)
            .collect::<Vec<Digit>>();
        let common_digits =
            len_5_digits
                .iter()
                .fold(DigitSegment::all_segments().to_vec(), |ret, next| {
                    ret.into_iter()
                        .filter(|val| next.input_segments.contains(val))
                        .collect()
                });

        possibilities.filter_down_possibilities(
            &common_digits,
            &[DigitSegment::A, DigitSegment::D, DigitSegment::G],
        )
    }

    pub fn calculate_output(self: &mut DigitLine) -> usize {
        let mut possibilities = DigitPossibilities::new();
        self.update_possibilities_for_len_5_digits(&mut possibilities);
        self.update_possibilities_for_len_6_digits(&mut possibilities);

        while self.output_not_found() {
            for digit in &mut self.input_digits[..] {
                digit.update_possibilities(&mut possibilities)
            }
            for digit in &mut self.output_digits[..] {
                digit.update_possibilities(&mut possibilities)
            }
        }
        self.build_output_number()
    }
}
