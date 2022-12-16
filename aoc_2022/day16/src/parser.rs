extern crate peg;
use crate::tunnel::{Tunnel, Tunnels};
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day16_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule tunnel_name() -> String
        = name:$(['A'..='Z']+) { name.to_string() }
    rule tunnel() -> Tunnel
        = "Valve " tunnel_name:tunnel_name() " has flow rate="
            rate:number() "; tunnel" ("s"?) " lead" ("s"?)
            " to valve" ("s"?) (" ") neighbours:tunnel_name() ++ ", " {
                Tunnel::new ( tunnel_name, rate, neighbours )
            }
    pub rule parse() -> Tunnels
        = tunnels:tunnel() ++ ("\n") "\n" * {
            Tunnels::new(tunnels)
        }
}}

pub fn parse_data(input: &str) -> Result<Tunnels, AOCFileOrParseError> {
    if let Ok(ret) = day16_parser::parse(input) {
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
        let actual = day16_parser::parse(&input_str).expect("Should parse successfully");
        // assert_eq!(actual.len(), 10);
    }
}
