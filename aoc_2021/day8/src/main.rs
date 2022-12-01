use day8::DigitSetup;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut buffer = String::new();
    let mut f =
        File::open("input_data.txt").expect("We required input_data.txt to exist for this to run");
    f.read_to_string(&mut buffer).unwrap();
    let digit_setup = DigitSetup::new(&buffer[..]);
    let day_a = digit_setup.calculate_day_a();
    println!("Day a result: {}", day_a);
    let mut digit_setup = DigitSetup::new(&buffer[..]);
    let day_b = digit_setup.calculate_day_b();
    println!("Day b result: {}", day_b);
}
