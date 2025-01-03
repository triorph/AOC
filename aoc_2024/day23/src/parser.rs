extern crate peg;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day23_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule node() -> String
        = node:$(['a'..='z']+) { node.to_string() }
    rule edge() -> (String, String)
        = left:node() "-" right:node() { (left, right) }
    pub rule parse() -> Vec<(String, String)>
        = lines:edge() ++ ("\n" +) "\n" * { lines }
}}

pub fn parse_data(input: &str) -> Result<Vec<(String, String)>, AOCFileOrParseError> {
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
        let expected: Vec<(String, String)> = vec![
            (String::from("kh"), String::from("tc")),
            (String::from("qp"), String::from("kh")),
            (String::from("de"), String::from("cg")),
            (String::from("ka"), String::from("co")),
            (String::from("yn"), String::from("aq")),
            (String::from("qp"), String::from("ub")),
            (String::from("cg"), String::from("tb")),
            (String::from("vc"), String::from("aq")),
            (String::from("tb"), String::from("ka")),
            (String::from("wh"), String::from("tc")),
            (String::from("yn"), String::from("cg")),
            (String::from("kh"), String::from("ub")),
            (String::from("ta"), String::from("co")),
            (String::from("de"), String::from("co")),
            (String::from("tc"), String::from("td")),
            (String::from("tb"), String::from("wq")),
            (String::from("wh"), String::from("td")),
            (String::from("ta"), String::from("ka")),
            (String::from("td"), String::from("qp")),
            (String::from("aq"), String::from("cg")),
            (String::from("wq"), String::from("ub")),
            (String::from("ub"), String::from("vc")),
            (String::from("de"), String::from("ta")),
            (String::from("wq"), String::from("aq")),
            (String::from("wq"), String::from("vc")),
            (String::from("wh"), String::from("yn")),
            (String::from("ka"), String::from("de")),
            (String::from("kh"), String::from("ta")),
            (String::from("co"), String::from("tc")),
            (String::from("wh"), String::from("qp")),
            (String::from("tb"), String::from("vc")),
            (String::from("td"), String::from("yn")),
        ];
        assert_eq!(expected, actual)
    }
}
