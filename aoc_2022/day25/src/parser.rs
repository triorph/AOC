extern crate peg;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day25_parser() for str {
    rule positive_digit() -> isize
        = n:$(['0'..='2']) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule negative_one() -> isize
        = "-" { -1 }
    rule negative_two() -> isize
        = "=" { -2 }
    rule digit() -> isize
        = n:(positive_digit() / negative_one() / negative_two()) { n }
    rule snafu_number() -> Vec<isize>
        = digits:digit() ++ "" " "* { digits }
    pub rule parse() -> Vec<Vec<isize>>
        = snafu_numbers:snafu_number() ++ ("\n") "\n" * {
             snafu_numbers
        }
}}

pub fn parse_data(input: &str) -> Result<Vec<Vec<isize>>, AOCFileOrParseError> {
    if let Ok(ret) = day25_parser::parse(input) {
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
    fn test_parse() {
        let input_str = read_input_file("data/test_data.txt").unwrap();
        let actual = day25_parser::parse(&input_str).expect("Should parse successfully");
        let expected: Vec<Vec<isize>> = vec![
            vec![1, -2, -1, 0, -1, 2],
            vec![1, 2, 1, 1, 1],
            vec![2, -2, 0, -2],
            vec![2, 1],
            vec![2, -2, 0, 1],
            vec![1, 1, 1],
            vec![2, 0, 0, 1, 2],
            vec![1, 1, 2],
            vec![1, -2, -1, 1, -2],
            vec![1, -1, 1, 2],
            vec![1, 2],
            vec![1, -2],
            vec![1, 2, 2],
        ];
        assert_eq!(expected, actual)
    }
}
