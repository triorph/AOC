mod parser;
use std::collections::VecDeque;

use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day9 {
    data: Vec<usize>,
}

impl AOCCalculator for Day9 {
    fn new(filename: &str) -> Result<Day9, AOCFileOrParseError> {
        Ok(Day9 {
            data: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day9 {
    fn calculate_checksum(&self, defragged_sector: &[(Option<usize>, usize, usize)]) -> usize {
        defragged_sector.iter().fold(0, |checksum, sector_entry| {
            let (sector_id, sector_start, sector_end) = sector_entry;
            if let Some(sector_id) = sector_id {
                checksum
                    + (*sector_start..*sector_end)
                        .map(|i| (i * sector_id))
                        .sum::<usize>()
            } else {
                checksum
            }
        })
    }

    fn build_sector_map(&self) -> VecDeque<(Option<usize>, usize, usize)> {
        self.data
            .chunks(2)
            .enumerate()
            .fold(
                (0, VecDeque::new()),
                |(counter, mut sector_ranges), (i, x)| {
                    sector_ranges.push_back((Some(i), counter, counter + x[0]));
                    if x.len() == 2 {
                        sector_ranges.push_back((None, counter + x[0], counter + x[0] + x[1]));
                        (counter + x[0] + x[1], sector_ranges)
                    } else {
                        (counter + x[0], sector_ranges)
                    }
                },
            )
            .1
    }

    fn calculate_day_a(&self) -> usize {
        let mut ret = vec![];
        let mut sectors = self.build_sector_map();
        let mut current_sector = sectors.pop_front();
        let mut sector_to_move = sectors.pop_back();
        while current_sector.is_some() && sector_to_move.is_some() {
            let (id, start, end) = current_sector.unwrap();
            let (other_id, other_start, other_end) = sector_to_move.unwrap();
            if other_id.is_none() {
                sector_to_move = sectors.pop_back();
            } else if id.is_some() {
                ret.push(current_sector.unwrap());
                current_sector = sectors.pop_front();
            } else {
                let size_moved = usize::min(end - start, other_end - other_start);
                ret.push((other_id, start, start + size_moved));
                let start = start + size_moved;
                let other_end = other_end - size_moved;
                if start == end {
                    current_sector = sectors.pop_front();
                } else {
                    current_sector = Some((id, start, end));
                }
                if other_start == other_end {
                    sector_to_move = sectors.pop_back();
                } else {
                    sector_to_move = Some((other_id, other_start, other_end))
                }
            }
        }
        if let Some((id, start, end)) = sector_to_move {
            if id.is_some() {
                ret.push((id, start, end))
            }
        }

        self.calculate_checksum(&ret)
    }

    fn calculate_day_b(&self) -> usize {
        let mut sector_map = self.build_sector_map();
        let max_value = sector_map
            .iter()
            .map(|(id, _, _)| id.unwrap_or(0))
            .max()
            .unwrap();
        for id in (0..=max_value).rev() {
            let sector_entry = sector_map
                .iter()
                .enumerate()
                .find(|(_, val)| val.0 == Some(id))
                .unwrap_or_else(|| {
                    panic!(
                        "for some reason entry {:?} doesn't exist, {:?}",
                        id, sector_map
                    );
                })
                .0;
            sector_map[sector_entry].0 = None;
            let size = sector_map[sector_entry].2 - sector_map[sector_entry].1;
            let earliest_placement = sector_map
                .iter()
                .enumerate()
                .find(
                    |(_, (free_sector_id, free_sector_start, free_sector_end))| {
                        free_sector_id.is_none() && free_sector_end - free_sector_start >= size
                    },
                )
                .expect("Will at least go back to where it was")
                .0;
            let (_, free_sector_start, free_sector_end) = sector_map[earliest_placement];
            let free_size = free_sector_end - free_sector_start;
            sector_map[earliest_placement].0 = Some(id);
            sector_map[earliest_placement].2 = free_sector_start + size;
            if free_size > size {
                // create a new free sector from the remaining stuff
                // (do we need to join to the next, surely not?)
                sector_map.insert(
                    earliest_placement + 1,
                    (
                        None,
                        free_sector_start + size,
                        free_sector_start + free_size,
                    ),
                );
            }
        }
        self.calculate_checksum(
            &sector_map
                .into_iter()
                .collect::<Vec<(Option<usize>, usize, usize)>>(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day9 = Day9::new("data/test_data.txt").unwrap();
        let expected = 1928;
        let actual = day9.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day9 = Day9::new("data/test_data.txt").unwrap();
        let expected = 2858;
        let actual = day9.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day9 = Day9::new("data/input_data.txt").unwrap();
        let expected = 6241633730082;
        let actual = day9.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day9 = Day9::new("data/input_data.txt").unwrap();
        let expected = 6265268809555;
        let actual = day9.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_checksum() {
        let day9 = Day9::new("data/test_data.txt").unwrap();
        let input = [
            (Some(0), 0, 2),
            (Some(9), 2, 4),
            (Some(2), 4, 5),
            (Some(1), 5, 8),
            (Some(7), 8, 11),
            (None, 11, 12),
            (Some(4), 12, 14),
            (None, 14, 15),
            (Some(3), 15, 18),
            (None, 18, 22),
            (Some(5), 22, 26),
            (None, 26, 27),
            (Some(6), 27, 31),
            (None, 31, 36),
            (Some(8), 36, 40),
        ];
        let actual = day9.calculate_checksum(&input);
        let expected = 2858;
        assert_eq!(expected, actual);
    }
}
