use crate::knot::Knot;
#[derive(Debug)]
pub struct Rope {
    data: Vec<Knot>,
}

impl Rope {
    pub fn new(capacity: usize) -> Rope {
        let mut data = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            data.push(Knot(0, 0));
        }
        Rope { data }
    }

    pub fn follow_instruction(&mut self, multiple_steps: &Knot) -> Vec<Knot> {
        let mut tail_locations = Vec::new();
        for one_step in multiple_steps.get_steps().iter() {
            tail_locations.push(self.follow_the_leader_one_step(one_step));
        }
        tail_locations
    }

    fn follow_the_leader_one_step(&mut self, one_step: &Knot) -> Knot {
        self.data[0] = self.data[0] + one_step;
        for i in 1..self.data.len() {
            self.data[i] = self.data[i].follow_other(&self.data[i - 1], 1)
        }
        self.get_tail()
    }

    fn get_tail(&self) -> Knot {
        self.data[self.data.len() - 1]
    }
}
