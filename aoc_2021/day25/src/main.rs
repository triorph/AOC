use day25::Day25Setup;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("input_data.txt").expect("Need input_data.txt to exist");
    let mut input_str: String = String::new();
    f.read_to_string(&mut input_str).unwrap();
    let day25_setup = Day25Setup::new(&input_str[..]);
    let day_a = day25_setup.calculate_day_a();
    println!("Day a result: {}", day_a);
}
