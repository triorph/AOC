use crate::filter_rule::PartCategory;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Part {
    pub x: usize,
    pub m: usize,
    pub a: usize,
    pub s: usize,
}

impl Part {
    pub fn get(&self, part_category: PartCategory) -> usize {
        match part_category {
            PartCategory::X => self.x,
            PartCategory::M => self.m,
            PartCategory::A => self.a,
            PartCategory::S => self.s,
        }
    }

    pub fn get_rating(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}
