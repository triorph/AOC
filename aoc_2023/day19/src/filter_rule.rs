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
        match self {
            FilterRule::LessThan(part_category, value, target) => {
                range.split(*part_category, *value, target, &range.target)
            }
            FilterRule::GreaterThan(part_category, value, target) => {
                range.split(*part_category, *value + 1, &range.target, target)
            }
            FilterRule::Target(target) => vec![range.clone_with_new_target(target)],
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next_target_range() {
        assert_eq!(
            FilterRule::LessThan(PartCategory::X, 500, "abc".to_string()).next_target_range(
                PartRange::first().clone_with_part(PartCategory::X, 0..1000, "in")
            ),
            vec![
                PartRange::first().clone_with_part(PartCategory::X, 0..500, "abc"),
                PartRange::first().clone_with_part(PartCategory::X, 500..1000, "in")
            ]
        );
        assert_eq!(
            FilterRule::LessThan(PartCategory::X, 1500, "abc".to_string()).next_target_range(
                PartRange::first().clone_with_part(PartCategory::X, 0..1000, "in")
            ),
            vec![PartRange::first().clone_with_part(PartCategory::X, 0..1000, "abc"),]
        );
        assert_eq!(
            FilterRule::LessThan(PartCategory::X, 1500, "abc".to_string()).next_target_range(
                PartRange::first().clone_with_part(PartCategory::X, 2000..3000, "in")
            ),
            vec![PartRange::first().clone_with_part(PartCategory::X, 2000..3000, "in"),]
        );
        assert_eq!(
            FilterRule::GreaterThan(PartCategory::X, 500, "abc".to_string()).next_target_range(
                PartRange::first().clone_with_part(PartCategory::X, 0..1000, "in")
            ),
            vec![
                PartRange::first().clone_with_part(PartCategory::X, 0..501, "in"),
                PartRange::first().clone_with_part(PartCategory::X, 501..1000, "abc")
            ]
        );
        assert_eq!(
            FilterRule::GreaterThan(PartCategory::X, 1500, "abc".to_string()).next_target_range(
                PartRange::first().clone_with_part(PartCategory::X, 0..1000, "in")
            ),
            vec![PartRange::first().clone_with_part(PartCategory::X, 0..1000, "in"),]
        );
        assert_eq!(
            FilterRule::GreaterThan(PartCategory::X, 500, "abc".to_string()).next_target_range(
                PartRange::first().clone_with_part(PartCategory::X, 1000..2000, "in")
            ),
            vec![PartRange::first().clone_with_part(PartCategory::X, 1000..2000, "abc")]
        );
    }
}
