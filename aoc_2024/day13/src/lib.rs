mod parser;
use crate::parser::parse_data;
use aoc_helpers::{point2d::Point2D, read_input_file, AOCCalculator, AOCFileOrParseError};
use parser::ClawMachine;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day13 {
    claw_machines: Vec<ClawMachine>,
}

impl AOCCalculator for Day13 {
    fn new(filename: &str) -> Result<Day13, AOCFileOrParseError> {
        Ok(Day13 {
            claw_machines: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day13 {
    fn calculate_prize_button_calls_day_a(
        &self,
        claw_machine: &ClawMachine,
        offset: &Point2D,
        limit: isize,
    ) -> Option<(isize, isize)> {
        let mut ret = None;
        let target = claw_machine.2 + *offset;
        for i in 0..=limit {
            let difference = target + (&claw_machine.0 * -i);
            if difference.x / claw_machine.1.x == difference.y / claw_machine.1.y
                && difference.x % claw_machine.1.x == 0
                && difference.y % claw_machine.1.y == 0
            {
                let other = difference.x / claw_machine.1.x;
                if !(0..=limit).contains(&other) {
                    continue;
                }
                if let Some((a, b)) = ret {
                    if 3 * i + other < 3 * a + b {
                        ret = Some((i, other));
                    }
                } else {
                    ret = Some((i, other));
                }
            }
        }
        ret
    }

    fn gaussian_elimination(&self, mut matrix: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        for pivot in 0..2 {
            for point in (pivot..matrix[pivot].len()).rev() {
                matrix[pivot][point] /= matrix[pivot][pivot];
            }
            for other in 0..2 {
                if other != pivot {
                    for point in (pivot..matrix[other].len()).rev() {
                        matrix[other][point] -= matrix[pivot][point] * matrix[other][pivot]
                    }
                }
            }
        }
        matrix
    }

    fn calculate_prize_button_calls_day_b(
        &self,
        claw_machine: &ClawMachine,
        offset: &Point2D,
        limit: Option<isize>,
    ) -> Option<(isize, isize)> {
        // ideally here we want to consider the formula,
        // x_a * a + x_b * b == x_c
        // &&
        // y_a * a + y_b * b == y_c
        //
        // and minimise for 3*a + b == answer
        //
        // so 2 formula and 2 unknowns, a and b
        // gaussian elimination:
        // [
        //   x_a   x_b  | x_c
        //   y_a   y_b  | y_c
        // ] and minimisation is a red herring
        let x_a = claw_machine.0.x as f64;
        let x_b = claw_machine.1.x as f64;
        let x_c = (claw_machine.2 + *offset).x as f64;
        let y_a = claw_machine.0.y as f64;
        let y_b = claw_machine.1.y as f64;
        let y_c = (claw_machine.2 + *offset).y as f64;
        let matrix = self.gaussian_elimination(vec![vec![x_a, x_b, x_c], vec![y_a, y_b, y_c]]);
        let a = matrix[0][2];
        let b = matrix[1][2];
        if (a.round() - a).abs() < 0.01
            && (b.round() - b).abs() < 0.01
            && a >= 0.
            && b >= 0.
            && (limit.is_none()
                || limit.is_some()
                    && a.round() as isize <= limit.unwrap()
                    && b.round() as isize <= limit.unwrap())
        {
            Some((a.round() as isize, b.round() as isize))
        } else {
            None
        }
    }

    fn calculate_day_a(&self) -> isize {
        self.claw_machines
            .iter()
            .map(|claw_machine| {
                let ret1 = self.calculate_prize_button_calls_day_a(
                    claw_machine,
                    &Point2D::from_usize(0, 0),
                    100,
                );
                let ret2 = self.calculate_prize_button_calls_day_b(
                    claw_machine,
                    &Point2D::from_usize(0, 0),
                    Some(100),
                );
                if ret1 != ret2 {
                    // sanity check the new method vs the old
                    panic!("Invalid result, {:?}, {:?}", ret1, ret2);
                }
                ret1
            })
            .map(|prize_calls| match prize_calls {
                Some((a, b)) => 3 * a + b,
                _ => 0,
            })
            .sum()
    }

    fn calculate_day_b(&self) -> isize {
        self.claw_machines
            .iter()
            .map(|claw_machine| {
                self.calculate_prize_button_calls_day_b(
                    claw_machine,
                    &Point2D::from_usize(10000000000000, 10000000000000),
                    None,
                )
            })
            .map(|prize_calls| match prize_calls {
                Some((a, b)) => 3 * a + b,
                _ => 0,
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day13 = Day13::new("data/test_data.txt").unwrap();
        let expected = 480;
        let actual = day13.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day13 = Day13::new("data/test_data.txt").unwrap();
        let expected = 875318608908;
        let actual = day13.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day13 = Day13::new("data/input_data.txt").unwrap();
        let expected = 40369;
        let actual = day13.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day13 = Day13::new("data/input_data.txt").unwrap();
        let expected = 72587986598368;
        let actual = day13.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
