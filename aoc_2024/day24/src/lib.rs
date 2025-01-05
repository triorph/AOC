mod parser;
use std::collections::HashMap;

use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};
use parser::{Operation, OperationLine};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day24 {
    initial_values: HashMap<String, usize>,
    equations: Vec<OperationLine>,
}

impl AOCCalculator for Day24 {
    fn new(filename: &str) -> Result<Day24, AOCFileOrParseError> {
        let (initial_values, equations) = parse_data(&read_input_file(filename)?)?;
        Ok(Day24 {
            initial_values,
            equations,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day24 {
    fn run_equations(&self) -> HashMap<String, usize> {
        let mut values = self.initial_values.clone();
        let mut equations: Vec<OperationLine> = self
            .equations
            .clone()
            .into_iter()
            .filter(|(_, _, _, out)| !values.contains_key(out))
            .collect();
        let mut to_calculate: Vec<OperationLine> = equations
            .clone()
            .into_iter()
            .filter(|(in1, _, in2, _)| values.contains_key(in1) && values.contains_key(in2))
            .collect();
        while !to_calculate.is_empty() {
            for (in1, op, in2, out) in to_calculate.iter() {
                let val = match (values.get(in1), op, values.get(in2)) {
                    (Some(in1), Operation::Or, Some(in2)) => in1 | in2,
                    (Some(in1), Operation::Xor, Some(in2)) => in1 ^ in2,
                    (Some(in1), Operation::And, Some(in2)) => in1 & in2,
                    _ => panic!("in1 or in2 were None but shouldn't have been"),
                };
                values.insert(out.clone(), val);
            }
            equations = self
                .equations
                .clone()
                .into_iter()
                .filter(|(_, _, _, out)| !values.contains_key(out))
                .collect();
            to_calculate = equations
                .clone()
                .into_iter()
                .filter(|(in1, _, in2, _)| values.contains_key(in1) && values.contains_key(in2))
                .collect();
        }
        values
    }

    fn calculate_day_a(&self) -> usize {
        let values = self.run_equations();
        let mut z_entries = values
            .into_iter()
            .filter(|(k, _)| k.starts_with('z'))
            .collect::<Vec<(String, usize)>>();
        z_entries.sort();
        z_entries
            .into_iter()
            .rev()
            .map(|(_, v)| v)
            .fold(0, |acc, x| acc * 2 + x)
    }

    fn build_expression(&self, output: &str) -> String {
        if self.initial_values.contains_key(output) {
            format!("{} ({})", output, self.initial_values.get(output).unwrap())
        } else {
            let equation = self
                .equations
                .iter()
                .find(|(_, _, _, out)| out == output)
                .unwrap();
            format!(
                "({} {} {})",
                self.build_expression(&equation.0),
                equation.1,
                self.build_expression(&equation.2)
            )
        }
    }

    fn find_swap_pairs(&self) -> usize {
        // We want z[0]..z[n] to be the sum of x[0]..x[n-1] and y[0]..y[n-1]
        //
        // z[0] should be x[0] xor y[0]
        // z[1] should be x[1] xor y[1] but also needs the "carry", so also xor (x[0] and y[0])
        // The tricky part is how do we work out the "carry" for z[2]. If we store the
        // carry for x[0] and y[0] as c[0], then we should expect
        // the carry c[1] to be x[1] and y[1] or x[1] and c[0] or y[1] and c[0]
        //
        // Let's just start by printing the "expression" of each z entry and see what it looks like
        for z_out in self
            .equations
            .iter()
            .filter(|(_, _, _, out)| out.starts_with('z'))
        {
            println!("Finding equation for {}", z_out.3);
            println!("{}", self.build_expression(&z_out.3));
        }
        0
    }

    fn calculate_day_b(&self) -> usize {
        self.find_swap_pairs();
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day24 = Day24::new("data/test_data.txt").unwrap();
        let expected = 2024;
        let actual = day24.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day24 = Day24::new("data/test_data.txt").unwrap();
        let expected = 0;
        let actual = day24.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day24 = Day24::new("data/input_data.txt").unwrap();
        let expected = 65740327379952;
        let actual = day24.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day24 = Day24::new("data/input_data.txt").unwrap();
        let expected = 0;
        let actual = day24.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
