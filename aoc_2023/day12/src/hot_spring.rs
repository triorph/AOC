use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ConditionReport {
    hot_spring: Vec<HotSpringCondition>,
    criteria: Vec<usize>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct CriterionState {
    current_damaged_len: usize,
    current_index: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum HotSpringCondition {
    Damaged,
    Undamaged,
    Unknown,
}

impl CriterionState {
    fn first() -> CriterionState {
        CriterionState {
            current_damaged_len: 0,
            current_index: 0,
        }
    }
}

impl ConditionReport {
    pub fn new(hot_spring: Vec<HotSpringCondition>, criteria: Vec<usize>) -> ConditionReport {
        ConditionReport {
            hot_spring,
            criteria,
        }
    }

    fn get_permutations(&self) -> HashMap<CriterionState, usize> {
        let mut ret = HashMap::new();
        ret.insert(CriterionState::first(), 1);
        for value in self.hot_spring.iter() {
            ret = match value {
                HotSpringCondition::Damaged => self.get_next_damaged(&ret),
                HotSpringCondition::Undamaged => self.get_next_undamaged(&ret),
                HotSpringCondition::Unknown => self.get_next_unknown(&ret),
            };
        }
        ret
    }

    fn get_next_undamaged(
        &self,
        previous: &HashMap<CriterionState, usize>,
    ) -> HashMap<CriterionState, usize> {
        let mut ret = HashMap::new();
        for (key, value) in previous.iter() {
            if key.current_damaged_len == 0 {
                *ret.entry(*key).or_insert(0) += value;
            } else if key.current_damaged_len == self.criteria[key.current_index] {
                *ret.entry(CriterionState {
                    current_damaged_len: 0,
                    current_index: key.current_index + 1,
                })
                .or_insert(0) += value;
            }
        }
        ret
    }

    fn get_next_damaged(
        &self,
        previous: &HashMap<CriterionState, usize>,
    ) -> HashMap<CriterionState, usize> {
        let mut ret = HashMap::new();
        for (key, value) in previous.iter() {
            if key.current_index < self.criteria.len()
                && key.current_damaged_len < self.criteria[key.current_index]
            {
                *ret.entry(CriterionState {
                    current_damaged_len: key.current_damaged_len + 1,
                    current_index: key.current_index,
                })
                .or_insert(0) += value
            }
        }
        ret
    }

    fn get_next_unknown(
        &self,
        previous: &HashMap<CriterionState, usize>,
    ) -> HashMap<CriterionState, usize> {
        let mut ret = self.get_next_damaged(previous);
        for (key, value) in self.get_next_undamaged(previous) {
            *ret.entry(key).or_insert(0) += value;
        }
        ret
    }

    pub fn solve_day_a(&self) -> usize {
        let permutations = self.get_permutations();
        let final_permute = self.get_next_undamaged(&permutations);
        println!("Final permutation is: {:?}", final_permute);
        *final_permute
            .get(&CriterionState {
                current_damaged_len: 0,
                current_index: self.criteria.len(),
            })
            .unwrap_or(&0)
    }

    pub fn solve_day_b(&self) -> usize {
        let mut criteria_multiplied = self.criteria.to_owned();
        let mut hot_spring_multiplied = self.hot_spring.to_owned();
        for _ in 0..4 {
            criteria_multiplied.extend(&self.criteria);
            hot_spring_multiplied.push(HotSpringCondition::Unknown);
            hot_spring_multiplied.extend(&self.hot_spring);
        }
        ConditionReport::new(hot_spring_multiplied, criteria_multiplied).solve_day_a()
    }
}
