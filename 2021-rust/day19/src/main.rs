use day19::Day19Setup;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("input_data.txt").expect("Need input_data.txt to exist");
    let mut input_str: String = String::new();
    f.read_to_string(&mut input_str).unwrap();
    let mut day19_setup = Day19Setup::new(&input_str[..]);
    day19_setup.perform_scanner_mapping();
    let day_a = day19_setup.calculate_day_a();
    println!("Day a result: {}", day_a);
    let day_b = day19_setup.calculate_day_b();
    println!("Day b result: {}", day_b);
}
