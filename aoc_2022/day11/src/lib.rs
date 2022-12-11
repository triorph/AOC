mod monkey;
mod parser;
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
        println!("{}b answer is {:?}", name, obj.calculate_day_b());
    }
}

impl Day11 {
    fn calculate_day_a(&mut self) -> usize {
        for _ in 0..20 {
            self.run_one_round();
            println!("self: {}", self);
        }
        self.get_monkey_score()
    }

    fn run_one_round(&mut self) {
        for i in 0..self.monkeys.len() {
            let (result, monkey) = self.monkeys[i].take_turn();
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
        processed[0] * processed[1]
    }

    fn calculate_day_b(&self) -> usize {
        0
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
        let day11 = Day11::new("data/test_data.txt").unwrap();
        let expected = 0;
        let actual = day11.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
