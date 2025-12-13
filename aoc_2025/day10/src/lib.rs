mod parser;
use crate::parser::{parse_data, Machine};
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};
use itertools::Itertools;
use z3::{ast::Int, Optimize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day10 {
    machines: Vec<Machine>,
}

impl AOCCalculator for Day10 {
    fn new(filename: &str) -> Result<Day10, AOCFileOrParseError> {
        Ok(Day10 {
            machines: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day10 {
    fn calculate_day_a(&self) -> usize {
        self.machines
            .iter()
            .map(|machine| self.per_machine_solve_part_a(machine))
            .sum()
    }

    fn calculate_day_b(&self) -> usize {
        self.machines
            .iter()
            .map(|machine| self.per_machine_solve_part_b_z3(machine))
            .sum()
    }

    fn per_machine_solve_part_a(&self, machine: &Machine) -> usize {
        (1..=machine.1.len())
            .find(|combination_length| {
                self.does_combination_solve_machine_part_a(machine, *combination_length)
            })
            .unwrap()
    }

    fn does_combination_solve_machine_part_a(
        &self,
        machine: &Machine,
        combination_length: usize,
    ) -> bool {
        let (target, buttons, _) = machine;
        buttons
            .iter()
            .combinations(combination_length)
            .map(|buttons_to_try| self.sum_button_presses_part_a(target.len(), &buttons_to_try))
            .any(|result| &result == target)
    }

    fn sum_button_presses_part_a(&self, target_len: usize, buttons: &[&Vec<usize>]) -> Vec<bool> {
        buttons
            .iter()
            .fold(vec![false; target_len], |mut ret, button| {
                button.iter().for_each(|i| {
                    ret[*i] ^= true;
                });
                ret
            })
    }

    ///
    /// We can think of this as a simultaneous equation, probably over subscribed (which means
    /// finding the min))
    ///
    /// let us use  (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7} as the example
    ///
    /// so we have 6 different buttons that can be pressed, which means we need
    /// 6 different values to work through. Let's call these s_0 to s_5
    ///
    /// on each axis we know how much each value is assigned, e.g.
    /// for 0, only s_4 and s_5, so s_4 + s_5 == 3
    /// for 1, only s_1, and s_5, so s_1 + s_5 == 5
    /// for 2, only s_2, s_3 and s_4, so s_2 + s_3 + s_4 == 4
    /// for 3, only s_0, s_1 and s_3 so s_0 + s_1 + s_3 == 7
    ///
    /// This is a series of simultaneous equations, however we have 4 equations and 6 variables, so
    /// once we have reduced we still need to minimise
    ///
    /// For reference: AOC test data says the actual solution is:
    /// s=[1,3,0,3,1,2]
    ///
    /// Let us quickly write out the gaussian elimination for this
    ///
    /// 0 0 0 0 1 1 | 3
    /// 0 1 0 0 0 1 | 5
    /// 0 0 1 1 1 0 | 4
    /// 1 1 0 1 0 0 | 7
    /// ---------------
    /// subtract L2 from L4 (Might be tricky to programmatically figure out which variables are the
    /// "roots" that we can subtract from others)
    /// 0 0 0 0 1 1 | 3
    /// 0 1 0 0 0 1 | 5
    /// 0 0 1 1 1 0 | 4
    /// 1 0 0 1 0 -1 | 2
    /// ---------------
    /// subtract L1 from L3
    /// 0 0 0 0 1 1 | 3
    /// 0 1 0 0 0 1 | 5
    /// 0 0 1 1 0 -1 | 1
    /// 1 0 0 1 0 -1 | 2
    /// ---------------
    /// so unknown variables are s_3 and s_5, how do we minimise?
    ///
    /// Typically with something like this we would use LMS to work out the best answer:
    /// question: if we're doing the LMS do we even need the gaussian elimination first?
    ///
    ///  Start with s=[0;6]
    ///  Error = [3, 5, 1, 2];
    ///  update by something (let's say sqrt(value))
    ///  s = [sqrt(2), sqrt(5), sqrt(1), sqrt(1) + sqrt(2), sqrt(3), sqrt(3) + sqrt(5) - sqrt(1) - sqrt(2)]
    ///  s = [1.414, 2.236, 1.0, 2.414, 1.732, 1.554]
    ///
    ///  gives us values: [3.286, 3.790, 1.860, 2.27]
    ///
    #[allow(dead_code)]
    fn per_machine_solve_part_b(&self, _machine: &Machine) -> usize {
        0
    }

    fn per_machine_solve_part_b_z3(&self, machine: &Machine) -> usize {
        let (_, buttons, targets) = machine;
        // the number of times we press each button
        let variables: Vec<Int> = (0..buttons.len())
            .map(|i| Int::fresh_const(&format!("s{}", i)))
            .collect();
        let optimize = Optimize::new();
        // we cannot negative press buttons
        for variable in variables.iter() {
            optimize.assert(&variable.ge(0));
        }
        // assert that button presses give our targets
        for (i, target) in targets.iter().enumerate() {
            optimize.assert(
                &buttons
                    .iter()
                    .enumerate()
                    .filter(|(_, button)| button.contains(&i))
                    .map(|(var_index, _)| &variables[var_index])
                    .sum::<Int>()
                    .eq(Int::from_u64(*target as u64)),
            )
        }
        // the goal we minimise towards
        let ret = Int::fresh_const("ret");
        // assert that our goal is the sum of all the button presses
        optimize.assert(&variables.iter().sum::<Int>().eq(&ret));
        // minimise
        optimize.minimize(&ret);
        optimize.check(&[]);
        // retrieve the result
        let model = optimize.get_model().unwrap();
        model.get_const_interp(&ret).unwrap().as_u64().unwrap() as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day10 = Day10::new("data/test_data.txt").unwrap();
        let expected = 7;
        let actual = day10.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day10 = Day10::new("data/test_data.txt").unwrap();
        let expected = 33;
        let actual = day10.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day10 = Day10::new("data/input_data.txt").unwrap();
        let expected = 473;
        let actual = day10.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day10 = Day10::new("data/input_data.txt").unwrap();
        let expected = 0;
        let actual = day10.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
