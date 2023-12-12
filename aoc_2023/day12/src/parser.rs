extern crate peg;
use crate::hot_spring::{ConditionReport, HotSpringCondition};
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day12_parser() for str {
    rule unknown() -> HotSpringCondition
        = "?" { HotSpringCondition::Unknown }
    rule undamaged() -> HotSpringCondition
        = "." { HotSpringCondition::Undamaged }
    rule damaged() -> HotSpringCondition
        = "#" { HotSpringCondition::Damaged }
    rule hot_spring_condition() -> HotSpringCondition
        = condition: (unknown() / undamaged() / damaged()) { condition }
    rule hot_spring_conditions() -> Vec<HotSpringCondition>
        = conditions:hot_spring_condition() ++ "" { conditions }
    rule number() -> usize
        = n:$(['0'..='9']+) {(n.parse().expect(&format!("Was expecting a number string {}", n)[..]))}
    rule criteria() -> Vec<usize>
        = criteria:number() ++ ","
    rule condition_report() -> ConditionReport
        = conditions:hot_spring_conditions()  " "+ criteria:criteria() {  ConditionReport::new(conditions, criteria) }
    pub rule parse() -> Vec<ConditionReport>
        = condition_reports:condition_report() ++ ("\n" +) "\n" * { condition_reports }
}}

pub fn parse_data(input: &str) -> Result<Vec<ConditionReport>, AOCFileOrParseError> {
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
