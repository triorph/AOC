extern crate peg;
use crate::elf::Elf;
use aoc_helpers::AOCFileOrParseError;
use std::collections::HashSet;

peg::parser! { pub grammar day23_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule elf() -> bool
        =  "#" { true }
    rule floor() -> bool
        = "." { false }
    rule is_elf() -> bool
        = elf:(elf() / floor()) { elf }
    rule line() -> Vec<bool>
        = line:is_elf() ++ "" (" "*) { line }
    pub rule parse() -> HashSet<Elf>
        = lines:line() ++ "\n" ("\n"*) {
            let mut ret = HashSet::new();
            for (y,line) in lines.into_iter().enumerate() {
                for (x, is_elf) in line.into_iter().enumerate() {
                    if is_elf {
                        ret.insert(Elf::new(x, y));
                    }
                }
            }
            ret
        }
}}

pub fn parse_data(input: &str) -> Result<HashSet<Elf>, AOCFileOrParseError> {
    if let Ok(ret) = day23_parser::parse(input) {
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
        let actual = day23_parser::parse(&input_str).expect("Should parse successfully");
        assert_eq!(actual.len(), 22);
    }
}
