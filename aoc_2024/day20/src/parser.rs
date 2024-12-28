extern crate peg;
use aoc_helpers::AOCFileOrParseError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MazeTile {
    Wall,
    Empty,
    Start,
    End,
}

peg::parser! { pub grammar day20_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule wall() -> MazeTile
        = "#" { MazeTile::Wall }
    rule empty() -> MazeTile
        = "." { MazeTile::Empty }
    rule start() -> MazeTile
        = "S" { MazeTile::Start }
    rule end() -> MazeTile
        = "E" { MazeTile::End }
    rule maze_tile() -> MazeTile
        = maze_tile:(wall() / empty() / start() / end()) { maze_tile }
    rule line() -> Vec<MazeTile>
        = line:maze_tile() ++ "" { line }
    pub rule parse() -> Vec<Vec<MazeTile>>
        = lines:line() ++ ("\n" +) "\n" * { lines }
}}

pub fn parse_data(input: &str) -> Result<Vec<Vec<MazeTile>>, AOCFileOrParseError> {
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
        let expected: Vec<Vec<MazeTile>> = vec![
            vec![
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
            ],
            vec![
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Empty,
                MazeTile::Empty,
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Empty,
                MazeTile::Empty,
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Empty,
                MazeTile::Empty,
                MazeTile::Empty,
                MazeTile::Empty,
                MazeTile::Wall,
            ],
            vec![
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Wall,
            ],
            vec![
                MazeTile::Wall,
                MazeTile::Start,
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Empty,
                MazeTile::Empty,
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Empty,
                MazeTile::Empty,
                MazeTile::Wall,
            ],
            vec![
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
            ],
            vec![
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Empty,
                MazeTile::Empty,
                MazeTile::Wall,
            ],
            vec![
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Wall,
            ],
            vec![
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Empty,
                MazeTile::End,
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Empty,
                MazeTile::Empty,
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Empty,
                MazeTile::Empty,
                MazeTile::Wall,
            ],
            vec![
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
            ],
            vec![
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Empty,
                MazeTile::Empty,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Empty,
                MazeTile::Empty,
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Empty,
                MazeTile::Empty,
                MazeTile::Wall,
            ],
            vec![
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Wall,
            ],
            vec![
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Empty,
                MazeTile::Empty,
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Empty,
                MazeTile::Empty,
                MazeTile::Wall,
            ],
            vec![
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
            ],
            vec![
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Empty,
                MazeTile::Empty,
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Empty,
                MazeTile::Empty,
                MazeTile::Wall,
                MazeTile::Empty,
                MazeTile::Empty,
                MazeTile::Empty,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
            ],
            vec![
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
                MazeTile::Wall,
            ],
        ];
        assert_eq!(expected, actual);
    }
}
