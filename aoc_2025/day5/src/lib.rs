mod parser;
use crate::parser::{parse_data, IngredientRange};
use aoc_helpers::{read_input_file, vec::Concatable, AOCCalculator, AOCFileOrParseError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day5 {
    available_ingredient_ids: Vec<usize>,
    fresh_ingredient_id_ranges: Vec<IngredientRange>,
}

impl AOCCalculator for Day5 {
    fn new(filename: &str) -> Result<Day5, AOCFileOrParseError> {
        let (fresh_ingredient_id_ranges, available_ingredient_ids) =
            parse_data(&read_input_file(filename)?)?;
        Ok(Day5 {
            fresh_ingredient_id_ranges,
            available_ingredient_ids,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

trait RangeOverlaps {
    fn overlaps(&self, other: &Self) -> bool;

    fn overlap(&self, other: &Self) -> Self;

    fn len(&self) -> usize;
}

impl RangeOverlaps for IngredientRange {
    fn overlaps(&self, other: &IngredientRange) -> bool {
        other.contains(self.start())
            || other.contains(self.end())
            || self.contains(other.start())
            || self.contains(other.end())
    }

    fn overlap(&self, other: &IngredientRange) -> IngredientRange {
        (*self.start().min(other.start()))..=(*self.end().max(other.end()))
    }

    fn len(&self) -> usize {
        self.end() - self.start() + 1
    }
}

impl Day5 {
    fn calculate_day_a(&self) -> usize {
        self.available_ingredient_ids
            .iter()
            .filter(|ingredient_id| {
                self.fresh_ingredient_id_ranges
                    .iter()
                    .any(|fresh_range| fresh_range.contains(ingredient_id))
            })
            .count()
    }

    fn combine_ranges(
        &self,
        fresh_ranges: &[IngredientRange],
        to_combine: &IngredientRange,
    ) -> Vec<IngredientRange> {
        let (unchanged, ranges_to_merge): (Vec<_>, Vec<_>) = fresh_ranges
            .iter()
            .cloned()
            .partition(|range| !range.overlaps(to_combine));
        let combined_range = ranges_to_merge
            .iter()
            .filter(|range| range.overlaps(to_combine))
            .fold(
                to_combine.clone(),
                |acc: IngredientRange, x: &IngredientRange| acc.overlap(x),
            );
        unchanged.concat_element(combined_range)
    }

    fn calculate_day_b(&self) -> usize {
        self.fresh_ingredient_id_ranges
            .iter()
            .fold(
                self.fresh_ingredient_id_ranges.clone(),
                |combined_ranges, to_combine| self.combine_ranges(&combined_ranges, to_combine),
            )
            .into_iter()
            .map(|ingredient_range| ingredient_range.len())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day5 = Day5::new("data/test_data.txt").unwrap();
        let expected = 3;
        let actual = day5.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day5 = Day5::new("data/test_data.txt").unwrap();
        let expected = 14;
        let actual = day5.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day5 = Day5::new("data/input_data.txt").unwrap();
        let expected = 848;
        let actual = day5.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day5 = Day5::new("data/input_data.txt").unwrap();
        let expected = 334714395325710;
        let actual = day5.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
