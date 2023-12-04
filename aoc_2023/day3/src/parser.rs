extern crate peg;
use crate::schematic::{Schematic, SchematicValue};
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day3_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule schematic_number() -> SchematicValue
        = n:number() { SchematicValue::Digit(n) }
    rule schematic_blank() -> SchematicValue
        = "." { SchematicValue::Blank }
    rule schematic_gear_symbol() -> SchematicValue
        = "*" { SchematicValue::GearSymbol }
    rule schematic_symbol() -> SchematicValue
        = $([^'0'..='9'|'.'|'\n'|'*']) { SchematicValue::Symbol }
    rule schematic_value() -> SchematicValue
        = schematic_value:(schematic_number() / schematic_blank() / schematic_gear_symbol() / schematic_symbol()) { schematic_value }
    rule line() -> Vec<SchematicValue>
        = line:schematic_value() ++ "" { line }
    pub rule parse() -> Schematic
        = lines:line() ++ ("\n" +) "\n" * {
             Schematic::new(lines)
        }
}}

pub fn parse_data(input: &str) -> Result<Schematic, AOCFileOrParseError> {
    if let Ok(ret) = day3_parser::parse(input) {
        Ok(ret)
    } else {
        Err(AOCFileOrParseError)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use aoc_helpers::read_input_file;
    #[test]
    fn test_parse() {
        let input_str = read_input_file("data/test_data.txt").unwrap();
        day3_parser::parse(&input_str).expect("Should parse successfully");
    }
}
