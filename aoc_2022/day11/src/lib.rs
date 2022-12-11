mod monkey;
mod parser;
use aoc_helpers::hash_utils::HashVec;
use aoc_helpers::modular_math::least_common_multiple;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};
use monkey::Monkey;
use parser::parse_data;

#[derive(Clone)]
pub struct Day11 {
    monkeys: Vec<Monkey>,
}

impl AOCCalculator for Day11 {
    fn new(filename: &str) -> Result<Day11, AOCFileOrParseError> {
        Ok(Day11 {
            monkeys: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        let mut obj = self.clone();
        println!("{}a answer is {:?}", name, obj.calculate_day_a());
        let mut obj = self.clone();
        println!("{}b answer is {:?}", name, obj.calculate_day_b());
    }
}

impl Day11 {
    fn calculate_day_a(&mut self) -> usize {
        for _ in 0..20 {
            self.run_one_round_day_a();
        }
        self.get_monkey_score()
    }

    fn calculate_day_b(&mut self) -> usize {
        let shared_modulo = self.get_shared_modulo();
        for _ in 0..10000 {
            self.run_one_round_day_b(shared_modulo);
        }
        self.get_monkey_score()
    }

    fn run_one_round_day_a(&mut self) {
        for i in 0..self.monkeys.len() {
            let result = self.monkeys[i].take_turn_day_a();
            self.monkeys[i] = self.monkeys[i].empty_monkey();
            self.apply_result_to_monkeys(&result);
        }
    }

    fn run_one_round_day_b(&mut self, shared_modulo: usize) {
        for i in 0..self.monkeys.len() {
            let result = self.monkeys[i].take_turn_day_b(shared_modulo);
            self.monkeys[i] = self.monkeys[i].empty_monkey();
            self.apply_result_to_monkeys(&result);
        }
    }

    fn apply_result_to_monkeys(&mut self, result: &HashVec<usize, usize>) {
        self.monkeys = self
            .monkeys
            .iter()
            .map(|monkey| monkey.next_turn(result))
            .collect();
    }

    fn get_monkey_score(&self) -> usize {
        let mut processed = self
            .monkeys
            .iter()
            .map(|monkey| monkey.items_processed as usize)
            .collect::<Vec<usize>>();
        processed.sort();
        processed.reverse();
        processed[0] * processed[1]
    }

    fn get_shared_modulo(&self) -> usize {
        let shared_modulo = self
            .monkeys
            .iter()
            .map(|monkey| monkey.test_condition)
            .fold(1, least_common_multiple);
        for monkey in self.monkeys.iter() {
            assert_eq!((shared_modulo % monkey.test_condition), 0);
        }
        shared_modulo
    }
}

impl std::fmt::Display for Day11 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for monkey in self.monkeys.iter() {
            writeln!(f, "Monkey {}: {:?}", monkey.index, monkey.starting_items)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let mut day11 = Day11::new("data/test_data.txt").unwrap();
        let expected = 10605;
        let actual = day11.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_data_day_b() {
        let mut day11 = Day11::new("data/input_data.txt").unwrap();
        assert_eq!(day11.monkeys.len(), 8);
        // answer less than this
        assert_ne!(day11.calculate_day_b(), 13985281920);
    }

    #[test]
    fn test_calculate_day_b() {
        let mut day11 = Day11::new("data/test_data.txt").unwrap();
        let expected = 2713310158;
        let actual = day11.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
