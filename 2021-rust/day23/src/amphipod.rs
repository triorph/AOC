#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amphipod {
    pub fn get_energy_per_step(&self) -> usize {
        match self {
            Amphipod::Amber => 1,
            Amphipod::Bronze => 10,
            Amphipod::Copper => 100,
            Amphipod::Desert => 1000,
        }
    }

    pub fn get_x_for_stack(&self) -> usize {
        match self {
            Amphipod::Amber => 2,
            Amphipod::Bronze => 4,
            Amphipod::Copper => 6,
            Amphipod::Desert => 8,
        }
    }

    pub fn from_x_location(x: usize) -> Amphipod {
        match x {
            2 => Amphipod::Amber,
            4 => Amphipod::Bronze,
            6 => Amphipod::Copper,
            8 => Amphipod::Desert,
            _ => unreachable!(),
        }
    }

    pub fn all() -> [Amphipod; 4] {
        [
            Amphipod::Amber,
            Amphipod::Bronze,
            Amphipod::Copper,
            Amphipod::Desert,
        ]
    }
}
