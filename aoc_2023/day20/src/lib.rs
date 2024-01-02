mod parser;
mod pulse_module;
use std::collections::{HashMap, VecDeque};

use crate::parser::parse_data;
use crate::pulse_module::PulseModule;
use aoc_helpers::{
    hash_utils::HashVec, modular_math::least_common_multiple, read_input_file, AOCCalculator,
    AOCFileOrParseError,
};
use pulse_module::{Pulse, PulseModuleType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day20 {
    modules: HashMap<String, PulseModule>,
    rx_input: String,
}

fn update_conjunction_defaults(modules: &mut HashMap<String, PulseModule>) {
    let all_conjuctions: Vec<String> = modules
        .values()
        .filter(|m| matches!(m.pulse_type, PulseModuleType::Conjunction(_)))
        .map(|m| m.input.to_owned())
        .collect();
    let mut mapping = HashVec::new();
    for pulse_module in modules.values() {
        for output in pulse_module.outputs.iter() {
            if all_conjuctions.contains(output) {
                mapping.push(output.to_owned(), pulse_module.input.to_owned());
            }
        }
    }
    for (conjunction, inputs) in mapping.iter() {
        let mut state = HashMap::new();
        for input in inputs.iter() {
            state.insert(input.clone(), Pulse::Low);
        }
        modules
            .entry(conjunction.clone())
            .and_modify(|pulse_module| {
                pulse_module.pulse_type = PulseModuleType::Conjunction(state)
            });
    }
}

impl AOCCalculator for Day20 {
    fn new(filename: &str) -> Result<Day20, AOCFileOrParseError> {
        let data = parse_data(&read_input_file(filename)?)?;
        let mut modules = HashMap::new();
        for pulse_module in data.into_iter() {
            modules.insert(pulse_module.input.to_owned(), pulse_module);
        }
        update_conjunction_defaults(&mut modules);
        let rx_input = modules
            .values()
            .find(|m| m.outputs.contains(&"rx".to_string()))
            .map(|m| m.input.clone())
            .unwrap_or("rx".to_string());

        Ok(Day20 { modules, rx_input })
    }

    fn print_results(&self, name: &str) {
        let mut part_a = self.to_owned();
        println!("{}a answer is {:?}", name, part_a.calculate_day_a());
        let mut part_b = self.to_owned();
        println!("{}b answer is {:?}", name, part_b.calculate_day_b());
    }
}

impl Day20 {
    fn push_button_once(&mut self) -> (usize, usize) {
        let mut queue = VecDeque::new();
        queue.push_back(("broadcaster".to_string(), "button".to_string(), Pulse::Low));
        let mut low_count = 0;
        let mut high_count = 0;
        while let Some((dest, source, pulse)) = queue.pop_front() {
            if pulse == Pulse::Low {
                low_count += 1
            } else {
                high_count += 1
            }
            if let Some(pulse_module) = self.modules.get_mut(&dest) {
                if let Some(next_pulse) = pulse_module.receive_pulse(&source, &pulse) {
                    for output in pulse_module.outputs.iter() {
                        queue.push_back((output.clone(), dest.clone(), next_pulse));
                    }
                }
            }
        }
        (low_count, high_count)
    }

    fn calculate_day_a(&mut self) -> usize {
        let mut low_count = 0;
        let mut high_count = 0;
        for _ in 0..1000 {
            let (next_low, next_high) = self.push_button_once();
            low_count += next_low;
            high_count += next_high;
        }
        low_count * high_count
    }

    fn get_inputs(&self, key: &str) -> HashMap<String, Pulse> {
        if let PulseModuleType::Conjunction(inputs) =
            self.modules.get(key).unwrap().pulse_type.clone()
        {
            inputs
        } else {
            HashMap::new()
        }
    }

    fn get_rx_inputs(&self) -> HashMap<String, Pulse> {
        self.get_inputs(&self.rx_input)
    }

    fn calculate_day_b(&mut self) -> usize {
        let rx_input_key_len = self.get_rx_inputs().len();
        let mut count_until: HashMap<String, usize> = HashMap::new();
        let mut count = 0;
        loop {
            self.push_button_once();
            count += 1;
            for rx_input in self.get_rx_inputs().into_iter() {
                if self.modules.get(&rx_input.0).unwrap().counts.1 > 0 {
                    count_until.entry(rx_input.0).or_insert(count);
                }
            }
            if count_until.len() == rx_input_key_len {
                break;
            }
        }
        count_until
            .into_values()
            .reduce(least_common_multiple)
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a_first_example() {
        let mut day20 = Day20::new("data/test_data.txt").unwrap();
        let expected = 32000000;
        let actual = day20.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_a_second_example() {
        let mut day20 = Day20::new("data/test_data_b.txt").unwrap();
        let expected = 11687500;
        let actual = day20.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let mut day20 = Day20::new("data/input_data.txt").unwrap();
        let expected = 839775244;
        let actual = day20.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let mut day20 = Day20::new("data/input_data.txt").unwrap();
        let expected = 207787533680413;
        let actual = day20.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
