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

    #[allow(dead_code)]
    fn build_expression(
        &self,
        output: &str,
        equations: &HashMap<String, usize>,
        indent_level: usize,
    ) -> String {
        if self.initial_values.contains_key(output) {
            format!("{} ({})", output, self.initial_values.get(output).unwrap())
        } else {
            let equation = self
                .equations
                .iter()
                .find(|(_, _, _, out)| out == output)
                .unwrap();
            let tabs = "\t".repeat(indent_level);
            let mut leftright = [
                self.build_expression(&equation.0, equations, indent_level + 1),
                self.build_expression(&equation.2, equations, indent_level + 1),
            ];
            leftright.sort();
            format!(
                "{}:\n\t{}{}\n\t{}{}\n{}{}({})",
                equation.1,
                tabs,
                leftright[0],
                tabs,
                leftright[1],
                tabs,
                equation.3,
                equations.get(&equation.3).unwrap()
            )
        }
    }

    fn find_swap_pairs(&mut self) -> String {
        // We want z[0]..z[n] to be the sum of x[0]..x[n-1] and y[0]..y[n-1]
        //
        // z[0] should be x[0] xor y[0]
        // z[1] should be x[1] xor y[1] but also needs the "carry", so also xor (x[0] and y[0])
        // The tricky part is how do we work out the "carry" for z[2]. If we store the
        // carry for x[0] and y[0] as c[0], then we should expect
        // the carry c[1] to be x[1] and y[1] or x[1] and c[0] or y[1] and c[0]
        //
        // Let's just start by printing the "expression" of each z entry and see what it looks like
        //
        // and printing out when its wrong
        //
        // note: I kind of fucked up and got the right answer then used `git checkout --` to undo
        // my input_data changes and lost all of my work here instead. Trying to reproduce:
        //
        // First incorrect is z07, which is just the AND value instead of doing the correct stuff
        // looks like swt is correct for this
        //
        // Next incorrect is z13, which while not giving the wrong result, starts with an OR
        // so is the carry. This needs to be swapped with pqc
        //
        // Next incorrect is z24, which is using the AND instead of the XOR of the input wires
        // so we need to swap rjm with wsv
        //
        // Lastly we have z31, which is an AND instead of an XOR, and is representing its carry
        // needs to be swapped with bgs
        //
        // The sorted list of wires is thus:
        // bgs,pqc,rjm,swt,wsv,z07,z13,z31
        self.swap_wires("z07", "swt");
        self.swap_wires("z13", "pqc");
        self.swap_wires("rjm", "wsv");
        self.swap_wires("bgs", "z31");

        let z = self.sum_x_and_y();
        let equations = self.run_equations();
        let mut sorted_z: Vec<String> = self
            .equations
            .iter()
            .map(|(_, _, _, name)| name.clone())
            .filter(|name| name.starts_with('z'))
            .collect();
        sorted_z.sort();
        let mut found = true;
        for z_out in sorted_z.into_iter() {
            // println!("Finding equation for {}", z_out);
            let index = z_out[1..z_out.len()].parse::<usize>().unwrap();

            if *equations.get(&z_out).unwrap() != (z >> index) % 2 {
                // println!("Value incorrect at: {}", z_out);
                found = false;
            } else {
                // println!("Value correct at: {}", z_out);
            }
            // println!("{}", self.build_expression(&z_out, &equations, 0));
        }
        assert!(found);
        "bgs,pqc,rjm,swt,wsv,z07,z13,z31".to_string()
    }

    fn swap_wires(&mut self, a: &str, b: &str) {
        for wire in self.equations.iter_mut() {
            if wire.3 == a {
                wire.3 = b.to_string();
            } else if wire.3 == b {
                wire.3 = a.to_string();
            }
        }
    }

    fn calculate_day_b(&self) -> String {
        let mut obj = self.clone();
        obj.find_swap_pairs()
    }

    fn sum_x_and_y(&self) -> usize {
        let mut x = 0;
        let mut y = 0;
        for (key, value) in self.initial_values.iter() {
            let index = key[1..key.len()].parse::<usize>().unwrap();
            if key.starts_with('x') {
                x |= value << index
            } else {
                y |= value << index;
            }
        }
        x + y
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
        //There is no test data for part b
        // let day24 = Day24::new("data/test_data.txt").unwrap();
        // let expected = 0;
        // let actual = day24.calculate_day_b();
        // assert_eq!(expected, actual);
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
        let expected = "bgs,pqc,rjm,swt,wsv,z07,z13,z31".to_string();
        let actual = day24.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
