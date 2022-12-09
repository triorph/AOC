#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct Knot(pub isize, pub isize);

impl Knot {
    pub fn follow_other(self, other: &Knot, distance: usize) -> Knot {
        if self.calculate_distance(other) > distance {
            self.move_1_towards(other)
        } else {
            self
        }
    }

    pub fn get_steps(&self) -> Vec<Knot> {
        let quantity = self.0.abs() + self.1.abs();
        let direction = self.get_unit_val();
        (0..quantity).map(|_| direction).collect()
    }

    fn calculate_distance(&self, other: &Knot) -> usize {
        let x_diff = self.0.abs_diff(other.0);
        let y_diff = self.1.abs_diff(other.1);
        x_diff.max(y_diff)
    }

    fn move_1_towards(self, other: &Knot) -> Knot {
        let diff = (*other - &self).get_unit_val();
        self + &diff
    }
}

impl std::ops::Add<&Knot> for Knot {
    type Output = Knot;
    fn add(self, other: &Knot) -> Knot {
        Knot(self.0 + other.0, self.1 + other.1)
    }
}

impl std::ops::Sub<&Knot> for Knot {
    type Output = Knot;
    fn sub(self, other: &Knot) -> Knot {
        Knot(self.0 - other.0, self.1 - other.1)
    }
}

trait Unit {
    fn get_unit_val(&self) -> Self;
}

impl Unit for isize {
    fn get_unit_val(&self) -> isize {
        match self {
            x if x > &0 => 1,
            0 => 0,
            _ => -1,
        }
    }
}

impl Unit for Knot {
    fn get_unit_val(&self) -> Knot {
        Knot(self.0.get_unit_val(), self.1.get_unit_val())
    }
}
