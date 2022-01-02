extern crate peg;
use crate::octopus::Octopus;
use crate::octopusflash::{from_octopi, OctopusFlashSetup};
use crate::point::Point;

peg::parser! {  grammar day11_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule energy_values() -> Vec<usize>
        = energy_values:number() ** <100,100> ("\n" *) { energy_values }
    pub rule parse() -> OctopusFlashSetup
        = energy_values:energy_values() "\n" * {
            let octopi : [Octopus; 100] = energy_values.into_iter().enumerate().map(|( i, energy )| {
                let x = (i % 10) as isize;
                let y = (i / 10) as isize;
                Octopus::new_at_location_with_energy(energy, Point{x, y})
            }).collect::<Vec<Octopus>>().try_into().unwrap();
            from_octopi( octopi )
        }
}}

impl OctopusFlashSetup {
    /// Generates a new OctopusFlashSetup object to be calculated upon
    ///
    /// Inputs: the full string contents of the input data set.
    ///
    /// Returns: A new OctopusFlashSetup object, with methods `calculate_day_a` and `calculate_day_b`
    /// available
    pub fn new(octopi_input_str: &str) -> OctopusFlashSetup {
        day11_parser::parse(octopi_input_str).unwrap()
    }
}
