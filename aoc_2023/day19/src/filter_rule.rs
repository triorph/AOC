use crate::part::Part;

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
}
