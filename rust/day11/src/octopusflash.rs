use crate::octopus::Octopus;
use crate::point::Point;
use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct OctopusFlashSetup {
    octopi: [Octopus; 100],
}

pub fn from_octopi(octopi: [Octopus; 100]) -> OctopusFlashSetup {
    OctopusFlashSetup { octopi }
}

impl OctopusFlashSetup {
    fn get_points_iter() -> Box<dyn Iterator<Item = Point>> {
        Box::new(
            (0..10)
                .cartesian_product(0..10)
                .map(|(y, x)| Point::new(x, y)),
        )
    }

    fn get_octopus_at_point(self: &mut OctopusFlashSetup, point: &Point) -> Option<&mut Octopus> {
        if point.out_of_bounds() {
            None
        } else {
            let index = point.get_as_index();
            Some(&mut self.octopi[index])
        }
    }

    fn update_neighbours(self: &mut OctopusFlashSetup, location: &Point) {
        for point in location.get_neighbours() {
            self.increase_octopus_energy(&point);
        }
    }

    fn bump_energy_for_octopus_at_location(self: &mut OctopusFlashSetup, location: &Point) -> bool {
        if let Some(octopus) = self.get_octopus_at_point(location) {
            octopus.bump_energy()
        } else {
            false
        }
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
            .map(|octopus| octopus.reset_energy())
            .filter(|octopus| *octopus);
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
