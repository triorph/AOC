use crate::filter_rule::FilterRule;
use crate::part::Part;

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
}
