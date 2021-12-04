extern crate peg;
#[derive(PartialEq, Debug, Clone)]
pub enum BinaryValue {
    One,
    Zero,
}

#[derive(PartialEq, Clone)]
pub struct BingoBoard {
    values: [[usize; 5]; 5],
}

pub struct BingoMask {
    values: [[bool; 5]; 5],
}

pub struct BingoSetup {
    bingo_boards: Vec<BingoBoard>,
    bingo_numbers: Vec<usize>,
}

peg::parser! { grammar day4_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule bingo_numbers() -> Vec<usize>
        = n:number() ++ "," { n }
    rule bingo_line() -> Vec<usize>
        = " " * n:number() **<5,5> (" " *) { n }
    rule bingo_board() -> Vec<Vec<usize>>
        = line: bingo_line() **<5,5> "\n" {
            line
        }
    pub rule parse() -> BingoSetup
        = bingo_numbers:bingo_numbers() ("\n" +) bingo_boards:bingo_board() ** ("\n" +) "\n" * {
            let bingo_boards: Vec<BingoBoard> = bingo_boards.iter().map(|board_data|BingoBoard::new(&board_data[..])).collect();
            BingoSetup{bingo_boards, bingo_numbers}
        }

}
}

impl BingoBoard {
    fn new(bingo_input: &[Vec<usize>]) -> BingoBoard {
        if bingo_input.len() != 5 {
            panic!(
                "Invalid BingoBoard height {}, {:?}",
                bingo_input.len(),
                bingo_input
            );
        }
        let mut ret: [[usize; 5]; 5] = [[0; 5]; 5];
        for i in 0..5 {
            for j in 0..5 {
                ret[i][j] = bingo_input[i][j];
            }
        }
        BingoBoard { values: ret }
    }

    fn get_found_mask(self: &BingoBoard, current_bingo_numbers: &[usize]) -> BingoMask {
        let mut values = [[true; 5]; 5];
        for i in 0..5 {
            for j in 0..5 {
                values[i][j] = current_bingo_numbers.contains(&self.values[i][j]);
            }
        }
        BingoMask { values }
    }

    fn get_victory_score(self: &BingoBoard, current_bingo_numbers: &[usize]) -> Option<usize> {
        let mask = self.get_found_mask(current_bingo_numbers);
        if mask.is_victory() {
            Some(self.calculate_score(mask, current_bingo_numbers[current_bingo_numbers.len() - 1]))
        } else {
            None
        }
    }

    fn calculate_score(self: &BingoBoard, mask: BingoMask, final_number: usize) -> usize {
        let mut sum_not_calculated = 0;
        for i in 0..5 {
            for j in 0..5 {
                if !mask.values[i][j] {
                    sum_not_calculated += self.values[i][j]
                }
            }
        }
        sum_not_calculated * final_number
    }
}

impl BingoMask {
    fn is_victory(self: &BingoMask) -> bool {
        for row in 0..5 {
            if self.values[row]
                .iter()
                .fold(0, |acc, val| if *val { acc + 1 } else { acc })
                == 5
            {
                return true;
            }
        }
        for col in 0..5 {
            if self
                .values
                .iter()
                .fold(0, |acc, line| if line[col] { acc + 1 } else { acc })
                == 5
            {
                return true;
            }
        }
        false
    }
}

impl BingoSetup {
    fn new(bingo_setup_input: &str) -> BingoSetup {
        day4_parser::parse(bingo_setup_input).unwrap()
    }
    fn calculate_day_a(self: &BingoSetup) -> usize {
        for i in 1..=self.bingo_numbers.len() {
            let current_bingo_numbers = &self.bingo_numbers[0..i];
            for bingo_board in &self.bingo_boards {
                if let Some(result) = bingo_board.get_victory_score(current_bingo_numbers) {
                    return result;
                }
            }
        }
        panic!("No bingo result found");
    }

    fn get_excluded_bingo_boards(
        bingo_boards: Vec<BingoBoard>,
        bingo_board_to_exclude: &BingoBoard,
    ) -> Vec<BingoBoard> {
        bingo_boards
            .into_iter()
            .filter(|board| board != bingo_board_to_exclude)
            .collect()
    }

    fn calculate_day_b(self: &BingoSetup) -> usize {
        let mut unwon_bingo_boards;
        let mut next_bingo_boards = self.bingo_boards.clone();
        for i in 1..=self.bingo_numbers.len() {
            let current_bingo_numbers = &self.bingo_numbers[0..i];
            unwon_bingo_boards = next_bingo_boards.clone();
            for bingo_board in unwon_bingo_boards.iter() {
                if let Some(result) = bingo_board.get_victory_score(current_bingo_numbers) {
                    if next_bingo_boards.len() == 1 {
                        return result;
                    } else {
                        next_bingo_boards =
                            BingoSetup::get_excluded_bingo_boards(next_bingo_boards, bingo_board);
                    }
                }
            }
        }
        panic!("No bingo result found");
    }
}

fn main() {
    let bingo_setup = BingoSetup::new(include_str!("../input_data.txt"));
    let day_a = bingo_setup.calculate_day_a();
    println!("Day a result: {}", day_a);
    let day_b = bingo_setup.calculate_day_b();
    println!("Day b result: {}", day_b);
}

#[cfg(test)]
mod test {
    use crate::BingoMask;
    use crate::BingoSetup;

    #[test]
    fn test_parse() {
        let bingo_setup = BingoSetup::new(include_str!("../test_data.txt"));
        assert_eq!(bingo_setup.bingo_boards.len(), 3);
    }

    #[test]
    fn test_mask_victory_check() {
        let bingo_mask = BingoMask {
            values: [
                [false, false, true, false, false],
                [false, false, true, false, false],
                [false, false, true, false, false],
                [false, false, true, false, false],
                [false, false, true, false, false],
            ],
        };
        assert!(bingo_mask.is_victory());
        let bingo_mask = BingoMask {
            values: [
                [false, false, false, false, false],
                [false, false, false, false, false],
                [true, true, true, true, true],
                [false, false, false, false, false],
                [false, false, false, false, false],
            ],
        };
        assert!(bingo_mask.is_victory());
        let bingo_mask = BingoMask {
            values: [
                [false, true, false, false, false],
                [false, true, false, false, false],
                [true, false, true, true, true],
                [false, true, false, false, false],
                [false, true, false, false, false],
            ],
        };
        assert!(!bingo_mask.is_victory());
    }

    #[test]
    fn test_day_a() {
        let bingo_setup = BingoSetup::new(include_str!("../test_data.txt"));
        assert_eq!(bingo_setup.calculate_day_a(), 4512);
    }

    #[test]
    fn test_day_b() {
        let bingo_setup = BingoSetup::new(include_str!("../test_data.txt"));
        assert_eq!(bingo_setup.calculate_day_b(), 1924);
    }
}
