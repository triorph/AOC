extern crate peg;
use aoc_helpers::AOCFileOrParseError;
use std::ops::RangeInclusive;

pub type IngredientRange = RangeInclusive<usize>;

peg::parser! { pub grammar day5_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule range() -> IngredientRange
        = start:number() "-" end:number() { start..=end }
    rule ranges() -> Vec<IngredientRange>
        = ranges:range() ++ ("\n" +) { ranges }
    rule numbers() -> Vec<usize>
        = lines:number() ++ ("\n" +) { lines }
    pub rule parse() -> (Vec<IngredientRange>, Vec<usize>)
        = ranges:ranges() "\n" * numbers:numbers() "\n" * { (ranges, numbers) }
}}

pub fn parse_data(input: &str) -> Result<(Vec<IngredientRange>, Vec<usize>), AOCFileOrParseError> {
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
        let (actual_ranges, actual_numbers) =
            day5_parser::parse(&input_str).expect("Should parse successfully");
        let expected_ranges: Vec<IngredientRange> = vec![3..=5, 10..=14, 16..=20, 12..=18];
        let expected_numbers: Vec<usize> = vec![1, 5, 8, 11, 17, 32];
        assert_eq!(expected_ranges, actual_ranges);
        assert_eq!(expected_numbers, actual_numbers);
    }
}
