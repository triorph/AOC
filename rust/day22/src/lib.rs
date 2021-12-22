extern crate peg;
use itertools::Itertools;
use std::cmp::{max, min};

pub struct Day22Setup {
    volume_steps: Vec<VolumeStep>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct VolumeStep {
    volume_type: bool,
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    min_z: isize,
    max_z: isize,
}

peg::parser! { grammar day22_parser() for str {
    rule positive_number() -> isize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule negative_number() -> isize
        = "-" n:positive_number() { -n }
    rule number() -> isize
        = n:(negative_number() / positive_number()) { n }
    rule volume_on() -> bool
        = "on" { true }
    rule volume_off() -> bool
        = "off" { false }
    rule volume_type() -> bool
        = v:(volume_on() / volume_off()) { v }
    rule volume_step() -> VolumeStep
        = volume_type:volume_type() " x=" min_x:number() ".." max_x:number() ",y=" min_y:number() ".." max_y:number() ",z=" min_z:number() ".." max_z:number() {
            VolumeStep { volume_type, min_x, max_x, min_y, max_y, min_z, max_z }
        }
    pub rule parse() -> Day22Setup
        = volume_steps:volume_step() ++ "\n" "\n" * {
            Day22Setup { volume_steps }
        }
}}

impl VolumeStep {
    fn constrain_day_a(&self) -> VolumeStep {
        VolumeStep {
            volume_type: self.volume_type,
            min_x: max(self.min_x, -50),
            max_x: min(self.max_x, 50),
            min_y: max(self.min_y, -50),
            max_y: min(self.max_y, 50),
            min_z: max(self.min_z, -50),
            max_z: min(self.max_z, 50),
        }
    }

    fn is_empty(&self) -> bool {
        self.min_x > self.max_x || self.min_y > self.max_y || self.min_z > self.max_z
    }

    fn is_overlapping(&self, other: &VolumeStep) -> bool {
        (other.min_x..other.max_x).contains(&self.min_x)
            || (other.min_x..other.max_x).contains(&self.max_x)
            || (other.min_y..other.max_y).contains(&self.min_y)
            || (other.min_y..other.max_y).contains(&self.max_y)
            || (other.min_z..other.max_z).contains(&self.min_z)
            || (other.min_z..other.max_z).contains(&self.max_z)
    }

    fn subtract_volume_for_x(&self, other: &VolumeStep) -> Vec<VolumeStep> {
        if self.is_overlapping(other) {
            let split_x = self.split_x(other);
            split_x
                .into_iter()
                .map(|x| x.subtract_volume_for_y(other))
                .flatten()
                .filter(|v| !v.is_empty())
                .filter(|v| !v.is_subset_of(other))
                .unique()
                .collect()
        } else {
            vec![self.clone()]
        }
    }

    fn is_subset_of(&self, other: &VolumeStep) -> bool {
        println!("Checking is subset: {:?} {:?}", self, other);
        let ret = self.min_x >= other.min_x
            && self.max_x <= other.max_x
            && self.min_y >= other.min_y
            && self.max_y <= other.max_y
            && self.min_z >= other.min_z
            && self.max_z <= other.min_z;
        println!("subset result: {}", ret);
        ret
    }

    fn split_x(&self, other: &VolumeStep) -> Vec<VolumeStep> {
        if other.min_x >= self.min_x && other.min_x <= self.max_x
            || other.max_x >= self.min_x && other.max_x <= self.max_x
        {
            vec![
                VolumeStep {
                    volume_type: true,
                    max_x: min(other.min_x - 1, self.max_x),
                    ..self.clone()
                }
                .constrain_day_a(),
                VolumeStep {
                    volume_type: true,
                    min_x: max(other.min_x, self.min_x),
                    max_x: min(other.max_x, self.max_x),
                    ..self.clone()
                }
                .constrain_day_a(),
                VolumeStep {
                    volume_type: true,
                    min_x: max(other.max_x + 1, self.min_x),
                    ..self.clone()
                }
                .constrain_day_a(),
            ]
        } else {
            vec![self.clone()]
        }
    }

    fn subtract_volume_for_y(&self, other: &VolumeStep) -> Vec<VolumeStep> {
        let split_y = self.split_y(other);
        split_y
            .into_iter()
            .map(|y| y.subtract_volume_for_z(other))
            .flatten()
            .filter(|v| !v.is_empty())
            .filter(|v| !v.is_subset_of(other))
            .unique()
            .collect()
    }

