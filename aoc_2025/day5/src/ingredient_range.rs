use std::ops::RangeInclusive;
pub type IngredientRange = RangeInclusive<usize>;

pub trait RangeOverlaps
where
    Self: std::marker::Sized,
{
    fn overlaps(&self, other: &Self) -> bool;

    fn overlap(&self, other: &Self) -> Result<Self, String>;

    fn len(&self) -> usize;
}

impl RangeOverlaps for IngredientRange {
    fn overlaps(&self, other: &IngredientRange) -> bool {
        other.contains(self.start())
            || other.contains(self.end())
            || self.contains(other.start())
            || self.contains(other.end())
    }

    fn overlap(&self, other: &IngredientRange) -> Result<IngredientRange, String> {
        if self.overlaps(other) {
            Ok((*self.start().min(other.start()))..=(*self.end().max(other.end())))
        } else {
            Err("Does not overlap".to_string())
        }
    }

    fn len(&self) -> usize {
        self.end() - self.start() + 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(0..=3, 3..=5, true)]
    #[case(3..=5, 0..=3, true)]
    #[case(0..=3, 4..=5, false)]
    #[case(4..=5, 0..=3, false)]
    #[case(1..=5, 2..=3, true)]
    #[case(2..=3, 1..=5, true)]
    fn test_overlaps(
        #[case] range1: IngredientRange,
        #[case] range2: IngredientRange,
        #[case] expected: bool,
    ) {
        assert_eq!(range1.overlaps(&range2), expected);
    }

    #[rstest]
    #[case(0..=3, 3..=5, Ok(0..=5))]
    #[case(3..=5, 0..=3, Ok(0..=5))]
    #[case(0..=3, 4..=5, Err("Does not overlap".to_string()))]
    #[case(4..=5, 0..=3, Err("Does not overlap".to_string()))]
    #[case(1..=5, 2..=3, Ok(1..=5))]
    #[case(2..=3, 1..=5, Ok(1..=5))]
    fn test_overlap(
        #[case] range1: IngredientRange,
        #[case] range2: IngredientRange,
        #[case] expected: Result<IngredientRange, String>,
    ) {
        assert_eq!(range1.overlap(&range2), expected);
    }
}
