use day9::SmokeLocations;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("input_data.txt").expect("Need input_data.txt to exist");
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    let smoke_locations = SmokeLocations::new(&buffer[..]);
    let day_a = smoke_locations.calculate_day_a();
    println!("Day a result: {}", day_a);
    let smoke_locations = SmokeLocations::new(&buffer[..]);
    let day_b = smoke_locations.calculate_day_b();
    println!("Day b result: {}", day_b);
}
