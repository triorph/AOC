use day23::AmphipodMover;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("input_data.txt").expect("Need input_data.txt to exist");
    let mut input_str: String = String::new();
    f.read_to_string(&mut input_str).unwrap();
    let mut day23_setup = AmphipodMover::new(&input_str[..]);
    let day_a = day23_setup.calculate_day_a();
    println!("Day a result: {}", day_a);
    let mut day23_setup = AmphipodMover::new(&input_str[..]);
    let day_b = day23_setup.calculate_day_b();
    println!("Day b result: {}", day_b);
}
