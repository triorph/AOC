extern crate peg;
use crate::filter_chain::FilterChain;
use crate::filter_rule::{FilterRule, PartCategory};
use crate::part::Part;
use aoc_helpers::AOCFileOrParseError;
use std::collections::HashMap;

peg::parser! { pub grammar day19_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule word() -> String
        = word:$(['a'..='z'|'A'..='Z']*) { word.to_string() }
    rule x() -> PartCategory
        = "x" { PartCategory::X }
    rule m() -> PartCategory
        = "m" { PartCategory::M }
    rule a() -> PartCategory
        = "a" { PartCategory::A }
    rule s() -> PartCategory
        = "s" { PartCategory::S }
    rule part_category() -> PartCategory
        = part_category:(x() / m() / a() / s()) { part_category }
    rule less_than() -> FilterRule
        = part_category:part_category() "<" number:number() ":" target:word() {
            FilterRule::LessThan(part_category, number, target)
        }
    rule greater_than() -> FilterRule
        = part_category:part_category() ">" number:number() ":" target:word() {
            FilterRule::GreaterThan(part_category, number, target)
        }
    rule target() -> FilterRule
        = target:word() { FilterRule::Target(target) }
    rule filter_rule() -> FilterRule
        = filter_rule:(less_than() / greater_than() / target())
    rule named_rule_chain() -> (String, FilterChain)
        = name:word() "{" rules:filter_rule() ++ "," "}" { (name, FilterChain { chain: rules }) }
    rule all_rule_chains() -> HashMap<String, FilterChain>
        = all_rule_chains:named_rule_chain() ++ "\n" {
            let mut ret = HashMap::new();
            for (name, filter_rule) in all_rule_chains.into_iter() {
                ret.insert(name, filter_rule);
            }
            ret
        }
    rule part_object() -> Part
        = "{x=" x:number() ",m=" m:number() ",a=" a:number() ",s=" s:number() "}" { Part {x, m, a, s} }
    rule all_parts() -> Vec<Part>
        = all_parts:part_object() ++ "\n" { all_parts }
    pub rule parse() -> (HashMap<String, FilterChain>, Vec<Part>)
        = all_rule_chains:all_rule_chains() "\n" * all_parts:all_parts() "\n" * { (all_rule_chains, all_parts) }
}}

pub fn parse_data(
    input: &str,
) -> Result<(HashMap<String, FilterChain>, Vec<Part>), AOCFileOrParseError> {
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
        let (_actual_chains, actual_parts) =
            day19_parser::parse(&input_str).expect("Should parse successfully");
        let expected_parts: Vec<Part> = vec![
            Part {
                x: 787,
                m: 2655,
                a: 1222,
                s: 2876,
            },
            Part {
                x: 1679,
                m: 44,
                a: 2067,
                s: 496,
            },
            Part {
                x: 2036,
                m: 264,
                a: 79,
                s: 2244,
            },
            Part {
                x: 2461,
                m: 1339,
                a: 466,
                s: 291,
            },
            Part {
                x: 2127,
                m: 1623,
                a: 2188,
                s: 1013,
            },
        ];
        assert_eq!(expected_parts, actual_parts)
    }
}