    fn split_y(&self, other: &VolumeStep) -> Vec<VolumeStep> {
        if other.min_y >= self.min_y && other.min_y <= self.max_y
            || other.max_y >= self.min_y && other.max_y <= self.max_y
        {
            vec![
                VolumeStep {
                    volume_type: true,
                    max_y: min(other.min_y - 1, self.max_y),
                    ..self.clone()
                }
                .constrain_day_a(),
                VolumeStep {
                    volume_type: true,
                    min_y: max(other.min_y, self.min_y),
                    max_y: min(other.max_y, self.max_y),
                    ..self.clone()
                }
                .constrain_day_a(),
                VolumeStep {
                    volume_type: true,
                    min_y: max(other.max_y + 1, self.min_y),
                    ..self.clone()
                }
                .constrain_day_a(),
            ]
        } else {
            vec![self.clone()]
        }
    }

    fn subtract_volume_for_z(&self, other: &VolumeStep) -> Vec<VolumeStep> {
        self.split_z(other)
    }

    fn split_z(&self, other: &VolumeStep) -> Vec<VolumeStep> {
        if other.min_z >= self.min_z && other.min_z <= self.max_z
            || other.max_z >= self.min_z && other.max_z <= self.max_z
        {
            vec![
                VolumeStep {
                    volume_type: true,
                    max_z: min(other.min_z - 1, self.max_z),
                    ..self.clone()
                }
                .constrain_day_a(),
                VolumeStep {
                    volume_type: true,
                    min_z: max(other.min_z, self.min_z),
                    max_z: min(other.max_z, self.max_z),
                    ..self.clone()
                }
                .constrain_day_a(),
                VolumeStep {
                    volume_type: true,
                    min_z: max(other.max_z + 1, self.min_z),
                    ..self.clone()
                }
                .constrain_day_a(),
            ]
        } else {
            vec![self.clone()]
        }
    }

    fn calculate_volume(&self) -> usize {
        ((self.max_x - self.min_x + 1)
            * (self.max_y - self.min_y + 1)
            * (self.max_z - self.min_z + 1)) as usize
    }
}

impl Day22Setup {
    /// Generates a new Day22Setup object to be calculated upon
    ///
    /// Inputs: the full string contents of the input data set.
    ///
    /// Returns: A new Day22Setup object, with methods `calculate_day_a` and `calculate_day_b`
    /// available
    pub fn new(input_str: &str) -> Day22Setup {
        day22_parser::parse(input_str).unwrap()
    }

    fn subtract_from_other(
        &self,
        to_be_subtracted: &[VolumeStep],
        to_subtract: &[VolumeStep],
    ) -> Vec<VolumeStep> {
        to_be_subtracted
            .iter()
            .map(|vol_to_be| {
                if to_subtract.is_empty() {
                    vec![vol_to_be.clone()]
                } else {
                    to_subtract
                        .iter()
                        .map(|vol_to_subtract| vol_to_be.subtract_volume_for_x(vol_to_subtract))
                        .flatten()
                        .unique()
                        .collect::<Vec<VolumeStep>>()
                }
            })
            .flatten()
            .unique()
            .collect::<Vec<VolumeStep>>()
    }

    fn run_steps(&self) -> Vec<VolumeStep> {
        let mut resulting_volumes = vec![];
        for step in self.volume_steps.iter() {
            println!("Running step:{:?}", step);
            let starting_volumes = vec![step.constrain_day_a()];
            if starting_volumes[0].is_empty() {
                continue;
            }
            if step.volume_type {
                let additional_volumes =
                    self.subtract_from_other(&starting_volumes, &resulting_volumes);
                for volume in additional_volumes.into_iter() {
                    println!("Adding volume {:?}", volume);
                    resulting_volumes.push(volume);
                }
            } else {
                resulting_volumes = self.subtract_from_other(&resulting_volumes, &starting_volumes);
            }
            println!(
                "Volume after step: {}-{}",
                resulting_volumes.len(),
                resulting_volumes
                    .iter()
                    .map(|v| v.calculate_volume())
                    .sum::<usize>()
            );
        }
        resulting_volumes
    }

    /// Calculate the part a response
    pub fn calculate_day_a(self: &Day22Setup) -> usize {
        let volumes = self.run_steps();
        volumes.into_iter().map(|v| v.calculate_volume()).sum()
    }

    /// Calculate the part b response
    pub fn calculate_day_b(self: &Day22Setup) -> usize {
        0
    }
}

#[cfg(test)]
mod test {
    use crate::Day22Setup;

    #[test]
    fn test_parse() {
        let _day22_setup = Day22Setup::new(include_str!("../test_data.txt"));
    }

    #[test]
    fn test_day_a() {
        let day22_setup = Day22Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day22_setup.calculate_day_a(), 39);
    }

    #[test]
    fn test_daya_larger() {
        let day22_setup = Day22Setup::new(include_str!("../test_data2.txt"));
        assert_eq!(day22_setup.calculate_day_a(), 590784);
    }

    #[test]
    fn test_day_b() {
        let day22_setup = Day22Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day22_setup.calculate_day_b(), 0);
    }
}
