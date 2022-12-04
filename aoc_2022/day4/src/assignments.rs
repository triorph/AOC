pub type Range = (usize, usize);
pub type Assignment = (Range, Range);

trait Contains {
    fn contains(&self, other: &Self) -> bool;
    fn overlaps(&self, other: &Self) -> bool;
}

pub trait Overlaps {
    fn complete_overlap(&self) -> bool;
    fn partial_overlap(&self) -> bool;
}

impl Contains for Range {
    fn contains(&self, &other: &Range) -> bool {
        self.0 >= other.0 && self.1 <= other.1
    }

    fn overlaps(&self, &other: &Range) -> bool {
        (self.0..=self.1).contains(&other.0) || (self.0..=self.1).contains(&other.1)
    }
}

impl Overlaps for Assignment {
    fn complete_overlap(&self) -> bool {
        self.0.contains(&self.1) || self.1.contains(&self.0)
    }

    fn partial_overlap(&self) -> bool {
        self.0.overlaps(&self.1) || self.1.overlaps(&self.0)
    }
}
