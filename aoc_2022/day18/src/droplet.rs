#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct Droplet {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Droplet {
    pub fn calculate_exposed_surface(&self, other_droplets: &[Droplet]) -> usize {
        self.get_neighbours()
            .into_iter()
            .filter(|neighbour| !other_droplets.contains(neighbour))
            .count()
    }

    pub fn get_neighbours(&self) -> Vec<Droplet> {
        vec![
            Droplet {
                x: self.x + 1,
                y: self.y,
                z: self.z,
            },
            Droplet {
                x: self.x - 1,
                y: self.y,
                z: self.z,
            },
            Droplet {
                x: self.x,
                y: self.y + 1,
                z: self.z,
            },
            Droplet {
                x: self.x,
                y: self.y - 1,
                z: self.z,
            },
            Droplet {
                x: self.x,
                y: self.y,
                z: self.z + 1,
            },
            Droplet {
                x: self.x,
                y: self.y,
                z: self.z - 1,
            },
        ]
    }
}
