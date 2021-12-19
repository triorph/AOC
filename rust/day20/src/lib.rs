extern crate peg;
use itertools::Itertools;

pub struct Day20Setup {}

peg::parser! { grammar day20_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    pub rule parse() -> Day20Setup
        = number:number() "\n" * {
            Day20Setup {}
        }
}}

impl Day20Setup {
    /// Generates a new Day20Setup object to be calculated upon
    ///
    /// Inputs: the full string contents of the input data set.
    ///
    /// Returns: A new Day20Setup object, with methods `calculate_day_a` and `calculate_day_b`
    /// available
    pub fn new(input_str: &str) -> Day20Setup {
        day20_parser::parse(input_str).unwrap()
    }

    /// Calculate the part a response
    pub fn calculate_day_a(self: &Day20Setup) -> usize {
        0
    }

    /// Calculate the part b response
    pub fn calculate_day_b(self: &Day20Setup) -> usize {
        0
    }
}

#[cfg(test)]
mod test {
    use crate::Day20Setup;

    #[test]
    fn test_parse() {
        let _day20_setup = Day20Setup::new(include_str!("../test_data.txt"));
    }

    #[test]
    fn test_day_a() {
        let day20_setup = Day20Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day20_setup.calculate_day_a(), 0);
    }

    #[test]
    fn test_day_b() {
        let day20_setup = Day20Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day20_setup.calculate_day_b(), 0);
    }
}
