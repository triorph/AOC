extern crate peg;
use num::BigUint;

#[derive(PartialEq, Debug, Clone)]
pub enum BinaryValue {
    One,
    Zero,
}

#[derive(Clone)]
struct DiagnosticReport {
    binary_values: Vec<Vec<BinaryValue>>,
}

peg::parser! { grammar binary_parser() for str {
    rule binary() -> BinaryValue
        = n:(one() / zero()) { n }
    rule one() -> BinaryValue
        = "1" { BinaryValue::One }
    rule zero() -> BinaryValue
        = "0" { BinaryValue::Zero }
    pub rule parse() -> Vec<BinaryValue>
        = binary:binary() ** "" { binary }

}
}

impl DiagnosticReport {
    fn new(lines: std::str::Lines) -> DiagnosticReport {
        let mut binary_values = Vec::<Vec<BinaryValue>>::new();
        let mut width: Option<usize> = None;
        for line in lines {
            if let Ok(binary_line) = binary_parser::parse(line) {
                if width.is_none() {
                    width = Some(binary_line.len());
                }
                if let Some(current_width) = width {
                    if current_width == binary_line.len() {
                        binary_values.push(binary_line);
                    } else {
                        println!("invalid line in diagnostic report data: {:?}", binary_line);
                    }
                }
            }
        }
        DiagnosticReport { binary_values }
    }
    fn count_bits_matching_value(
        self: &DiagnosticReport,
        bit_index: usize,
        value_to_match: BinaryValue,
    ) -> (usize, usize) {
        let num_matching: usize = self
            .binary_values
            .iter()
            .map(|line| (line[bit_index] == value_to_match) as usize)
            .sum();
        let num_not_matching: usize = self
            .binary_values
            .iter()
            .map(|line| (line[bit_index] != value_to_match) as usize)
            .sum();
        (num_not_matching, num_matching)
    }
    fn filter_to_bits_matching_value(
        self: &mut DiagnosticReport,
        bit_index: usize,
        value_to_match: &BinaryValue,
    ) {
        // Rewrites the objects binary_values, meaning this method consumes the object somewhat.
        if self.binary_values.len() > 1 {
            self.binary_values = self
                .binary_values
                .clone()
                .into_iter()
                .filter(|line| line[bit_index] == *value_to_match)
                .collect();
        };
    }

    fn filter_down_most_common(self: &mut DiagnosticReport, oxygen: bool) -> BigUint {
        // filter_to_bits_matching_value consumes the self value, so this method does too.
        // It is worth cloning the object before you call this method if you want to reuse it later.
        let mut ret: BigUint = BigUint::new(vec![0]);
        let (value_to_match, value_to_not_match, match_increase, not_match_increase) = match oxygen
        {
            true => (BinaryValue::One, BinaryValue::Zero, 1_usize, 0_usize),
            false => (BinaryValue::Zero, BinaryValue::One, 0_usize, 1_usize),
        };
        for bit_index in 0..self.binary_values[0].len() {
            ret *= 2_usize;
            if self.binary_values.len() > 1 {
                let (num_not_match, num_match) =
                    self.count_bits_matching_value(bit_index, BinaryValue::One);
                if num_match >= num_not_match {
                    ret += match_increase;
                    self.filter_to_bits_matching_value(bit_index, &value_to_match);
                } else {
                    ret += not_match_increase;
                    self.filter_to_bits_matching_value(bit_index, &value_to_not_match);
                }
            } else if self.binary_values[0][bit_index] == BinaryValue::One {
                ret += 1_usize;
            }
        }
        ret
    }
    fn calculate_day_a(self: &DiagnosticReport) -> BigUint {
        let mut gamma = BigUint::new(vec![0]); // most common bits
        let mut epsilon = BigUint::new(vec![0]); //least common bits
        for bit_index in 0..self.binary_values[0].len() {
            gamma *= 2_usize;
            epsilon *= 2_usize;
            let (num_0, num_1) = self.count_bits_matching_value(bit_index, BinaryValue::One);
            if num_1 > num_0 {
                gamma += 1_usize;
            } else {
                epsilon += 1_usize;
            }
        }
        epsilon * gamma
    }
    fn calculate_day_b(self: &DiagnosticReport) -> BigUint {
        // Since filter_down_most_common consumes the object, we want to clone it first.
        let oxygen_rating = self.clone().filter_down_most_common(true);
        let co2_scrubber_rating = self.clone().filter_down_most_common(false);
        oxygen_rating * co2_scrubber_rating
    }
}

fn main() {
    let report = DiagnosticReport::new(include_str!("../big_data.txt").lines());
    let day_a = report.calculate_day_a();
    println!("Day a result: {}", day_a);
    let day_b = report.calculate_day_b();
    println!("Day b result: {}", day_b);
}

#[cfg(test)]
mod test {
    use crate::DiagnosticReport;

    #[test]
    fn test_day_a() {
        let report = DiagnosticReport::new(include_str!("../test_data.txt").lines());
        assert_eq!(report.calculate_day_a(), 198);
    }

    #[test]
    fn test_oxygen_rating() {
        let mut report = DiagnosticReport::new(include_str!("../test_data.txt").lines());
        assert_eq!(report.filter_down_most_common(true), 23);
    }

    #[test]
    fn test_co2_scrubber_rating() {
        let mut report = DiagnosticReport::new(include_str!("../test_data.txt").lines());
        assert_eq!(report.filter_down_most_common(false), 10);
    }

    #[test]
    fn test_day_b() {
        let report = DiagnosticReport::new(include_str!("../test_data.txt").lines());
        assert_eq!(report.calculate_day_b(), 230);
    }
}
