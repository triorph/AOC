use crate::cuboid::Cuboid;
use crate::volumestep::VolumeStep;
use crate::Day22Setup;

peg::parser! { grammar day22_parser() for str {
    rule positive_number() -> isize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule negative_number() -> isize
        = "-" n:positive_number() { -n }
    rule number() -> isize
        = n:(negative_number() / positive_number()) { n }
    rule volume_on() -> bool
        = "on" { true }
    rule volume_off() -> bool
        = "off" { false }
    rule volume_type() -> bool
        = v:(volume_on() / volume_off()) { v }
    rule volume_step() -> VolumeStep
        = volume_type:volume_type() " x=" min_x:number() ".." max_x:number() ",y=" min_y:number() ".." max_y:number() ",z=" min_z:number() ".." max_z:number() {
            VolumeStep{ volume_type, block: Cuboid::new( min_x, max_x, min_y, max_y, min_z, max_z ) }
        }
    pub rule parse() -> Day22Setup
        = volume_steps:volume_step() ++ "\n" "\n" * {
            Day22Setup { volume_steps }
        }
}}

impl Day22Setup {
    /// Generates a new Day22Setup object to be calculated upon
    ///
    /// Inputs: the full string contents of the input data set.
    ///
    /// Returns: A new Day22Setup object, with methods `calculate_day_a` and `calculate_day_b`
    /// available
    pub fn new(input_str: &str) -> Day22Setup {
        day22_parser::parse(input_str).unwrap()
    }
}
