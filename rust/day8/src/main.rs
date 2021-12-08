extern crate peg;
use std::collections::HashMap;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
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

#[derive(Clone)]
struct DigitLine {
    input_digits: Vec<Digit>,
    output_digits: [Digit; 4],
}

#[derive(Clone)]
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
        return [
            DigitSegment::A,
            DigitSegment::B,
            DigitSegment::C,
            DigitSegment::D,
            DigitSegment::E,
            DigitSegment::F,
            DigitSegment::G,
        ];
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

    fn filter_down_possibilities(
        self: &Digit,
        possibilities: &mut HashMap<DigitSegment, Vec<DigitSegment>>,
        outputs: Vec<DigitSegment>,
    ) {
        for digit_segment in DigitSegment::all_segments().iter() {
            let entry = possibilities
                .entry(digit_segment.clone())
                .or_insert(DigitSegment::all_segments().to_vec());
            *entry = entry
                .into_iter()
                .filter(|possible_value| {
                    if self.input_segments.contains(digit_segment) {
                        outputs.contains(possible_value)
                    } else {
                        !outputs.contains(possible_value)
                    }
                })
                .map(|digit_segment| digit_segment.clone())
                .collect::<Vec<DigitSegment>>();
        }
    }

    fn determine_if_two_three_or_five(
        self: &Digit,
        possibilities: &HashMap<DigitSegment, Vec<DigitSegment>>,
    ) -> DigitValue {
        let mut possible_values = vec![];
        for digit_segment in &self.input_segments[..] {
            for output_digit_segment in &possibilities[digit_segment][..] {
                if !possible_values.contains(output_digit_segment) {
                    possible_values.push(output_digit_segment.clone());
                }
            }
        }
        if !possible_values.contains(&DigitSegment::C) {
            // Only 5s don't have segment C
            DigitValue::Five
        } else if !possible_values.contains(&DigitSegment::B)
            && !possible_values.contains(&DigitSegment::E)
        {
            DigitValue::Three
        } else if !possible_values.contains(&DigitSegment::C)
            && !possible_values.contains(&DigitSegment::E)
        {
            DigitValue::Two
        } else {
            DigitValue::Unknown
        }
    }

    fn determine_if_zero_six_or_nine(
        self: &Digit,
        possibilities: &HashMap<DigitSegment, Vec<DigitSegment>>,
    ) -> DigitValue {
        DigitValue::Unknown
    }

    fn update_possibilities(
        self: &mut Digit,
        possibilities: &mut HashMap<DigitSegment, Vec<DigitSegment>>,
    ) {
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
        let outputs = self.digit_value.get_outputs_for_type();
        self.filter_down_possibilities(possibilities, outputs);
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
        self.output_digits.iter().fold(0_usize, |acc, digit| {
            acc * 10 + digit.digit_value.to_usize()
        })
    }

    fn calculate_output(self: &mut DigitLine) -> usize {
        let mut digit_possibilities: HashMap<DigitSegment, Vec<DigitSegment>> = HashMap::new();
        for digit in DigitSegment::all_segments().into_iter() {
            digit_possibilities.insert(digit, DigitSegment::all_segments().to_vec());
        }
        while self.output_not_found() {
            for mut digit in &mut self.input_digits[..] {
                digit.update_possibilities(&mut digit_possibilities)
            }
            for mut digit in &mut self.output_digits[..] {
                digit.update_possibilities(&mut digit_possibilities)
            }
        }
        self.build_output_number()
    }
}

impl DigitSetup {
    fn new(digit_setup_input_str: &str) -> DigitSetup {
        day8_parser::parse(digit_setup_input_str).unwrap()
    }

    fn calculate_day_a(self: &DigitSetup) -> usize {
        self.digit_lines
            .iter()
            .map(|digit_line| digit_line.count_simple_digits())
            .sum()
    }

    fn calculate_day_b(self: &DigitSetup) -> usize {
        0
    }
}

fn main() {
    let digit_setup = DigitSetup::new(include_str!("../input_data.txt"));
    let day_a = digit_setup.calculate_day_a();
    println!("Day a result: {}", day_a);
    let digit_setup = DigitSetup::new(include_str!("../input_data.txt"));
    let day_b = digit_setup.calculate_day_b();
    println!("Day b result: {}", day_b);
}

#[cfg(test)]
mod test {
    use crate::DigitSetup;

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
    fn test_day_a() {
        let digit_setup = DigitSetup::new(include_str!("../test_data.txt"));
        assert_eq!(digit_setup.calculate_day_a(), 26);
    }

    #[test]
    fn test_day_b() {
        let digit_setup = DigitSetup::new(include_str!("../test_data.txt"));
        assert_eq!(digit_setup.calculate_day_b(), 61229);
    }
}
