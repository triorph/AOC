extern crate peg;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day1_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']) {(n.parse().expect(&format!("Was expecting a number string {}", n)[..]))}
    rule one_word() ->usize
        = "one" { 1 }
    rule two_word() ->usize
        = "two" { 2 }
    rule three_word() ->usize
        = "three" { 3 }
    rule four_word() -> usize
        = "four" { 4 }
    rule five_word() -> usize
        = "five" { 5 }
    rule six_word() -> usize
        = "six" { 6 }
    rule seven_word() -> usize
        = "seven" { 7 }
    rule eight_word() -> usize
        = "eight" { 8 }
    rule nine_word() -> usize
        = "nine" { 9 }
    rule number_word() -> usize
        = n:(one_word() / two_word() / three_word() / four_word() / five_word() / six_word() / seven_word() / eight_word() / nine_word()) {
            n
        }
    rule number_word_consume_1_letter() -> usize
        = n:(&number_word()) line_a_delim() { n }
    rule number_or_number_word() -> usize
        = n:(number() / number_word_consume_1_letter()) { n }
    rule line_a_delim() -> ()
        = ['a'..='z'] {}
    rule line_b_delim() -> ()
        = !number_or_number_word() line_a_delim() {}
    rule line_a() -> Vec<usize>
        = line_a_delim()* line:number() ** (line_a_delim()*) line_a_delim()* { line }
    rule line_b() -> Vec<usize>
        = line_b_delim()* line:number_or_number_word() ** (line_b_delim()*) line_b_delim()* { line }

    pub rule parse_a() -> Vec<Vec<usize>>
        = lines:line_a() ++ ("\n" +) "\n" * {
             lines.into_iter().filter(|line| !line.is_empty()).collect()
        }

    pub rule parse_b() -> Vec<Vec<usize>>
        = lines:line_b() ++ ("\n" +) "\n" * {
             lines.into_iter().filter(|line| !line.is_empty()).collect()
        }
}}

pub fn parse_data_a(input: &str) -> Result<Vec<Vec<usize>>, AOCFileOrParseError> {
    if let Ok(ret) = day1_parser::parse_a(input) {
        Ok(ret)
    } else {
        Err(AOCFileOrParseError)
    }
}

pub fn parse_data_b(input: &str) -> Result<Vec<Vec<usize>>, AOCFileOrParseError> {
    if let Ok(ret) = day1_parser::parse_b(input) {
        Ok(ret)
    } else {
        Err(AOCFileOrParseError)
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    use aoc_helpers::read_input_file;
    #[test]
    fn test_parse_a() {
        let input_str = read_input_file("data/test_data_a.txt").unwrap();
        let actual = day1_parser::parse_a(&input_str).expect("Should parse successfully");
        let expected: Vec<Vec<usize>> = vec![vec![1, 2], vec![3, 8], vec![1, 2, 3, 4, 5], vec![7]];
        assert_eq!(expected, actual)
    }

    #[test]
    fn test_parse_b() {
        let input_str = read_input_file("data/test_data_b.txt").unwrap();
        let actual = day1_parser::parse_b(&input_str).expect("Should parse successfully");
        let expected: Vec<Vec<usize>> = vec![
            vec![2, 1, 9],
            vec![8, 2, 3],
            vec![1, 2, 3],
            vec![2, 1, 3, 4],
            vec![4, 9, 8, 7, 2],
            vec![1, 8, 2, 3, 4],
            vec![7, 6],
        ];
        assert_eq!(expected, actual)
    }

    #[test]
    fn test_parse_a_with_data_from_b() {
        let input_str = read_input_file("data/test_data_b.txt").unwrap();
        let actual = day1_parser::parse_a(&input_str).expect("Should parse successfully");
        let expected: Vec<Vec<usize>> = vec![
            vec![1],
            vec![2],
            vec![3],
            vec![4, 2],
            vec![2, 3, 4],
            vec![7],
        ];
        assert_eq!(expected, actual)
    }
}
