extern crate peg;
use itertools::Itertools;

pub struct Day17Setup {}

peg::parser! { grammar day17_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    pub rule parse() -> Day17Setup
        = number:number() "\n" * {
            Day17Setup {}
        }
}}

impl Day17Setup {
    /// Generates a new Day17Setup object to be calculated upon
    ///
    /// Inputs: the full string contents of the input data set.
    ///
    /// Returns: A new Day17Setup object, with methods `calculate_day_a` and `calculate_day_b`
    /// available
    pub fn new(input_str: &str) -> Day17Setup {
        day17_parser::parse(input_str).unwrap()
    }

    /// Calculate the part a response
    pub fn calculate_day_a(self: &Day17Setup) -> usize {
        0
    }

    /// Calculate the part b response
    pub fn calculate_day_b(self: &Day17Setup) -> usize {
        0
    }
}

#[cfg(test)]
mod test {
    use crate::Day17Setup;

    #[test]
    fn test_parse() {
        let _octo_setup = Day17Setup::new(include_str!("../test_data.txt"));
    }

    #[test]
    fn test_day_a() {
        let day17_setup = Day17Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day17_setup.calculate_day_a(), 0);
    }

    #[test]
    fn test_day_b() {
        let day17_setup = Day17Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day17_setup.calculate_day_b(), 0);
    }
}
