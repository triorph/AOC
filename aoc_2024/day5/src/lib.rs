mod parser;
use crate::parser::parse_data;
use aoc_helpers::{hash_utils::FromVec, read_input_file, AOCCalculator, AOCFileOrParseError};
use parser::{OrderRule, PageNumber};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day5 {
    rules: Vec<OrderRule>,
    page_numbers: Vec<PageNumber>,
}

impl AOCCalculator for Day5 {
    fn new(filename: &str) -> Result<Day5, AOCFileOrParseError> {
        let (rules, page_numbers) = parse_data(&read_input_file(filename)?)?;
        Ok(Day5 {
            rules,
            page_numbers,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day5 {
    fn build_invalidator_map(&self) -> HashMap<&usize, HashSet<usize>> {
        self.rules.iter().fold(
            HashMap::new(),
            |mut acc: HashMap<&usize, HashSet<usize>>, (left, right)| {
                acc.entry(left).or_default().insert(*right);
                acc
            },
        )
    }

    fn is_page_number_correct(
        &self,
        page_number: &[usize],
        invalidator: &HashMap<&usize, HashSet<usize>>,
    ) -> bool {
        page_number.iter().enumerate().all(|(index, value)| {
            invalidator
                .get(value)
                .unwrap_or(&HashSet::new())
                .intersection(&HashSet::from_vec(&page_number[0..index]))
                .count()
                == 0
        })
    }

    fn get_middle_value(&self, page_number: &[usize]) -> usize {
        page_number[page_number.len() / 2]
    }

    fn calculate_day_a(&self) -> usize {
        let invalidator = self.build_invalidator_map();
        self.page_numbers
            .iter()
            .filter(|page_number| self.is_page_number_correct(page_number, &invalidator))
            .map(|page_number| self.get_middle_value(page_number))
            .sum::<usize>()
    }

    fn insert_element(
        &self,
        mut acc: Vec<usize>,
        x: usize,
        invalidator: &HashMap<&usize, HashSet<usize>>,
    ) -> Vec<usize> {
        // Find the position of the first invalid element that we have to insert before
        // (or default to inserting at the end)
        let position = acc
            .iter()
            .position(|p| invalidator.get(&x).unwrap_or(&HashSet::new()).contains(p))
            .unwrap_or(acc.len());

        // is there a better way to make this functional?
        // (I guess since it moves acc into context, we don't have to worry that this is
        // technically "in place" editing)
        acc.insert(position, x);
        acc
    }

    fn reorder_page_number(
        &self,
        page_number: &[usize],
        invalidator: &HashMap<&usize, HashSet<usize>>,
    ) -> Vec<usize> {
        page_number
            .iter()
            // Insert each character 1 at a time, at the latest possible place that doesn't break
            // the rules invalidator.
            // note: this doesn't handle loops, but luckily the real data doesn't include any
            // looped failures
            .fold(vec![], |acc, x| self.insert_element(acc, *x, invalidator))
    }

    fn calculate_day_b(&self) -> usize {
        let invalidator = self.build_invalidator_map();
        self.page_numbers
            .iter()
            .filter(|page_number| !self.is_page_number_correct(page_number, &invalidator))
            .map(|page_number| self.reorder_page_number(page_number, &invalidator))
            .map(|page_number| self.get_middle_value(&page_number))
            .sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day5 = Day5::new("data/test_data.txt").unwrap();
        let expected = 143;
        let actual = day5.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day5 = Day5::new("data/test_data.txt").unwrap();
        let expected = 123;
        let actual = day5.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day5 = Day5::new("data/input_data.txt").unwrap();
        let expected = 4872;
        let actual = day5.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day5 = Day5::new("data/input_data.txt").unwrap();
        let expected = 5564;
        let actual = day5.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
