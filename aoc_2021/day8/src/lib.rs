extern crate peg;
mod digit;
mod digit_line;
mod digit_possibilities;
mod digit_segment;
mod digit_setup;
mod digit_value;
mod parser;
pub use crate::digit_setup::DigitSetup;

#[cfg(test)]
mod test {
    use crate::digit::Digit;
    use crate::digit_possibilities::DigitPossibilities;
    use crate::digit_segment::DigitSegment;
    use crate::digit_setup::DigitSetup;
    use crate::digit_value::DigitValue;

    #[test]
    fn test_parse() {
        let digit_setup = DigitSetup::new(include_str!("../test_data.txt"));
        assert_eq!(digit_setup.digit_lines.len(), 11);
    }

    #[test]
    fn test_count_simple() {
        let digit_setup = DigitSetup::new(include_str!("../test_data.txt"));
        assert_eq!(digit_setup.digit_lines[0].count_simple_digits(), 0);
        assert_eq!(digit_setup.digit_lines[1].count_simple_digits(), 2);
        assert_eq!(digit_setup.digit_lines[2].count_simple_digits(), 3);
        assert_eq!(digit_setup.digit_lines[3].count_simple_digits(), 3);
        assert_eq!(digit_setup.digit_lines[4].count_simple_digits(), 1);
        assert_eq!(digit_setup.digit_lines[5].count_simple_digits(), 3);
        assert_eq!(digit_setup.digit_lines[6].count_simple_digits(), 4);
        assert_eq!(digit_setup.digit_lines[7].count_simple_digits(), 3);
        assert_eq!(digit_setup.digit_lines[8].count_simple_digits(), 1);
        assert_eq!(digit_setup.digit_lines[9].count_simple_digits(), 4);
        assert_eq!(digit_setup.digit_lines[10].count_simple_digits(), 2);
    }
    #[test]
    fn test_calculate_result() {
        let mut digit_setup = DigitSetup::new(include_str!("../test_data.txt"));
        assert_eq!(digit_setup.digit_lines[0].calculate_output(), 5353);
        assert_eq!(digit_setup.digit_lines[1].calculate_output(), 8394);
        assert_eq!(digit_setup.digit_lines[2].calculate_output(), 9781);
        assert_eq!(digit_setup.digit_lines[3].calculate_output(), 1197);
        assert_eq!(digit_setup.digit_lines[4].calculate_output(), 9361);
        assert_eq!(digit_setup.digit_lines[5].calculate_output(), 4873);
        assert_eq!(digit_setup.digit_lines[6].calculate_output(), 8418);
        assert_eq!(digit_setup.digit_lines[7].calculate_output(), 4548);
        assert_eq!(digit_setup.digit_lines[8].calculate_output(), 1625);
        assert_eq!(digit_setup.digit_lines[9].calculate_output(), 8717);
        assert_eq!(digit_setup.digit_lines[10].calculate_output(), 4315);
    }

    #[test]
    fn test_update_possibilities() {
        let mut digit = Digit {
            input_segments: vec![DigitSegment::A, DigitSegment::B],
            digit_value: DigitValue::Unknown,
        };
        let mut possibilities = DigitPossibilities::new();
        digit.update_possibilities(&mut possibilities);
        assert_eq!(
            possibilities.get(&DigitSegment::A),
            &vec![DigitSegment::C, DigitSegment::F]
        );
        assert_eq!(
            possibilities.get(&DigitSegment::B),
            &vec![DigitSegment::C, DigitSegment::F]
        );
        assert_eq!(
            possibilities.get(&DigitSegment::C),
            &vec![
                DigitSegment::A,
                DigitSegment::B,
                DigitSegment::D,
                DigitSegment::E,
                DigitSegment::G
            ]
        );
    }

    #[test]
    fn test_update_possibilities_no_change() {
        let mut digit = Digit {
            input_segments: DigitValue::Five.get_outputs_for_type(),
            digit_value: DigitValue::Unknown,
        };
        let mut possibilities = DigitPossibilities::new();
        digit.update_possibilities(&mut possibilities);
        for digit_segment in &DigitSegment::all_segments()[..] {
            assert_eq!(
                possibilities.get(digit_segment),
                &DigitSegment::all_segments().to_vec()
            );
        }
    }

    #[test]
    fn test_day_a() {
        let digit_setup = DigitSetup::new(include_str!("../test_data.txt"));
        assert_eq!(digit_setup.calculate_day_a(), 26);
    }

    #[test]
    fn test_day_b() {
        let mut digit_setup = DigitSetup::new(include_str!("../test_data.txt"));
        assert_eq!(digit_setup.calculate_day_b(), 61229 + 5353);
    }
}
