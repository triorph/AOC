extern crate peg;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day19_parser() for str {
    rule stripe() -> char
        = stripe:(['a'..='z']) { stripe }
    rule towel() -> String
        = towel:stripe() ++ "" { towel.into_iter().collect() }
    rule towel_patterns() -> Vec<String>
        = towel_patterns:towel() ++ ", " { towel_patterns }
    rule desired_designs() -> Vec<String>
        = desired_designs:towel() ++ "\n" { desired_designs }
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    pub rule parse() -> (Vec<String>, Vec<String>)
        = towel_patterns:towel_patterns() "\n"+ desired_designs:desired_designs() "\n"* { (towel_patterns, desired_designs) }
}}

pub fn parse_data(input: &str) -> Result<(Vec<String>, Vec<String>), AOCFileOrParseError> {
    if let Ok(ret) = day19_parser::parse(input) {
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
        let actual = day19_parser::parse(&input_str).expect("Should parse successfully");
        let expected = (
            vec![
                String::from("r"),
                String::from("wr"),
                String::from("b"),
                String::from("g"),
                String::from("bwu"),
                String::from("rb"),
                String::from("gb"),
                String::from("br"),
            ],
            vec![
                String::from("brwrr"),
                String::from("bggr"),
                String::from("gbbr"),
                String::from("rrbgbr"),
                String::from("ubwu"),
                String::from("bwurrg"),
                String::from("brgr"),
                String::from("bbrgwb"),
            ],
        );
        assert_eq!(expected, actual)
    }
}
