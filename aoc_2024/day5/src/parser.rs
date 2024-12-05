extern crate peg;
use aoc_helpers::AOCFileOrParseError;

pub type OrderRule = (usize, usize);
pub type PageNumber = Vec<usize>;

peg::parser! { pub grammar day5_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule order_rule() -> OrderRule
        = left:number() "|" right:number() { (left, right)  }
    rule page_numbers() -> PageNumber
        = page_numbers:number() ++ "," { page_numbers }
    rule order_rules() -> Vec<OrderRule>
        = order_rules:order_rule() ++ "\n" { order_rules }
    rule all_page_numbers() -> Vec<PageNumber>
        = all_page_numbers:page_numbers() ++ "\n" { all_page_numbers }
    pub rule parse() -> (Vec<OrderRule>, Vec<PageNumber>)
        = order_rules:order_rules() "\n"+ all_page_numbers:all_page_numbers() "\n"* { (order_rules,all_page_numbers) }
}}

pub fn parse_data(input: &str) -> Result<(Vec<OrderRule>, Vec<PageNumber>), AOCFileOrParseError> {
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
        let actual = day5_parser::parse(&input_str).expect("Should parse successfully");
        let expected: (Vec<OrderRule>, Vec<PageNumber>) = (
            vec![
                (47, 53),
                (97, 13),
                (97, 61),
                (97, 47),
                (75, 29),
                (61, 13),
                (75, 53),
                (29, 13),
                (97, 29),
                (53, 29),
                (61, 53),
                (97, 53),
                (61, 29),
                (47, 13),
                (75, 47),
                (97, 75),
                (47, 61),
                (75, 61),
                (47, 29),
                (75, 13),
                (53, 13),
            ],
            vec![
                vec![75, 47, 61, 53, 29],
                vec![97, 61, 53, 29, 13],
                vec![75, 29, 13],
                vec![75, 97, 47, 61, 53],
                vec![61, 13, 29],
                vec![97, 13, 75, 29, 47],
            ],
        );
        assert_eq!(expected, actual)
    }
}
