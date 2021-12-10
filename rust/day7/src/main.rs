extern crate peg;
use std::cmp;

#[derive(Clone)]
pub struct CrabSetup {
    crab_locations: Vec<usize>,
}

peg::parser! { grammar day7_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    pub rule parse() -> CrabSetup
        = crab_locations:number() ** ("," +) "\n" * {
            CrabSetup { crab_locations}
        }

}
}

impl CrabSetup {
    fn new(crab_setup_input_str: &str) -> CrabSetup {
        day7_parser::parse(crab_setup_input_str).unwrap()
    }

    fn get_fuel_used_for_location_day_a(self: &CrabSetup, target_location: usize) -> usize {
        self.crab_locations
            .iter()
            .map(|crab_location| {
                (*crab_location as isize - target_location as isize).abs() as usize
            })
            .sum()
    }

    fn fuel_modifier(distance: usize) -> usize {
        // rough formula for n + n - 1 + n - 2 + ... + 1 = n * (n + 1) / 2
        distance * (distance + 1) / 2
    }

    fn get_fuel_used_for_location_day_b(self: &CrabSetup, target_location: usize) -> usize {
        self.crab_locations
            .iter()
            .map(|crab_location| {
                (*crab_location as isize - target_location as isize).abs() as usize
            })
            .map(CrabSetup::fuel_modifier)
            .sum()
    }

    fn get_values_to_check(self: &CrabSetup) -> std::ops::Range<usize> {
        let lower: usize = *self.crab_locations.iter().reduce(cmp::min).unwrap();
        let upper: usize = *self.crab_locations.iter().reduce(cmp::max).unwrap();
        lower..upper
    }

    fn calculate_day_a(self: &CrabSetup) -> usize {
        self.get_values_to_check()
            .map(|crab_location| self.get_fuel_used_for_location_day_a(crab_location))
            .reduce(cmp::min)
            .unwrap()
    }

    fn calculate_day_b(self: &CrabSetup) -> usize {
        self.get_values_to_check()
            .map(|crab_location| self.get_fuel_used_for_location_day_b(crab_location))
            .reduce(cmp::min)
            .unwrap()
    }
}

fn main() {
    let crab_setup = CrabSetup::new(include_str!("../input_data.txt"));
    let day_a = crab_setup.calculate_day_a();
    println!("Day a result: {}", day_a);
    let crab_setup = CrabSetup::new(include_str!("../input_data.txt"));
    let day_b = crab_setup.calculate_day_b();
    println!("Day b result: {}", day_b);
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
    fn test_fuel_used_per_distance_day_a() {
        let crab_setup = CrabSetup::new(include_str!("../test_data.txt"));
        assert_eq!(crab_setup.get_fuel_used_for_location_day_a(1), 41);
        assert_eq!(crab_setup.get_fuel_used_for_location_day_a(3), 39);
        assert_eq!(crab_setup.get_fuel_used_for_location_day_a(10), 71);
    }

    #[test]
    fn test_fuel_used_per_distance_day_b() {
        let crab_setup = CrabSetup::new(include_str!("../test_data.txt"));
        assert_eq!(crab_setup.get_fuel_used_for_location_day_b(5), 168);
        assert_eq!(crab_setup.get_fuel_used_for_location_day_b(2), 206);
    }

    #[test]
    fn test_fuel_modifier() {
        assert_eq!(CrabSetup::fuel_modifier(1), 1);
        assert_eq!(CrabSetup::fuel_modifier(2), 3);
        assert_eq!(CrabSetup::fuel_modifier(3), 6);
        assert_eq!(CrabSetup::fuel_modifier(4), 10);
        assert_eq!(CrabSetup::fuel_modifier(5), 15);
    }

    #[test]
    fn test_day_a() {
        let crab_setup = CrabSetup::new(include_str!("../test_data.txt"));
        assert_eq!(crab_setup.calculate_day_a(), 37);
    }

    #[test]
    fn test_day_b() {
        let crab_setup = CrabSetup::new(include_str!("../test_data.txt"));
        assert_eq!(crab_setup.calculate_day_b(), 168);
    }
}
