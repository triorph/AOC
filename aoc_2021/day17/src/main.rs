use day17::Day17Setup;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("input_data.txt").expect("Need input_data.txt to exist");
    let mut input_str: String = String::new();
    f.read_to_string(&mut input_str).unwrap();
    let day17_setup = Day17Setup::new(&input_str[..]);
    if let Some(day_a) = day17_setup.calculate_day_a() {
        println!("Day a result: {}", day_a);
    } else {
        println!("Unable to calculate day_a result");
    }
    let day17_setup = Day17Setup::new(&input_str[..]);
    let day_b = day17_setup.calculate_day_b();
    println!("Day b result: {}", day_b);
}
