extern crate peg;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day25_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule word() -> String
        = word:$(['a'..='z']+) { word.to_string() }
    rule line() -> (String, Vec<String>)
        = left:word() ": " right:word() ++ " " { (left, right) }
    pub rule parse() -> Vec<( String, String )>
        = lines:line() ++ ("\n" +) "\n" * {
            let mut ret = Vec::new();
            for (left, right) in lines.into_iter() {
                for right_inner in right.into_iter() {
                    ret.push((left.clone(), right_inner));
                }
            }
            ret
        }
}}

pub fn parse_data(input: &str) -> Result<Vec<(String, String)>, AOCFileOrParseError> {
    if let Ok(ret) = day25_parser::parse(input) {
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
        let actual = day25_parser::parse(&input_str).expect("Should parse successfully");
        let mut expected: Vec<(String, String)> = Vec::new();
        for (left, right) in [
            ("jqt", "rhn"),
            ("jqt", "xhk"),
            ("jqt", "nvd"),
            ("rsh", "frs"),
            ("rsh", "pzl"),
            ("rsh", "lsr"),
            ("xhk", "hfx"),
            ("cmg", "qnr"),
            ("cmg", "nvd"),
            ("cmg", "lhk"),
            ("cmg", "bvb"),
            ("rhn", "xhk"),
            ("rhn", "bvb"),
            ("rhn", "hfx"),
            ("bvb", "xhk"),
            ("bvb", "hfx"),
            ("pzl", "lsr"),
            ("pzl", "hfx"),
            ("pzl", "nvd"),
            ("qnr", "nvd"),
            ("ntq", "jqt"),
            ("ntq", "hfx"),
            ("ntq", "bvb"),
            ("ntq", "xhk"),
            ("nvd", "lhk"),
            ("lsr", "lhk"),
            ("rzs", "qnr"),
            ("rzs", "cmg"),
            ("rzs", "lsr"),
            ("rzs", "rsh"),
            ("frs", "qnr"),
            ("frs", "lhk"),
            ("frs", "lsr"),
        ]
        .into_iter()
        {
            expected.push((left.to_string(), right.to_string()));
        }
        assert_eq!(expected, actual)
    }
}
