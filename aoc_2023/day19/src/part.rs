use std::ops::Range;

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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PartRange {
    pub x: Range<usize>,
    pub m: Range<usize>,
    pub a: Range<usize>,
    pub s: Range<usize>,
    pub target: String,
}

impl PartRange {
    pub fn first() -> PartRange {
        PartRange {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
            target: "in".to_string(),
        }
    }
    pub fn get(&self, part_category: PartCategory) -> Range<usize> {
        match part_category {
            PartCategory::X => self.x.clone(),
            PartCategory::M => self.m.clone(),
            PartCategory::A => self.a.clone(),
            PartCategory::S => self.s.clone(),
        }
    }

    pub fn get_size(&self) -> usize {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }

    fn clone_with_part(
        &self,
        part_category: PartCategory,
        range: Range<usize>,
        target: &str,
    ) -> PartRange {
        match part_category {
            PartCategory::X => PartRange {
                x: range,
                target: target.to_string(),
                ..self.clone()
            },
            PartCategory::M => PartRange {
                x: range,
                target: target.to_string(),
                ..self.clone()
            },
            PartCategory::A => PartRange {
                a: range,
                target: target.to_string(),
                ..self.clone()
            },
            PartCategory::S => PartRange {
                s: range,
                target: target.to_string(),
                ..self.clone()
            },
        }
    }

    pub fn clone_with_new_target(&self, target: &str) -> PartRange {
        PartRange {
            target: target.to_string(),
            ..self.clone()
        }
    }

    pub fn split(
        &self,
        part_category: PartCategory,
        value: usize,
        lower_target: &str,
        upper_target: &str,
    ) -> Vec<PartRange> {
        let range = self.get(part_category);
        if range.contains(&value) {
            vec![
                self.clone_with_part(part_category, range.start..value, lower_target),
                self.clone_with_part(part_category, value..range.end, upper_target),
            ]
        } else if value < range.start {
            vec![self.clone_with_part(part_category, range, lower_target)]
        } else {
            vec![self.clone_with_part(part_category, range, upper_target)]
        }
    }
}
