extern crate peg;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day01_parser() for str {
    rule number() -> isize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule left_dial() -> isize
        = "L" n:number() { - n }
    rule right_dial() -> isize
        = "R" n:number() { n }
    rule dial() -> isize
        = dial:(left_dial() / right_dial()) { dial }
    pub rule parse() -> Vec<isize>
        = lines:dial() ++ ("\n" +) "\n" * { lines }
}}

pub fn parse_data(input: &str) -> Result<Vec<isize>, AOCFileOrParseError> {
    if let Ok(ret) = day01_parser::parse(input) {
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
        let actual = day01_parser::parse(&input_str).expect("Should parse successfully");
        let expected: Vec<isize> = vec![-68, -30, 48, -5, 60, -55, -1, -99, 14, -82];
        assert_eq!(expected, actual)
    }
}
