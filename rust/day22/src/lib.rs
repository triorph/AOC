extern crate peg;

pub struct Day22Setup {
    volume_steps: Vec<VolumeStep>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Cuboid {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    min_z: isize,
    max_z: isize,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct VolumeStep {
    volume_type: bool,
    block: Cuboid,
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
            VolumeStep { volume_type, block: Cuboid{min_x, max_x, min_y, max_y, min_z, max_z }}
        }
    pub rule parse() -> Day22Setup
        = volume_steps:volume_step() ++ "\n" "\n" * {
            Day22Setup { volume_steps }
        }
}}

impl Cuboid {
    fn outside_day_a_limit(&self) -> bool {
        self.min_x < -50
            || self.min_x > 50
            || self.max_x < -50
            || self.max_x > 50
            || self.min_y < -50
            || self.min_y > 50
            || self.max_y < -50
            || self.max_y > 50
            || self.min_z < -50
            || self.min_z > 50
            || self.max_z < -50
            || self.max_z > 50
    }

    fn is_empty(&self) -> bool {
        self.min_x > self.max_x || self.min_y > self.max_y || self.min_z > self.max_z
    }

    fn overlaps_x(&self, val: isize) -> bool {
        (self.min_x..=self.max_x).contains(&val)
    }

    fn overlaps_y(&self, val: isize) -> bool {
        (self.min_y..=self.max_y).contains(&val)
    }

    fn overlaps_z(&self, val: isize) -> bool {
        (self.min_z..=self.max_z).contains(&val)
    }

    fn is_overlapping_x(&self, other: &Cuboid) -> bool {
        self.overlaps_x(other.min_x)
            || self.overlaps_x(other.max_x)
            || other.overlaps_x(self.min_x)
            || other.overlaps_x(self.max_x)
    }
    fn is_overlapping_y(&self, other: &Cuboid) -> bool {
        self.overlaps_y(other.min_y)
            || self.overlaps_y(other.max_y)
            || other.overlaps_y(self.min_y)
            || other.overlaps_y(self.max_y)
    }
    fn is_overlapping_z(&self, other: &Cuboid) -> bool {
        self.overlaps_z(other.min_z)
            || self.overlaps_z(other.max_z)
            || other.overlaps_z(self.min_z)
            || other.overlaps_z(self.max_z)
    }
    fn is_overlapping(&self, other: &Cuboid) -> bool {
        self.is_overlapping_x(other) && self.is_overlapping_y(other) && self.is_overlapping_z(other)
    }
    fn is_subset_of(&self, other: &Cuboid) -> bool {
        self.min_x >= other.min_x
            && self.max_x <= other.max_x
            && self.min_y >= other.min_y
            && self.max_y <= other.max_y
            && self.min_z >= other.min_z
            && self.max_z <= other.max_z
    }

    fn subtract_volume_for_x(&self, other: &Cuboid) -> Vec<Cuboid> {
        if self.is_overlapping(other) {
            let split_x = self.split_x(other);
            split_x
                .into_iter()
                .map(|x| x.subtract_volume_for_y(other))
                .flatten()
                .filter(|v| !v.is_empty())
                .filter(|v| !v.is_subset_of(other))
                .collect()
        } else {
            vec![self.clone()]
        }
    }

    fn get_x_vals(&self, other: &Cuboid) -> Vec<isize> {
        let mut ret = vec![self.min_x];
        if self.overlaps_x(other.min_x) {
            ret.push(other.min_x - 1);
            ret.push(other.min_x)
        }
        if self.overlaps_x(other.max_x) {
            ret.push(other.max_x);
            ret.push(other.max_x + 1);
        }
        ret.push(self.max_x);
        ret
    }

    fn get_y_vals(&self, other: &Cuboid) -> Vec<isize> {
        let mut ret = vec![self.min_y];
        if self.overlaps_y(other.min_y) {
            ret.push(other.min_y - 1);
            ret.push(other.min_y)
        }
        if self.overlaps_y(other.max_y) {
            ret.push(other.max_y);
            ret.push(other.max_y + 1);
        }
        ret.push(self.max_y);
        ret
    }

    fn get_z_vals(&self, other: &Cuboid) -> Vec<isize> {
        let mut ret = vec![self.min_z];
        if self.overlaps_z(other.min_z) {
            ret.push(other.min_z - 1);
            ret.push(other.min_z)
        }
        if self.overlaps_z(other.max_z) {
            ret.push(other.max_z);
            ret.push(other.max_z + 1);
        }
        ret.push(self.max_z);
        ret
    }

    fn split_x(&self, other: &Cuboid) -> Vec<Cuboid> {
        let mut ret = vec![];
        let x_vals = self.get_x_vals(other);
        for i in 0..(x_vals.len() / 2) {
            ret.push(Cuboid {
                min_x: x_vals[2 * i],
                max_x: x_vals[2 * i + 1],
                ..self.clone()
            })
        }
        ret
    }

