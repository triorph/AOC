#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Cuboid {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    min_z: isize,
    max_z: isize,
}

impl Cuboid {
    pub fn new(
        min_x: isize,
        max_x: isize,
        min_y: isize,
        max_y: isize,
        min_z: isize,
        max_z: isize,
    ) -> Cuboid {
        Cuboid {
            min_x,
            max_x,
            min_y,
            max_y,
            min_z,
            max_z,
        }
    }

    pub fn outside_day_a_limit(&self) -> bool {
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

    pub fn calculate_volume(&self) -> usize {
        ((self.max_x - self.min_x + 1)
            * (self.max_y - self.min_y + 1)
            * (self.max_z - self.min_z + 1)) as usize
    }

    pub fn repeatedly_subtract_from(&self, to_subtract_list: &[Cuboid]) -> Vec<Cuboid> {
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
