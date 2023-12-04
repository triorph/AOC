use std::collections::HashSet;
pub type Card = (HashSet<usize>, HashSet<usize>);

pub trait CardTrait {
    fn matches(&self) -> usize;
    fn score(&self) -> usize;
}

impl CardTrait for Card {
    fn matches(&self) -> usize {
        return self.0.intersection(&self.1).count();
    }

    fn score(&self) -> usize {
        match self.matches() {
            0 => 0,
            x => 2_i32.pow(x as u32 - 1) as usize,
        }
    }
}
