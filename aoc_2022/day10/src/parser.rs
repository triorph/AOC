extern crate peg;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day10_parser() for str {
    rule positive_number() -> isize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule negative_number() -> isize
        = "-" n:positive_number() { - n }
    rule number() -> isize
        = n:(positive_number() / negative_number()) { n }
    rule noop() -> Option<isize>
        = "noop" { None }
    rule addx() -> Option<isize>
        = "addx " n:number() { Some(n) }
    rule instruction() -> Option<isize>
        = instruction:(noop() / addx()) { instruction }
    pub rule parse() -> Vec<Option<isize>>
        = line_of_instructions:instruction() ++ ("\n" +) "\n" * {
             { line_of_instructions }
        }
}}

pub fn parse_data(input: &str) -> Result<Vec<Option<isize>>, AOCFileOrParseError> {
    if let Ok(ret) = day10_parser::parse(input) {
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
        let actual = day10_parser::parse(&input_str).expect("Should parse successfully");
        let expected: Vec<Option<isize>> = vec![
            Some(15),
            Some(-11),
            Some(6),
            Some(-3),
            Some(5),
            Some(-1),
            Some(-8),
            Some(13),
            Some(4),
            None,
            Some(-1),
            Some(5),
            Some(-1),
            Some(5),
            Some(-1),
            Some(5),
            Some(-1),
            Some(5),
            Some(-1),
            Some(-35),
            Some(1),
            Some(24),
            Some(-19),
            Some(1),
            Some(16),
            Some(-11),
            None,
            None,
            Some(21),
            Some(-15),
            None,
            None,
            Some(-3),
            Some(9),
            Some(1),
            Some(-3),
            Some(8),
            Some(1),
            Some(5),
            None,
            None,
            None,
            None,
            None,
            Some(-36),
            None,
            Some(1),
            Some(7),
            None,
            None,
            None,
            Some(2),
            Some(6),
            None,
            None,
            None,
            None,
            None,
            Some(1),
            None,
            None,
            Some(7),
            Some(1),
            None,
            Some(-13),
            Some(13),
            Some(7),
            None,
            Some(1),
            Some(-33),
            None,
            None,
            None,
            Some(2),
            None,
            None,
            None,
            Some(8),
            None,
            Some(-1),
            Some(2),
            Some(1),
            None,
            Some(17),
            Some(-9),
            Some(1),
            Some(1),
            Some(-3),
            Some(11),
            None,
            None,
            Some(1),
            None,
            Some(1),
            None,
            None,
            Some(-13),
            Some(-19),
            Some(1),
            Some(3),
            Some(26),
            Some(-30),
            Some(12),
            Some(-1),
            Some(3),
            Some(1),
            None,
            None,
            None,
            Some(-9),
            Some(18),
            Some(1),
            Some(2),
            None,
            None,
            Some(9),
            None,
            None,
            None,
            Some(-1),
            Some(2),
            Some(-37),
            Some(1),
            Some(3),
            None,
            Some(15),
            Some(-21),
            Some(22),
            Some(-6),
            Some(1),
            None,
            Some(2),
            Some(1),
            None,
            Some(-10),
            None,
            None,
            Some(20),
            Some(1),
            Some(2),
            Some(2),
            Some(-6),
            Some(-11),
            None,
            None,
            None,
        ];
        assert_eq!(expected, actual)
    }
}
