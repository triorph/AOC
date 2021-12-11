extern crate peg;
use itertools::Itertools;

#[derive(Clone, Debug)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Clone, Debug)]
struct Octopus {
    energy: usize,
    location: Point,
}
#[derive(Clone, Debug)]
pub struct OctopusFlashSetup {
    octopi: [Octopus; 100],
}

peg::parser! { grammar day11_parser() for str {
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

impl std::ops::Add for Point {
    type Output = Point;
    fn add(self: Point, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl OctopusFlashSetup {
    /// Generates a new OctopusFlashSetup object to be calculated upon
    ///
    /// Inputs: the full string contents of the input data set.
    ///
    /// Returns: A new OctopusFlashSetup object, with methods `calculate_day_a` and `calculate_day_b`
    /// available
    pub fn new(octopi_input_str: &str) -> OctopusFlashSetup {
        day11_parser::parse(octopi_input_str).unwrap()
    }

    fn get_points_iter() -> Box<dyn Iterator<Item = Point>> {
        Box::new((0..10).cartesian_product(0..10).map(|(y, x)| Point {
            x: x as isize,
            y: y as isize,
        }))
    }

    fn get_octopus_at_point(self: &mut OctopusFlashSetup, point: &Point) -> Option<&mut Octopus> {
        if point.x < 0 || point.x >= 10 || point.y < 0 || point.y >= 10 {
            None
        } else {
            let index = (point.y * 10 + point.x) as usize;
            Some(&mut self.octopi[index])
        }
    }

    fn update_neighbours(self: &mut OctopusFlashSetup, location: &Point) {
        for point in [
            Point { x: -1, y: -1 },
            Point { x: 0, y: -1 },
            Point { x: 1, y: -1 },
            Point { x: -1, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: -1, y: 1 },
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
        ]
        .into_iter()
        {
            self.increase_octopus_energy(&(point + location.clone()));
        }
    }

    fn bump_energy_for_octopus_at_location(self: &mut OctopusFlashSetup, location: &Point) -> bool {
        if let Some(octopus) = self.get_octopus_at_point(location) {
            octopus.energy += 1;
            if octopus.energy == 10 {
                return true;
            }
        }
        false
    }

    fn increase_octopus_energy(self: &mut OctopusFlashSetup, location: &Point) {
        let needs_to_flash = self.bump_energy_for_octopus_at_location(location);
        if needs_to_flash {
            self.update_neighbours(location);
        }
    }

    fn run_step(self: &OctopusFlashSetup) -> OctopusFlashSetup {
        let mut next = self.clone();
        for point in OctopusFlashSetup::get_points_iter() {
            next.increase_octopus_energy(&point);
        }
        next
    }

    fn reset_flashes(self: &mut OctopusFlashSetup) -> usize {
        let octopi_flashing = self
            .octopi
            .iter_mut()
            .filter(|octopus| octopus.energy >= 10)
            .map(|octopus| octopus.energy = 0);
        octopi_flashing.count()
    }

    fn count_flashes_after_n_iterations(self: &OctopusFlashSetup, n: usize) -> usize {
        let mut ret = 0;
        let mut current = self.clone();
        for _ in 0..n {
            current = current.run_step();
            ret += current.reset_flashes();
        }
        ret
    }

    fn find_step_where_all_flash(self: &OctopusFlashSetup) -> usize {
        let mut step = 0;
        let mut current = self.clone();
        loop {
            step += 1;
            current = current.run_step();
            if current.reset_flashes() == 100 {
                break;
            }
        }
        step
    }

    /// Find out how many flashes occur after 100 iterations
    pub fn calculate_day_a(self: &OctopusFlashSetup) -> usize {
        self.count_flashes_after_n_iterations(100)
    }

    /// Find out how many steps it takes for all octopi to flash at the same moment.
    pub fn calculate_day_b(self: &OctopusFlashSetup) -> usize {
        self.find_step_where_all_flash()
    }
}

#[cfg(test)]
mod test {
    use crate::OctopusFlashSetup;

    #[test]
    fn test_parse() {
        let _octo_setup = OctopusFlashSetup::new(include_str!("../test_data.txt"));
    }

    #[test]
    fn test_flash_count() {
        let octo_setup = OctopusFlashSetup::new(include_str!("../test_data.txt"));
        assert_eq!(octo_setup.count_flashes_after_n_iterations(1), 0);
        assert_eq!(octo_setup.count_flashes_after_n_iterations(10), 204);
    }

    #[test]
    fn test_day_a() {
        let octo_setup = OctopusFlashSetup::new(include_str!("../test_data.txt"));
        assert_eq!(octo_setup.calculate_day_a(), 1656);
    }

    #[test]
    fn test_day_b() {
        let octo_setup = OctopusFlashSetup::new(include_str!("../test_data.txt"));
        assert_eq!(octo_setup.calculate_day_b(), 195);
    }
}
