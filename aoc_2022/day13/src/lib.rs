mod elf_packet;
mod parser;
use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};

use self::elf_packet::{compare, ElfPacket};
use std::cmp::Ordering;

pub struct Day13 {
    pairs: Vec<(ElfPacket, ElfPacket)>,
}

impl AOCCalculator for Day13 {
    fn new(filename: &str) -> Result<Day13, AOCFileOrParseError> {
        Ok(Day13 {
            pairs: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day13 {
    fn calculate_day_a(&self) -> usize {
        let mut ret = 0;
        for i in 0..self.pairs.len() {
            if compare(&self.pairs[i].0, &self.pairs[i].1) == Ordering::Less {
                ret += i + 1;
            }
        }
        ret
    }

    fn calculate_day_b(&self) -> usize {
        let mut new_pairs: Vec<ElfPacket> = self
            .pairs
            .iter()
            .flat_map(|(l, r)| vec![l, r])
            .cloned()
            .collect();
        let inserted_1 = ElfPacket::List(vec![ElfPacket::List(vec![ElfPacket::Literal(2)])]);
        new_pairs.push(inserted_1.clone());
        let inserted_2 = ElfPacket::List(vec![ElfPacket::List(vec![ElfPacket::Literal(6)])]);
        new_pairs.push(inserted_2.clone());
        new_pairs.sort();
        let mut indices: Vec<usize> = Vec::new();
        for (i, pair) in new_pairs.into_iter().enumerate() {
            if pair == inserted_1 || pair == inserted_2 {
                indices.push(i + 1);
            }
        }
        indices.iter().product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day13 = Day13::new("data/test_data.txt").unwrap();
        let expected = 13;
        let actual = day13.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day13 = Day13::new("data/test_data.txt").unwrap();
        let expected = 140;
        let actual = day13.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
