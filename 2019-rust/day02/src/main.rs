use day02::{calculate_day_a, calculate_day_b};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("input_data.txt").expect("Need input_data.txt to exist");
    let mut input_str: String = String::new();
    f.read_to_string(&mut input_str).unwrap();
    let day_a = calculate_day_a(&input_str);
    println!("Day a result: {}", day_a);
    let day_b = calculate_day_b(&input_str);
    println!("Day a result: {}", day_b);
}