    fn split_y(&self, other: &Cuboid) -> Vec<Cuboid> {
        let mut ret = vec![];
        let x_vals = self.get_y_vals(other);
        for i in 0..(x_vals.len() / 2) {
            ret.push(Cuboid {
                min_y: x_vals[2 * i],
                max_y: x_vals[2 * i + 1],
                ..self.clone()
            })
        }
        ret
    }

    fn split_z(&self, other: &Cuboid) -> Vec<Cuboid> {
        let mut ret = vec![];
        let x_vals = self.get_z_vals(other);
        for i in 0..(x_vals.len() / 2) {
            ret.push(Cuboid {
                min_z: x_vals[2 * i],
                max_z: x_vals[2 * i + 1],
                ..self.clone()
            })
        }
        ret
    }

    fn subtract_volume_for_y(&self, other: &Cuboid) -> Vec<Cuboid> {
        let split_y = self.split_y(other);
        split_y
            .into_iter()
            .map(|y| y.subtract_volume_for_z(other))
            .flatten()
            .collect()
    }

    fn subtract_volume_for_z(&self, other: &Cuboid) -> Vec<Cuboid> {
        self.split_z(other)
    }

    fn calculate_volume(&self) -> usize {
        ((self.max_x - self.min_x + 1)
            * (self.max_y - self.min_y + 1)
            * (self.max_z - self.min_z + 1)) as usize
    }

    fn repeatedly_subtract_from(&self, to_subtract_list: &[Cuboid]) -> Vec<Cuboid> {
        let mut ret = vec![self.clone()];
        for vol_to_subtract in to_subtract_list.iter() {
            ret = ret
                .into_iter()
                .filter(|volume| !volume.is_subset_of(vol_to_subtract))
                .map(|volume| volume.subtract_volume_for_x(vol_to_subtract))
                .flatten()
                .collect::<Vec<Cuboid>>();
        }
        ret
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

    fn subtract_from_other(to_be_subtracted: &[Cuboid], to_subtract: &[Cuboid]) -> Vec<Cuboid> {
        to_be_subtracted
            .iter()
            .map(|vol_to_be| vol_to_be.repeatedly_subtract_from(to_subtract))
            .flatten()
            .collect::<Vec<Cuboid>>()
    }

    fn get_next_volume_step(&mut self) -> VolumeStep {
        let ret = self.volume_steps[0].clone();
        self.volume_steps = self.volume_steps[1..].to_vec();
        ret
    }

    fn run_one_step(&mut self, resulting_volumes: &[Cuboid], check_day_a: bool) -> Vec<Cuboid> {
        let step = self.get_next_volume_step();
        if check_day_a && step.block.outside_day_a_limit() {
            return resulting_volumes.to_vec();
        }
        let mut resulting_volumes =
            Day22Setup::subtract_from_other(resulting_volumes, &[step.block.clone()]);
        if step.volume_type {
            resulting_volumes.push(step.block);
        }
        resulting_volumes
    }

    fn calculate_all_volumes_size(volumes: &[Cuboid]) -> usize {
        volumes.iter().map(|v| v.calculate_volume()).sum::<usize>()
    }

    fn run_steps(&mut self, check_day_a: bool) -> Vec<Cuboid> {
        let mut resulting_volumes = vec![];
        while !self.volume_steps.is_empty() {
            resulting_volumes = self.run_one_step(&resulting_volumes, check_day_a);
        }
        resulting_volumes
    }

    /// Calculate the part a response
    pub fn calculate_day_a(&mut self) -> usize {
        let volumes = self.run_steps(true);
        Day22Setup::calculate_all_volumes_size(&volumes)
    }

    /// Calculate the part b response
    pub fn calculate_day_b(&mut self) -> usize {
        let volumes = self.run_steps(false);
        Day22Setup::calculate_all_volumes_size(&volumes)
    }
}

#[cfg(test)]
mod test {
    use crate::Cuboid;
    use crate::Day22Setup;

    #[test]
    fn test_parse() {
        let _day22_setup = Day22Setup::new(include_str!("../test_data.txt"));
    }

    #[test]
    fn test_subtract_single() {
        let volume_a = Cuboid {
            min_x: 1,
            max_x: 1,
            min_y: 1,
            max_y: 3,
            min_z: 1,
            max_z: 3,
        };
        let volume_b = Cuboid {
            min_x: 1,
            max_x: 3,
            min_y: 1,
            max_y: 3,
            min_z: 1,
            max_z: 3,
        };
        let subtracted_volumes = Day22Setup::subtract_from_other(&[volume_b], &[volume_a]);
        let new_volume: usize = subtracted_volumes
            .iter()
            .map(|v| v.calculate_volume())
            .sum();
        assert_eq!(new_volume, 18);
    }

