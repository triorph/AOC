extern crate peg;

struct DigitPossibilities([Vec<DigitSegment>; 7]);

#[derive(Clone, Debug, Hash, Eq, PartialEq, Copy)]
enum DigitSegment {
    A, // top-horizontal
    B, // upper-left
    C, // upper-right
    D, // middle-horizontal
    E, // lower-left
    F, // lower-right
    G, // bottom-horizontal
}

#[derive(Clone, Debug, Hash, PartialEq)]
enum DigitValue {
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

#[derive(Clone, Debug)]
struct Digit {
    input_segments: Vec<DigitSegment>,
    digit_value: DigitValue,
}

struct DigitLine {
    input_digits: Vec<Digit>,
    output_digits: [Digit; 4],
}

pub struct DigitSetup {
    digit_lines: Vec<DigitLine>,
}

peg::parser! { grammar day8_parser() for str {
    rule digit() -> Digit
        = digit_segments:$(['a'..='g']+) {Digit::new(digit_segments)}
    rule digit_line() -> DigitLine
        = input_digits:digit() ++ (" ") " | " output_digits:digit() **<4,4> (" ") {
            let converted_output_digits = [0;4];
            let converted_output_digits = output_digits.try_into().unwrap();
            DigitLine{input_digits, output_digits: converted_output_digits}
        }
    pub rule parse() -> DigitSetup
        = digit_lines:digit_line() ** ("\n" +) "\n" * {
            DigitSetup { digit_lines }
        }
}}

impl DigitPossibilities {
    fn new() -> DigitPossibilities {
        DigitPossibilities([
            DigitSegment::all_segments().to_vec(),
            DigitSegment::all_segments().to_vec(),
            DigitSegment::all_segments().to_vec(),
            DigitSegment::all_segments().to_vec(),
            DigitSegment::all_segments().to_vec(),
            DigitSegment::all_segments().to_vec(),
            DigitSegment::all_segments().to_vec(),
        ])
    }

    fn index_for_key(key: &DigitSegment) -> usize {
        match *key {
            DigitSegment::A => 0,
            DigitSegment::B => 1,
            DigitSegment::C => 2,
            DigitSegment::D => 3,
            DigitSegment::E => 4,
            DigitSegment::F => 5,
            DigitSegment::G => 6,
        }
    }

    fn get(self: &DigitPossibilities, key: &DigitSegment) -> &Vec<DigitSegment> {
        &self.0[DigitPossibilities::index_for_key(key)]
    }

    fn set(self: &mut DigitPossibilities, key: &DigitSegment, value: Vec<DigitSegment>) {
        self.0[DigitPossibilities::index_for_key(key)] = value;
    }

    fn filter_down_possibilities(
        self: &mut DigitPossibilities,
        inputs: &[DigitSegment],
        outputs: &[DigitSegment],
    ) {
        if inputs.len() == outputs.len() {
            for digit_segment in DigitSegment::all_segments().iter() {
                let old_outputs_for_input = self.get(digit_segment);
                let new_outputs_for_input = old_outputs_for_input
                    .iter()
                    .filter(|possible_value| {
                        if inputs.contains(digit_segment) {
                            outputs.contains(possible_value)
                        } else {
                            !outputs.contains(possible_value)
                        }
                    })
                    .copied()
                    .collect::<Vec<DigitSegment>>();
                self.set(digit_segment, new_outputs_for_input);
            }
        }
    }

    fn build_possible_outputs(
        self: &DigitPossibilities,
        inputs: &[DigitSegment],
    ) -> Vec<DigitSegment> {
        let mut possible_outputs = vec![];
        for digit_segment in &inputs[..] {
            for output_digit_segment in &self.get(digit_segment)[..] {
                if !possible_outputs.contains(output_digit_segment) {
                    possible_outputs.push(*output_digit_segment);
                }
            }
        }
        possible_outputs
    }
}
impl DigitSegment {
    fn from(c: char) -> DigitSegment {
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

    fn all_segments() -> [DigitSegment; 7] {
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

impl DigitValue {
    fn get_outputs_for_type(self: &DigitValue) -> Vec<DigitSegment> {
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

    fn to_usize(self: &DigitValue) -> usize {
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

impl Digit {
    fn new(digits: &str) -> Digit {
        let input_segments: Vec<DigitSegment> = digits.chars().map(DigitSegment::from).collect();
        Digit {
            input_segments,
            digit_value: DigitValue::Unknown,
        }
    }

    fn is_simple(self: &Digit) -> bool {
        [2_usize, 3_usize, 4_usize, 7_usize].contains(&self.input_segments.len())
    }

    fn determine_if_two_three_or_five(
        self: &Digit,
        possibilities: &DigitPossibilities,
    ) -> DigitValue {
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

    fn update_possibilities(self: &mut Digit, possibilities: &mut DigitPossibilities) {
        self.determine_digit_type(possibilities);
        let outputs = self.digit_value.get_outputs_for_type();
        possibilities.filter_down_possibilities(&self.input_segments, &outputs);
    }
}

impl DigitLine {
    fn count_simple_digits(self: &DigitLine) -> usize {
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

    fn calculate_output(self: &mut DigitLine) -> usize {
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

impl DigitSetup {
    /// Creates a new DigitSetup object from the string values of the input test data
    pub fn new(digit_setup_input_str: &str) -> DigitSetup {
        day8_parser::parse(digit_setup_input_str).unwrap()
    }

    /// Calculate the quantity of digits in the outputs that can be derived on their
    /// segment length alone, e.g. 1, 4, 7 and 8 each uniquely only have 2, 4, 3 and 7 segments
    /// each.
    pub fn calculate_day_a(self: &DigitSetup) -> usize {
        self.digit_lines
            .iter()
            .map(|digit_line| digit_line.count_simple_digits())
            .sum()
    }

    /// For each digit line, work out what the output number is supposed to be, then sum all digit
    /// line outputs
    pub fn calculate_day_b(self: &mut DigitSetup) -> usize {
        self.digit_lines
            .iter_mut()
            .map(|digit_line| digit_line.calculate_output())
            .sum()
    }
}

#[cfg(test)]
mod test {
    use crate::Digit;
    use crate::DigitPossibilities;
    use crate::DigitSegment;
    use crate::DigitSetup;
    use crate::DigitValue;

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
