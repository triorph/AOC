use std::collections::HashSet;
use std::hash::Hash;

fn multi_hashset_intersection<T>(input: Vec<HashSet<T>>) -> HashSet<T>
where
    T: Eq + Hash + Copy,
{
    let mut start = input[0].clone();
    for other in input[1..].iter() {
        start = start.intersection(other).copied().collect()
    }
    start
}

pub trait HasPriority {
    fn get_priority(&self) -> u32;
}

impl HasPriority for char {
    fn get_priority(&self) -> u32 {
        if ('a'..='z').contains(self) {
            return u32::from(*self) - u32::from('a') + 1;
        }
        if ('A'..='Z').contains(self) {
            return u32::from(*self) - u32::from('A') + 27;
        }
        panic!("char {:?} wasn't found in a..z or A..Z", self);
    }
}

impl HasPriority for Vec<Vec<HashSet<char>>> {
    fn get_priority(&self) -> u32 {
        self.iter()
            .map(|group| {
                multi_hashset_intersection(group.clone())
                    .iter()
                    .map(|c| c.get_priority())
                    .sum::<u32>()
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::HasPriority;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_char_get_priority() {
        assert_eq!('p'.get_priority(), 16);
        assert_eq!('L'.get_priority(), 38);
        assert_eq!('P'.get_priority(), 42);
        assert_eq!('v'.get_priority(), 22);
        assert_eq!('t'.get_priority(), 20);
        assert_eq!('s'.get_priority(), 19);
    }
}
