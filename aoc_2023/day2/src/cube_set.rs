use std::collections::HashMap;

pub type CubeSet = HashMap<String, usize>;

pub trait CubeSetTrait {
    fn is_subset(&self, other: &CubeSet) -> bool;
    fn get_power(&self) -> usize;
}

impl CubeSetTrait for CubeSet {
    fn is_subset(&self, other: &CubeSet) -> bool {
        for (key, value) in self.iter() {
            if let Some(other_value) = other.get(key) {
                if other_value < value {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    fn get_power(&self) -> usize {
        self.values().product()
    }
}
