extern crate peg;
use itertools::Itertools;

pub struct Day13Setup {}

peg::parser! { grammar day13_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    pub rule parse() -> Day13Setup
        = number:number() "\n" * {
            Day13Setup {}
        }
}}

impl Day13Setup {
    /// Generates a new Day13Setup object to be calculated upon
    ///
    /// Inputs: the full string contents of the input data set.
    ///
    /// Returns: A new Day13Setup object, with methods `calculate_day_a` and `calculate_day_b`
    /// available
    pub fn new(input_str: &str) -> Day13Setup {
        day13_parser::parse(input_str).unwrap()
    }

    /// Calculate the part a response
    pub fn calculate_day_a(self: &Day13Setup) -> usize {
        0
    }

    /// Calculate the part b response
    pub fn calculate_day_b(self: &Day13Setup) -> usize {
        0
    }
}

#[cfg(test)]
mod test {
    use crate::Day13Setup;

    #[test]
    fn test_parse() {
        let _octo_setup = Day13Setup::new(include_str!("../test_data.txt"));
    }

    #[test]
    fn test_day_a() {
        let day13_setup = Day13Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day13_setup.calculate_day_a(), 0);
    }

    #[test]
    fn test_day_b() {
        let day13_setup = Day13Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day13_setup.calculate_day_b(), 0);
    }
}
