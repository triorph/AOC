mod parser;
mod schematic;
use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};
use schematic::Schematic;

pub struct Day3 {
    data: Schematic,
}

impl AOCCalculator for Day3 {
    fn new(filename: &str) -> Result<Day3, AOCFileOrParseError> {
        Ok(Day3 {
            data: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day3 {
    fn calculate_day_a(&self) -> usize {
        self.data
            .numbers
            .iter()
            .filter(|number| {
                number
                    .get_neighbours()
                    .intersection(&self.data.symbols)
                    .count()
                    > 0
            })
            .map(|number| number.number)
            .sum()
    }

    fn calculate_day_b(&self) -> usize {
        self.data
            .gear_symbols
            .iter()
            .map(|gear_symbol| self.data.get_number_neighbours_to_point(gear_symbol))
            .filter(|x| x.len() == 2)
            .map(|x| x.iter().product::<usize>())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day3 = Day3::new("data/test_data.txt").unwrap();
        let expected = 4361;
        let actual = day3.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day3 = Day3::new("data/test_data.txt").unwrap();
        let expected = 467835;
        let actual = day3.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day3 = Day3::new("data/input_data.txt").unwrap();
        let expected = 540025;
        let actual = day3.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day3 = Day3::new("data/input_data.txt").unwrap();
        let expected = 84584891;
        let actual = day3.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
