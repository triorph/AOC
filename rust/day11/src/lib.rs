extern crate peg;

pub struct CrabSetup {
    crab_locations: Vec<usize>,
}

peg::parser! { grammar day11_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    pub rule parse() -> CrabSetup
        = crab_locations:number() ** ("," +) "\n" * {
            CrabSetup { crab_locations}
        }
}}

impl CrabSetup {
    /// Generates a new CrabSetup object to be calculated upon
    ///
    /// Inputs: the full string contents of the input data set.
    ///
    /// Returns: A new CrabSetup object, with methods `calculate_day_a` and `calculate_day_b`
    /// available
    pub fn new(crab_setup_input_str: &str) -> CrabSetup {
        day11_parser::parse(crab_setup_input_str).unwrap()
    }

    /// Find the minimum distance for all crabs to move, assuming 1 fuel per movement
    ///
    /// Currently searches all possible locations and works out which is best, but using Median
    /// should work too. O(N^2) instead of O(N) but takes a few milliseconds with the input
    /// dataset.
    pub fn calculate_day_a(self: &CrabSetup) -> usize {
        0
    }

    /// Find the minimum distance for all crabs to move, assuming increasing fuel usage per
    /// additional movement. e.g. 1 fuel for first movement. +2 fuel (for 3 total) for next. +3
    /// fuel (for 6 total) after. The gauss formula n(n+1)/2 can be used here.
    ///
    /// Also searches all possible locations to work out which is best. Some smart people
    /// have figured that mean is mostly correct here, but you have to check +-0.5 to each side of
    /// it. This should be able to bring us down to O(N) over O(N^2) that we have here, but this
    /// still only takes a few milliseconds with the input dataset.
    pub fn calculate_day_b(self: &CrabSetup) -> usize {
        0
    }
}

#[cfg(test)]
mod test {
    use crate::CrabSetup;

    #[test]
    fn test_parse() {
        let crab_setup = CrabSetup::new(include_str!("../test_data.txt"));
        assert_eq!(crab_setup.crab_locations, [16, 1, 2, 0, 4, 2, 7, 1, 2, 14]);
    }

    #[test]
    fn test_day_a() {
        let crab_setup = CrabSetup::new(include_str!("../test_data.txt"));
        assert_eq!(crab_setup.calculate_day_a(), 0);
    }

    #[test]
    fn test_day_b() {
        let crab_setup = CrabSetup::new(include_str!("../test_data.txt"));
        assert_eq!(crab_setup.calculate_day_b(), 0);
    }
}
