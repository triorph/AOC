extern crate peg;
use aoc_helpers::AOCFileOrParseError;

pub type Line = (Vec<Option<bool>>, Vec<usize>);

peg::parser! { pub grammar day12_parser() for str {
    rule unknown() -> Option<bool>
        = "?" { None }
    rule empty() -> Option<bool>
        = "." { Some(false) }
    rule filled() -> Option<bool>
        = "#" { Some(true) }
    rule place() -> Option<bool>
        = place: (unknown() / empty() / filled()) { place }
    rule places() -> Vec<Option<bool>>
        = places:place() ++ ""
    rule number() -> usize
        = n:$(['0'..='9']+) {(n.parse().expect(&format!("Was expecting a number string {}", n)[..]))}
    rule arrangement() -> Vec<usize>
        = arrangement:number() ++ ","
    rule line() -> (Vec<Option<bool>>, Vec<usize>)
        = places:places()  " "+ arrangement:arrangement() { (places, arrangement) }
    pub rule parse() -> Vec<(Vec<Option<bool>>, Vec<usize>)>
        = lines:line() ++ ("\n" +) "\n" * { lines }
}}

pub fn parse_data(input: &str) -> Result<Vec<Line>, AOCFileOrParseError> {
    if let Ok(ret) = day12_parser::parse(input) {
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
        let actual = day12_parser::parse(&input_str).expect("Should parse successfully");
        assert_eq!(6, actual.len())
    }
}
