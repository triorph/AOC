mod parser;
use crate::parser::parse_data;
use aoc_helpers::{hashset_from_vec, read_input_file, AOCCalculator, AOCFileOrParseError};

pub struct Day6 {
    data: String,
}

impl AOCCalculator<usize> for Day6 {
    fn new(filename: &str) -> Result<Day6, AOCFileOrParseError> {
        Ok(Day6 {
            data: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn calculate_day_a(&self) -> usize {
        find_when_n_different(&self.data, 4).expect("Has an answer")
    }

    fn calculate_day_b(&self) -> usize {
        find_when_n_different(&self.data, 14).expect("Has an answer")
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

fn find_when_n_different(data: &str, n: usize) -> Result<usize, ()> {
    let chars = data.chars().collect::<Vec<char>>();
    for i in 0..(chars.len() - n) {
        if test_all_different(&chars[i..i + n]) {
            return Ok(i + n);
        }
    }
    Err(())
}

fn test_all_different(vals: &[char]) -> bool {
    hashset_from_vec(vals).len() == vals.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day6 = Day6::new("data/test_data.txt").unwrap();
        let expected = 7;
        let actual = day6.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn further_day_a_examples() {
        for (data, expected) in [
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ]
        .into_iter()
        {
            assert_eq!(
                Day6 {
                    data: data.to_string()
                }
                .calculate_day_a(),
                expected
            );
        }
    }

    #[test]
    fn test_calculate_day_b() {
        let day6 = Day6::new("data/test_data.txt").unwrap();
        let expected = 19;
        let actual = day6.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn further_day_b_examples() {
        for (data, expected) in [
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ]
        .into_iter()
        {
            assert_eq!(
                Day6 {
                    data: data.to_string()
                }
                .calculate_day_b(),
                expected
            );
        }
    }
}
