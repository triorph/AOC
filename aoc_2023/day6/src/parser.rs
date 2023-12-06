extern crate peg;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day6_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule times() -> Vec<usize>
        = "Time:"  " "* times:number() ++ (" "+)  " "* { times }
    rule distances() -> Vec<usize>
        = "Distance:"  " "* distances:number() ++ (" "+)  " "* { distances }
    pub rule parse() -> Vec<(usize, usize)>
        = times:times() "\n"* distances:distances() "\n"* {
            times.into_iter().zip(distances.into_iter()).collect()
        }
}}

pub fn parse_data(input: &str) -> Result<Vec<(usize, usize)>, AOCFileOrParseError> {
    if let Ok(ret) = day6_parser::parse(input) {
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
        let actual = day6_parser::parse(&input_str).expect("Should parse successfully");
        let expected: Vec<(usize, usize)> = vec![(7, 9), (15, 40), (30, 200)];
        assert_eq!(expected, actual)
    }
}
