extern crate peg;
use itertools::Itertools;

pub struct Day12Setup {}

peg::parser! { grammar day12_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    pub rule parse() -> Day12Setup
        = number:number() "\n" * {
            Day12Setup {}
        }
}}

impl Day12Setup {
    /// Generates a new Day12Setup object to be calculated upon
    ///
    /// Inputs: the full string contents of the input data set.
    ///
    /// Returns: A new Day12Setup object, with methods `calculate_day_a` and `calculate_day_b`
    /// available
    pub fn new(input_str: &str) -> Day12Setup {
        day12_parser::parse(input_str).unwrap()
    }

    /// Calculate the part a response
    pub fn calculate_day_a(self: &Day12Setup) -> usize {
        0
    }

    /// Calculate the part b response
    pub fn calculate_day_b(self: &Day12Setup) -> usize {
        0
    }
}

#[cfg(test)]
mod test {
    use crate::Day12Setup;

    #[test]
    fn test_parse() {
        let _day12_setup = Day12Setup::new(include_str!("../test_data.txt"));
    }

    #[test]
    fn test_day_a() {
        let day12_setup = Day12Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day12_setup.calculate_day_a(), 0);
    }

    #[test]
    fn test_day_b() {
        let day12_setup = Day12Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day12_setup.calculate_day_b(), 0);
    }
}
