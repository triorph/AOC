mod block;
mod parser;

use crate::block::Block;
use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};
use block::Overlaps;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day22 {
    blocks: Vec<Block>,
}

impl AOCCalculator for Day22 {
    fn new(filename: &str) -> Result<Day22, AOCFileOrParseError> {
        Ok(Day22 {
            blocks: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day22 {
    fn move_block_down(&self, block: &Block, blocks: &[Block]) -> Block {
        if block.is_at_bottom() {
            return block.clone();
        }
        let next = block.next();
        let overlaps = blocks
            .iter()
            .filter(|other| &block != other)
            .any(|other| next.overlaps(other));
        if overlaps {
            block.clone()
        } else {
            next
        }
    }

    fn move_all_blocks_down(&self, blocks: &[Block]) -> Vec<Block> {
        blocks
            .iter()
            .map(|block| self.move_block_down(block, blocks))
            .collect()
    }

    fn count_overlaps(&self, block: &Block, next: &Block, blocks: &[Block]) -> usize {
        blocks
            .iter()
            .filter(|other| other != &block)
            .filter(|other| next.overlaps(other))
            .count()
    }

    fn has_no_dependencies(&self, block: &Block, blocks: &[Block]) -> bool {
        blocks
            .iter()
            .filter(|other| other != &block && !other.is_at_bottom())
            .filter(|other| {
                other.next().overlaps(block)
                    && self.count_overlaps(other, &other.next(), blocks) == 1
            })
            .count()
            == 0
    }

    fn have_no_dependencies(&self, blocks: &[Block]) -> usize {
        blocks
            .iter()
            .filter(|block| self.has_no_dependencies(block, blocks))
            .count()
    }

    fn move_all_blocks_until_rest(&self, blocks: &[Block]) -> Vec<Block> {
        let mut blocks = blocks.to_owned();
        loop {
            let next_blocks = self.move_all_blocks_down(&blocks);
            if next_blocks.iter().map(|b| b.num_moves).sum::<usize>()
                == blocks.iter().map(|b| b.num_moves).sum()
            {
                return next_blocks;
            }
            blocks = next_blocks;
        }
    }

    fn calculate_day_a(&self) -> usize {
        let blocks = self.move_all_blocks_until_rest(&self.blocks);
        self.have_no_dependencies(&blocks)
    }

    fn calculate_day_b(&self) -> usize {
        let original = self.move_all_blocks_until_rest(&self.blocks);
        let mut move_count = 0;
        for block in original.iter() {
            let mut without = original.clone();
            without.retain(|b| b != block);
            let without_moved = self.move_all_blocks_until_rest(&without);
            let delta =
                without.len() - without.iter().filter(|w| without_moved.contains(w)).count();
            move_count += delta;
        }
        move_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day22 = Day22::new("data/test_data.txt").unwrap();
        let expected = 5;
        let actual = day22.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day22 = Day22::new("data/test_data.txt").unwrap();
        let expected = 7;
        let actual = day22.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day22 = Day22::new("data/input_data.txt").unwrap();
        let expected = 430;
        let actual = day22.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day22 = Day22::new("data/input_data.txt").unwrap();
        let expected = 0;
        let actual = day22.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
