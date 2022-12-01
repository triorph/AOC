use day13::Day13Setup;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("input_data.txt").expect("Need input_data.txt to exist");
    let mut input_str: String = String::new();
    f.read_to_string(&mut input_str).unwrap();
    let day13_setup = Day13Setup::new(&input_str[..]);
    let day_a = day13_setup.calculate_day_a();
    println!("Day a result: {}", day_a);
    let day13_setup = Day13Setup::new(&input_str[..]);
    day13_setup.calculate_day_b();
}
