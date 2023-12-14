mod parser;
use crate::parser::parse_data;
use crate::parser::Tile;
use aoc_helpers::vec::Rotatable;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};

#[derive(Clone)]
pub struct Day14 {
    tiles: Vec<Vec<Tile>>,
}

impl AOCCalculator for Day14 {
    fn new(filename: &str) -> Result<Day14, AOCFileOrParseError> {
        Ok(Day14 {
            tiles: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day14 {
    fn print(&self) -> String {
        let mut ret: String = String::new();
        for line in self.tiles.iter() {
            for tile in line.iter() {
                ret += match tile {
                    Tile::Empty => " ",
                    Tile::Rock => "O",
                    Tile::Wall => "#",
                };
            }
            ret += "\n"
        }
        ret
    }
    fn find_highest_empty_above(&self, x: usize, y: usize) -> usize {
        for target in (0..y).rev() {
            if self.tiles[target][x] != Tile::Empty {
                return target + 1;
            }
        }
        0
    }

    fn swap_tiles(&mut self, x: usize, y1: usize, y2: usize) {
        if y1 == y2 {
            return;
        }
        let tmp = self.tiles[y1][x];
        self.tiles[y1][x] = self.tiles[y2][x];
        self.tiles[y2][x] = tmp;
    }

    fn shift_all_north(&mut self) {
        for y in 1..self.tiles.len() {
            for x in 0..self.tiles[y].len() {
                if self.tiles[y][x] == Tile::Rock {
                    let target = self.find_highest_empty_above(x, y);
                    self.swap_tiles(x, y, target);
                }
            }
        }
    }

    fn calculate_load(&self) -> usize {
        self.tiles
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line.iter().filter(|&&tile| tile == Tile::Rock).count() * (self.tiles.len() - y)
            })
            .sum()
    }

    fn calculate_day_a(&self) -> usize {
        let mut obj = self.to_owned();
        obj.shift_all_north();
        obj.calculate_load()
    }

    fn run_one_cycle(&mut self) {
        for _ in 0..4 {
            self.shift_all_north();
            self.tiles = self.tiles.rot90();
        }
    }

    fn calculate_day_b(&self) -> usize {
        let mut obj = self.to_owned();
        for _ in 0..1000 {
            // just tried 1000 instead of 1billion and it worked.. lol
            obj.run_one_cycle();
        }
        obj.calculate_load()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day14 = Day14::new("data/test_data.txt").unwrap();
        let expected = 136;
        let actual = day14.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day14 = Day14::new("data/test_data.txt").unwrap();
        let expected = 64;
        let actual = day14.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day14 = Day14::new("data/input_data.txt").unwrap();
        let expected = 109939;
        let actual = day14.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day14 = Day14::new("data/input_data.txt").unwrap();
        let expected = 101010;
        let actual = day14.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
