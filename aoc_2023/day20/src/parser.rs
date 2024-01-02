extern crate peg;
use crate::pulse_module::PulseModule;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day20_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule word() -> String
        = word:$(['a'..='z']+) { word.to_string() }
    rule flip_flop() -> PulseModule
        = "%" input:word() " -> " outputs:word() ++ ", " { PulseModule::new_flip_flop(input, outputs) }
    rule conjunction() -> PulseModule
        = "&" input:word() " -> " outputs:word() ++ ", " { PulseModule::new_conjunction(input, outputs) }
    rule broadcaster() -> PulseModule
        = "broadcaster -> " outputs:word() ++ ", " { PulseModule::new_broadcaster(outputs) }
    rule pulse_module() -> PulseModule
        = pulse_module: (broadcaster() / flip_flop() / conjunction())
    pub rule parse() -> Vec<PulseModule>
        = lines:pulse_module() ++ ("\n" +) "\n" * { lines }
}}

pub fn parse_data(input: &str) -> Result<Vec<PulseModule>, AOCFileOrParseError> {
    if let Ok(ret) = day20_parser::parse(input) {
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
        let actual = day20_parser::parse(&input_str).expect("Should parse successfully");
        let expected: Vec<PulseModule> = vec![
            PulseModule::new_broadcaster(vec!["a".to_string(), "b".to_string(), "c".to_string()]),
            PulseModule::new_flip_flop("a".to_string(), vec!["b".to_string()]),
            PulseModule::new_flip_flop("b".to_string(), vec!["c".to_string()]),
            PulseModule::new_flip_flop("c".to_string(), vec!["inv".to_string()]),
            PulseModule::new_conjunction("inv".to_string(), vec!["a".to_string()]),
        ];
        assert_eq!(expected, actual)
    }
}
