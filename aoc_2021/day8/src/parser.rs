use crate::{digit::Digit, digit_line::DigitLine, digit_setup::DigitSetup};

peg::parser! { pub grammar day8_parser() for str {
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
