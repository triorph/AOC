extern crate peg;
fn main() {
    let lines: Vec<&str> = include_str!("../input_data.txt").lines().collect();
    let day_a = calculate_day_a(&lines[..]);
    println!("Day a result: {}", day_a);
    let day_b = calculate_day_b(&lines[..]);
    println!("Day b result: {}", day_b);
}
fn find_most_common_numbers_at_index_matching_char(
    char_lines: &[Vec<char>],
    bit_index: usize,
    char_to_match: char,
) -> (usize, usize) {
    let num_matching: usize = char_lines
        .iter()
        .map(|line| (line[bit_index] == char_to_match) as usize)
        .sum();
    let num_not_matching: usize = char_lines
        .iter()
        .map(|line| (line[bit_index] != char_to_match) as usize)
        .sum();
    (num_not_matching, num_matching)
}

fn calculate_day_a(lines: &[&str]) -> usize {
    let mut gamma = 0; // most common bits
    let mut epsilon = 0; // least common bits
    let char_lines: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    for bit_index in 0..char_lines[0].len() {
        gamma *= 2;
        epsilon *= 2;
        let (num_0, num_1) =
            find_most_common_numbers_at_index_matching_char(&char_lines, bit_index, '1');
        if num_1 > num_0 {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }
    epsilon * gamma
}

fn filter_by_char_at_bit(
    char_lines: Vec<Vec<char>>,
    bit_index: usize,
    char_to_match: char,
) -> Vec<Vec<char>> {
    if char_lines.len() > 1 {
        char_lines
            .into_iter()
            .filter(|line| line[bit_index] == char_to_match)
            .collect()
    } else {
        char_lines
    }
}

fn filter_down_most_common(lines: &[&str], oxygen: bool) -> usize {
    let mut char_lines: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let mut ret: usize = 0;
    let (char_to_match, char_to_not_match, match_increase, not_match_increase) = match oxygen {
        true => ('1', '0', 1, 0),
        false => ('0', '1', 0, 1),
    };
    for bit_index in 0..char_lines[0].len() {
        ret *= 2;
        if char_lines.len() > 1 {
            let (num_not_match, num_match) =
                find_most_common_numbers_at_index_matching_char(&char_lines, bit_index, '1');
            if num_match >= num_not_match {
                ret += match_increase;
                char_lines = filter_by_char_at_bit(char_lines, bit_index, char_to_match);
            } else {
                ret += not_match_increase;
                char_lines = filter_by_char_at_bit(char_lines, bit_index, char_to_not_match);
            }
        } else if char_lines[0][bit_index] == '1' {
            ret += 1;
        }
    }
    ret
}

fn calculate_day_b(lines: &[&str]) -> usize {
    let oxygen_rating = filter_down_most_common(lines, true);
    let co2_scrubber_rating = filter_down_most_common(lines, false);
    oxygen_rating * co2_scrubber_rating
}

#[cfg(test)]
mod test {
    use crate::calculate_day_a;
    use crate::calculate_day_b;
    use crate::filter_down_most_common;
    #[test]
    fn test_day_a() {
        let lines: Vec<&str> = include_str!("../test_data.txt").lines().collect();
        assert_eq!(calculate_day_a(&lines[..]), 198);
    }

    #[test]
    fn test_oxygen_rating() {
        let lines: Vec<&str> = include_str!("../test_data.txt").lines().collect();
        assert_eq!(filter_down_most_common(&lines[..], true), 23);
    }

    #[test]
    fn test_co2_scrubber_rating() {
        let lines: Vec<&str> = include_str!("../test_data.txt").lines().collect();
        assert_eq!(filter_down_most_common(&lines[..], false), 10);
    }

    #[test]
    fn test_day_b() {
        let lines: Vec<&str> = include_str!("../test_data.txt").lines().collect();
        assert_eq!(calculate_day_b(&lines[..]), 230);
    }
}

peg::parser! { grammar binary_parser() for str {
    rule binary() -> String
        = n:$(['0'..='1']+) {String::from(n)}

    pub rule parse() -> String
        = binary:binary() { binary }
}
}
