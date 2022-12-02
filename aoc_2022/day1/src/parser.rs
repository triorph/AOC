extern crate peg;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day1_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule calory_package() -> Vec<usize>
        = calories:number() ++ ("\n")
    pub rule parse() -> Vec<Vec<usize>>
        = all_elf_calories:calory_package() ++ ("\n" +) "\n" * {
             { all_elf_calories }
        }
}}

pub fn parse_data(input: &str) -> Result<Vec<Vec<usize>>, AOCFileOrParseError> {
    if let Ok(ret) = day1_parser::parse(input) {
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
        let expected = day1_parser::parse(&input_str).expect("Should parse successfully");
        let actual = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];
        assert_eq!(expected, actual)
    }
}
