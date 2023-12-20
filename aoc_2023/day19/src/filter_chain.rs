use crate::filter_rule::FilterRule;
use crate::part::{Part, PartRange};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilterChain {
    pub chain: Vec<FilterRule>,
}

impl FilterChain {
    pub fn next_target(&self, part: &Part) -> String {
        self.chain
            .iter()
            .fold(None, |prev, rule| prev.or_else(|| rule.next_target(part)))
            .expect("Expecting all filter chains to eventually complete")
    }

    pub fn next_target_range(&self, range: &PartRange) -> Vec<PartRange> {
        self.chain.iter().fold(vec![range.clone()], |ranges, rule| {
            ranges
                .into_iter()
                .flat_map(|r| {
                    if r.target == "A" || r.target == "R" || r.target != range.target {
                        vec![r]
                    } else {
                        rule.next_target_range(r)
                    }
                })
                .collect()
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::filter_rule::PartCategory;
    #[test]
    fn name() {
        assert_eq!(
            FilterChain {
                chain: vec![
                    FilterRule::LessThan(PartCategory::A, 2006, "qkq".to_string()),
                    FilterRule::GreaterThan(PartCategory::M, 2090, "A".to_string()),
                    FilterRule::Target("rfg".to_string())
                ]
            }
            .next_target_range(&PartRange {
                x: 1..4001,
                m: 1..4001,
                a: 1..4001,
                s: 1..1351,
                target: "px".to_string()
            }),
            vec![
                PartRange {
                    x: 1..4001,
                    m: 1..4001,
                    a: 1..2006,
                    s: 1..1351,
                    target: "qkq".to_string()
                },
                PartRange {
                    x: 1..4001,
                    m: 1..2091,
                    a: 2006..4001,
                    s: 1..1351,
                    target: "rfg".to_string()
                },
                PartRange {
                    x: 1..4001,
                    m: 2091..4001,
                    a: 2006..4001,
                    s: 1..1351,
                    target: "A".to_string()
                }
            ]
        );
    }
}
