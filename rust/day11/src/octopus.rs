use crate::point::Point;
#[derive(Clone, Debug)]
pub struct Octopus {
    energy: usize,
    location: Point,
}

impl Octopus {
    pub fn new_at_location_with_energy(energy: usize, location: Point) -> Octopus {
        Octopus { energy, location }
    }
    pub fn bump_energy(&mut self) -> bool {
        self.energy += 1;
        self.energy == 10
    }

    pub fn reset_energy(&mut self) -> bool {
        if self.energy >= 10 {
            self.energy = 0;
            true
        } else {
            false
        }
    }
}
