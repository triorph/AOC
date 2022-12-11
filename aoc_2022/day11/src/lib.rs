mod monkey;
mod parser;
use aoc_helpers::{lcm, read_input_file, AOCCalculator, AOCFileOrParseError};
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

    fn run_one_round_day_a(&mut self) {
        for i in 0..self.monkeys.len() {
            let (result, monkey) = self.monkeys[i].take_turn_day_a();
            self.monkeys[i] = monkey;
            self.monkeys = self
                .monkeys
                .iter()
                .map(|monkey| monkey.next_turn(&result))
                .collect();
        }
    }

    fn run_one_round_day_b(&mut self, shared_modulo: usize) {
        for i in 0..self.monkeys.len() {
            let (result, monkey) = self.monkeys[i].take_turn_day_b(shared_modulo);
            self.monkeys[i] = monkey;
            self.monkeys = self
                .monkeys
                .iter()
                .map(|monkey| monkey.next_turn(&result))
                .collect();
        }
    }

    fn get_monkey_score(&self) -> usize {
        let mut processed = self
            .monkeys
            .iter()
            .map(|monkey| monkey.items_processed)
            .collect::<Vec<usize>>();
        processed.sort();
        processed.reverse();
        println!("processed: {:?}", processed);
        processed[0] * processed[1]
    }

    fn calculate_day_b(&mut self) -> usize {
        let shared_modulo = lcm(&self
            .monkeys
            .iter()
            .map(|monkey| monkey.test_condition)
            .collect::<Vec<usize>>());
        for monkey in self.monkeys.iter() {
            println!(
                "monkey.test_condition % shared_modulo
                        : {:?}",
                (shared_modulo % monkey.test_condition)
            );
        }
        println!("shared_modulo: {:?}", shared_modulo);
        for i in 0..10000 {
            if i % 1000 == 0 {
                println!("i: {:?}", i);
                println!("self.get_monkey_score(): {:?}", self.get_monkey_score());
            }
            self.run_one_round_day_b(shared_modulo);
        }
        self.get_monkey_score()
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
    fn test_calculate_day_b() {
        let mut day11 = Day11::new("data/test_data.txt").unwrap();
        let expected = 2713310158;
        let actual = day11.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
