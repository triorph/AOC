use day11::OctopusFlashSetup;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("input_data.txt").expect("Need input_data.txt to exist");
    let mut input_str: String = String::new();
    f.read_to_string(&mut input_str).unwrap();
    let octopus_setup = OctopusFlashSetup::new(&input_str[..]);
    let day_a = octopus_setup.calculate_day_a();
    println!("Day a result: {}", day_a);
    let octopus_setup = OctopusFlashSetup::new(&input_str[..]);
    let day_b = octopus_setup.calculate_day_b();
    println!("Day b result: {}", day_b);
}
