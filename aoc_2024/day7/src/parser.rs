extern crate peg;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day7_parser() for str {
    rule number() -> isize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule line() -> (isize, Vec<isize>)
        = left:number() ":"  " "* right:number() ++ (" " +) " " * { (left, right) }
    pub rule parse() -> Vec<(isize, Vec<isize>)>
        = lines:line() ++ ("\n"+) "\n"* { lines }
}}

pub fn parse_data(input: &str) -> Result<Vec<(isize, Vec<isize>)>, AOCFileOrParseError> {
    if let Ok(ret) = day7_parser::parse(input) {
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
        let actual = day7_parser::parse(&input_str).expect("Should parse successfully");
        let expected: Vec<(isize, Vec<isize>)> = vec![
            (190, vec![10, 19]),
            (3267, vec![81, 40, 27]),
            (83, vec![17, 5]),
            (156, vec![15, 6]),
            (7290, vec![6, 8, 6, 15]),
            (161011, vec![16, 10, 13]),
            (192, vec![17, 8, 14]),
            (21037, vec![9, 7, 18, 13]),
            (292, vec![11, 6, 16, 20]),
        ];
        assert_eq!(expected, actual)
    }
}
