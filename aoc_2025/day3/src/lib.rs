mod parser;
use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day3 {
    power_banks: Vec<Vec<usize>>,
}

impl AOCCalculator for Day3 {
    fn new(filename: &str) -> Result<Day3, AOCFileOrParseError> {
        Ok(Day3 {
            power_banks: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day3 {
    fn find_best_battery_for_digit(&self, power_bank: &[usize], digit: usize) -> (usize, usize) {
        let bank_to_consider = &power_bank[..power_bank.len() - (digit - 1)];
        let max = bank_to_consider.iter().max().unwrap();
        bank_to_consider
            .iter()
            .copied()
            .enumerate()
            .find(|(_, x)| x == max)
            .unwrap()
    }

    fn find_highest_joltage(&self, power_bank: &[usize], num_batteries: usize) -> usize {
        (1..=num_batteries)
            .rev()
            .fold((0, power_bank), |(joltage, remaining_bank), digit| {
                let (next_battery_i, next_battery) =
                    self.find_best_battery_for_digit(remaining_bank, digit);
                (
                    joltage * 10 + next_battery,
                    &remaining_bank[next_battery_i + 1..remaining_bank.len()],
                )
            })
            .0
    }

    fn find_highest_joltage_part_a(&self, power_bank: &[usize]) -> usize {
        self.find_highest_joltage(power_bank, 2)
    }

    fn find_highest_joltage_part_b(&self, power_bank: &[usize]) -> usize {
        self.find_highest_joltage(power_bank, 12)
    }

    fn calculate_day_a(&self) -> usize {
        self.power_banks
            .iter()
            .map(|power_bank| self.find_highest_joltage_part_a(power_bank))
            .sum()
    }

    fn calculate_day_b(&self) -> usize {
        self.power_banks
            .iter()
            .map(|power_bank| self.find_highest_joltage_part_b(power_bank))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case([9,8,7,6,5,4,3,2,1,1,1,1,1,1,1], 98)]
    #[case([8,1,1,1,1,1,1,1,1,1,1,1,1,1,9], 89)]
    #[case([2,3,4,2,3,4,2,3,4,2,3,4,2,7,8], 78)]
    #[case([8,1,8,1,8,1,9,1,1,1,1,2,1,1,1], 92)]
    fn test_find_highest_joltage_part_a(#[case] input: [usize; 15], #[case] expected: usize) {
        let day3 = Day3::new("data/test_data.txt").unwrap();
        let actual = day3.find_highest_joltage_part_a(&input);
        assert_eq!(expected, actual)
    }

    #[rstest]
    #[case([9,8,7,6,5,4,3,2,1,1,1,1,1,1,1], 987654321111)]
    #[case([8,1,1,1,1,1,1,1,1,1,1,1,1,1,9], 811111111119)]
    #[case([2,3,4,2,3,4,2,3,4,2,3,4,2,7,8], 434234234278)]
    #[case([8,1,8,1,8,1,9,1,1,1,1,2,1,1,1], 888911112111)]
    fn test_find_highest_joltage_part_b(#[case] input: [usize; 15], #[case] expected: usize) {
        let day3 = Day3::new("data/test_data.txt").unwrap();
        let actual = day3.find_highest_joltage_part_b(&input);
        assert_eq!(expected, actual)
    }

    #[test]
    fn test_calculate_day_a() {
        let day3 = Day3::new("data/test_data.txt").unwrap();
        let expected = 357;
        let actual = day3.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day3 = Day3::new("data/test_data.txt").unwrap();
        let expected = 3121910778619;
        let actual = day3.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day3 = Day3::new("data/input_data.txt").unwrap();
        let expected = 17332;
        let actual = day3.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day3 = Day3::new("data/input_data.txt").unwrap();
        let expected = 172516781546707;
        let actual = day3.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
