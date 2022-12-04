use std::collections::HashSet;
fn multi_hashset_intersection<T>(input: Vec<HashSet<T>>) -> HashSet<T>
where
    T: Eq,
    T: std::hash::Hash,
    T: Copy,
{
    let mut start = input[0].clone();
    for other in input[1..].iter() {
        start = start.intersection(other).copied().collect()
    }
    start
}

pub trait FromVec<T>
where
    T: Eq,
    T: std::hash::Hash,
    T: Copy,
{
    fn from_vec(v: &[T]) -> Self;
}

impl FromVec<char> for HashSet<char> {
    fn from_vec(v: &[char]) -> HashSet<char> {
        HashSet::from_iter(v.iter().copied())
    }
}

pub trait HasPriority {
    fn get_priority(&self) -> usize;
}

impl HasPriority for char {
    fn get_priority(&self) -> usize {
        let mut count = 0;
        for maybe in 'a'..='z' {
            count += 1;
            if self == &maybe {
                return count;
            }
        }
        for maybe in 'A'..='Z' {
            count += 1;
            if self == &maybe {
                return count;
            }
        }
        panic!("char {:?} wasn't found in a..z or A..Z", self);
    }
}

impl HasPriority for Vec<Vec<HashSet<char>>> {
    fn get_priority(&self) -> usize {
        self.iter()
            .map(|group| {
                multi_hashset_intersection(group.clone())
                    .iter()
                    .map(|c| c.get_priority())
                    .sum::<usize>()
            })
            .sum()
    }
}
