extern crate peg;
use crate::droplet::Droplet;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day18_parser() for str {
    rule number() -> isize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule droplet() -> Droplet
        = x:number() "," y:number() "," z:number() { Droplet {x, y, z} }
    pub rule parse() -> Vec<Droplet>
        = droplets:droplet() ++ ("\n") "\n" * {
             { droplets }
        }
}}

pub fn parse_data(input: &str) -> Result<Vec<Droplet>, AOCFileOrParseError> {
    if let Ok(ret) = day18_parser::parse(input) {
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
        let actual = day18_parser::parse(&input_str).expect("Should parse successfully");
        assert_eq!(actual.len(), 13);
    }
}
