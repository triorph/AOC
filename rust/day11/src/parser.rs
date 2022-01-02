extern crate peg;
use crate::octopus::Octopus;
use crate::octopusflash::OctopusFlashSetup;
use crate::point::Point;

peg::parser! { pub grammar day11_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule energy_values() -> Vec<usize>
        = energy_values:number() ** <100,100> ("\n" *) { energy_values }
    pub rule parse() -> OctopusFlashSetup
        = energy_values:energy_values() "\n" * {
            let octopi : [Octopus; 100] = energy_values.into_iter().enumerate().map(|( i, energy )| {
                let x = (i % 10) as isize;
                let y = (i / 10) as isize;
                Octopus{ energy, location: Point{x, y}}
            }).collect::<Vec<Octopus>>().try_into().unwrap();
            OctopusFlashSetup{octopi}
        }
}}
