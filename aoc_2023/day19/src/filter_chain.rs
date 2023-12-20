use std::ops::Range;

use crate::filter_rule::{FilterRule, PartCategory};
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