    #[test]
    fn test_subtract_multiple() {
        // we have 2 volumes with a gap of 1 between, and are trying to add a volume
        // covering all 3.
        //
        // We should subtract both existing volumes from the new volume before adding, and
        // end up with the new size.
        let volume_a = Cuboid {
            min_x: 1,
            max_x: 1,
            min_y: 1,
            max_y: 3,
            min_z: 1,
            max_z: 3,
        };
        let volume_b = Cuboid {
            min_x: 3,
            max_x: 3,
            min_y: 1,
            max_y: 3,
            min_z: 1,
            max_z: 3,
        };
        let volume_c = Cuboid {
            min_x: 1,
            max_x: 3,
            min_y: 1,
            max_y: 3,
            min_z: 1,
            max_z: 3,
        };
        let subtracted_volumes =
            Day22Setup::subtract_from_other(&[volume_c], &[volume_a, volume_b]);
        let new_volume: usize = subtracted_volumes
            .iter()
            .map(|v| v.calculate_volume())
            .sum();
        assert_eq!(new_volume, 9);
    }

    #[test]
    fn test_day_a() {
        let mut day22_setup = Day22Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day22_setup.calculate_day_a(), 39);
    }

    #[test]
    fn test_daya_larger() {
        let mut day22_setup = Day22Setup::new(include_str!("../test_data2.txt"));
        assert_eq!(day22_setup.calculate_day_a(), 590784);
    }

    #[test]
    fn test_daya_larger_step_by_step() {
        let mut day22_setup = Day22Setup::new(include_str!("../test_data2.txt"));
        let next = day22_setup.run_one_step(&[], true);
        assert_eq!(Day22Setup::calculate_all_volumes_size(&next), 139590);
        let next = day22_setup.run_one_step(&next, true);
        assert_eq!(Day22Setup::calculate_all_volumes_size(&next), 210918);
        let next = day22_setup.run_one_step(&next, true);
        assert_eq!(Day22Setup::calculate_all_volumes_size(&next), 225476);
        let next = day22_setup.run_one_step(&next, true);
        assert_eq!(Day22Setup::calculate_all_volumes_size(&next), 328328);
        let next = day22_setup.run_one_step(&next, true);
        assert_eq!(Day22Setup::calculate_all_volumes_size(&next), 387734);
        let next = day22_setup.run_one_step(&next, true);
        assert_eq!(Day22Setup::calculate_all_volumes_size(&next), 420416);
        let next = day22_setup.run_one_step(&next, true);
        assert_eq!(Day22Setup::calculate_all_volumes_size(&next), 436132);
        let next = day22_setup.run_one_step(&next, true);
        assert_eq!(Day22Setup::calculate_all_volumes_size(&next), 478727);
        let next = day22_setup.run_one_step(&next, true);
        assert_eq!(Day22Setup::calculate_all_volumes_size(&next), 494759);
        let next = day22_setup.run_one_step(&next, true);
        assert_eq!(Day22Setup::calculate_all_volumes_size(&next), 494804);
        let next = day22_setup.run_one_step(&next, true);
        assert_eq!(Day22Setup::calculate_all_volumes_size(&next), 492164);
        let next = day22_setup.run_one_step(&next, true);
        assert_eq!(Day22Setup::calculate_all_volumes_size(&next), 534936);
        let next = day22_setup.run_one_step(&next, true);
        assert_eq!(Day22Setup::calculate_all_volumes_size(&next), 534936);
        let next = day22_setup.run_one_step(&next, true);
        assert_eq!(Day22Setup::calculate_all_volumes_size(&next), 567192,);
        let next = day22_setup.run_one_step(&next, true);
        assert_eq!(Day22Setup::calculate_all_volumes_size(&next), 567150);
        let next = day22_setup.run_one_step(&next, true);
        assert_eq!(Day22Setup::calculate_all_volumes_size(&next), 592167);
        let next = day22_setup.run_one_step(&next, true);
        assert_eq!(Day22Setup::calculate_all_volumes_size(&next), 588567);
        let next = day22_setup.run_one_step(&next, true);
        assert_eq!(Day22Setup::calculate_all_volumes_size(&next), 592902);
        let next = day22_setup.run_one_step(&next, true);
        assert_eq!(Day22Setup::calculate_all_volumes_size(&next), 590029);
        let next = day22_setup.run_one_step(&next, true);
        assert_eq!(Day22Setup::calculate_all_volumes_size(&next), 590784);
    }

    #[test]
    fn test_day_a_with_b_input() {
        let mut day22_setup = Day22Setup::new(include_str!("../test_data3.txt"));
        assert_eq!(day22_setup.calculate_day_a(), 474140);
    }

    #[test]
    fn test_day_b() {
        let mut day22_setup = Day22Setup::new(include_str!("../test_data3.txt"));
        assert_eq!(day22_setup.calculate_day_b(), 2758514936282235);
    }
}
