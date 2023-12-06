extern crate peg;
use crate::converter::{Converter, ConverterMap};
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day5_parser() for str {
    rule number() -> u64
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule seeds() -> Vec<u64>
        = "seeds:" " "* seeds:number() ++ (" "+) " "* { seeds }
    rule name() -> &'input str
        = name:$(['a'..='z'|'A'..='Z']+) { name }
    rule converter() -> Converter
        = input_start:number() " "+  output_start:number() " "+ range:number() {
            Converter::new(input_start, output_start, range)
    }
    rule converter_map() -> ConverterMap
        = input: name() "-to-" output:name() " map:\n" converters:converter() ++ ("\n" +) {
            ConverterMap::new( input, output, converters )
        }
    pub rule parse() -> (Vec<u64>, Vec<ConverterMap>)
        = seeds:seeds() "\n"+ converter_maps:converter_map() ++ ("\n"+) "\n" * {
            (seeds, converter_maps)
        }
}}

pub fn parse_data(input: &str) -> Result<(Vec<u64>, Vec<ConverterMap>), AOCFileOrParseError> {
    if let Ok(ret) = day5_parser::parse(input) {
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
        let (seeds, converter_maps) =
            day5_parser::parse(&input_str).expect("Should parse successfully");
        assert_eq!(seeds, vec![79, 14, 55, 13]);
        assert_eq!(converter_maps.len(), 7);
    }
}
