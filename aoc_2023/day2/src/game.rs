use crate::cube_set::{CubeSet, CubeSetTrait};
use std::cmp::max;

pub struct Game {
    pub id: usize,
    pub cube_sets: Vec<CubeSet>,
}

impl Game {
    pub fn new(id: usize, cube_sets: Vec<CubeSet>) -> Game {
        Game { id, cube_sets }
    }

    pub fn find_minimum_coverage(&self) -> CubeSet {
        let mut ret = CubeSet::new();
        for cube_set in self.cube_sets.iter() {
            for (key, value) in cube_set.iter() {
                ret.entry(key.clone())
                    .and_modify(|past_value| *past_value = max(*past_value, *value))
                    .or_insert(*value);
            }
        }
        ret
    }

    pub fn could_be_from_cubeset(&self, other_cube_set: &CubeSet) -> bool {
        !self
            .cube_sets
            .iter()
            .any(|my_cube_set| !my_cube_set.is_subset(&other_cube_set))
    }
}
