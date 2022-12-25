mod elf;
mod parser;
use std::collections::HashSet;

use crate::elf::Elf;
use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};

pub struct Day23 {
    elves: HashSet<Elf>,
}

impl AOCCalculator for Day23 {
    fn new(filename: &str) -> Result<Day23, AOCFileOrParseError> {
        Ok(Day23 {
            elves: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day23 {
    fn calculate_day_a(&self) -> usize {
        let mut elves = self.elves.clone();
        for i in 0..10 {
            elves = self.next_elves(&elves, i);
        }
        self.get_empty_tiles(&elves)
    }

    fn next_elves(&self, elves: &HashSet<Elf>, count: usize) -> HashSet<Elf> {
        let elves_and_proposals: Vec<(Elf, Elf)> = elves
            .iter()
            .map(|elf| {
                let proposal = elf.propose_move(count, elves);
                (*elf, proposal)
            })
            .collect();
        let proposals: Vec<Elf> = elves_and_proposals
            .iter()
            .map(|(_, proposal)| *proposal)
            .collect();
        elves_and_proposals
            .into_iter()
            .map(|(elf, proposal)| {
                if proposals.iter().filter(|&&p| p == proposal).count() > 1 {
                    elf
                } else {
                    proposal
                }
            })
            .collect()
    }

    fn get_min_max(&self, elves: &HashSet<Elf>) -> (isize, isize, isize, isize) {
        elves.iter().fold((0, 0, 0, 0), |acc, elf| {
            (
                acc.0.min(elf.x),
                acc.1.min(elf.y),
                acc.2.max(elf.x),
                acc.3.max(elf.y),
            )
        })
    }

    fn get_empty_tiles(&self, elves: &HashSet<Elf>) -> usize {
        let (min_x, min_y, max_x, max_y) = self.get_min_max(elves);
        ((max_x - min_x + 1) * (max_y - min_y + 1)) as usize - elves.len()
    }

    fn calculate_day_b(&self) -> usize {
        let mut elves = self.elves.clone();
        let mut count = 0;
        loop {
            let next_elves = self.next_elves(&elves, count);
            count += 1;
            if next_elves == elves {
                return count;
            } else {
                elves = next_elves;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day23 = Day23::new("data/test_data.txt").unwrap();
        let expected = 110;
        let actual = day23.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_super_simple() {
        let day23 = Day23::new("data/simple_data.txt").unwrap();
        let expected = 25;
        let actual = day23.calculate_day_a();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_calculate_day_b() {
        let day23 = Day23::new("data/test_data.txt").unwrap();
        let expected = 20;
        let actual = day23.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day23 = Day23::new("data/input_data.txt").unwrap();
        let expected = 4025;
        let actual = day23.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day23 = Day23::new("data/input_data.txt").unwrap();
        let expected = 935;
        let actual = day23.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
