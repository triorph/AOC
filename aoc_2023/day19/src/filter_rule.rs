use std::ops::Range;

use crate::part::{Part, PartRange};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilterRule {
    LessThan(PartCategory, usize, String),
    GreaterThan(PartCategory, usize, String),
    Target(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartCategory {
    X,
    M,
    A,
    S,
}

impl FilterRule {
    pub fn next_target(&self, part: &Part) -> Option<String> {
        match self {
            FilterRule::LessThan(part_category, value, target) => {
                if part.get(*part_category) < *value {
                    Some(target.to_owned())
                } else {
                    None
                }
            }
            FilterRule::GreaterThan(part_category, value, target) => {
                if part.get(*part_category) > *value {
                    Some(target.to_owned())
                } else {
                    None
                }
            }
            FilterRule::Target(target) => Some(target.to_owned()),
        }
    }

    pub fn next_target_range(&self, range: PartRange) -> Vec<PartRange> {
        let ret = match self {
            FilterRule::LessThan(part_category, value, target) => {
                range.split(*part_category, *value, target, &range.target)
            }
            FilterRule::GreaterThan(part_category, value, target) => {
                range.split(*part_category, *value, &range.target, target)
            }
            FilterRule::Target(target) => vec![range.clone_with_new_target(target)],
        };
        println!("Making ranges: {:?}", ret);
        ret
    }
}
