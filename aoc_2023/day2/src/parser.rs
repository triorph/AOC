extern crate peg;

use crate::cube_set::CubeSet;
use crate::game::Game;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day2_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule colour() -> &'input str
        = colour:$(['a'..='z']*) { colour }
    rule colour_and_quantity() -> (&'input str, usize)
        = quantity:number() " " colour:colour() { (colour, quantity) }
    rule cube_set() -> CubeSet
        = colours_and_quantities:colour_and_quantity() ++ (", "*) {
            CubeSet::from_iter(
                colours_and_quantities
                    .into_iter()
                    .map(|(colour, quantity)|
                        (colour.to_string(), quantity)))
        }
    rule game() -> Game
        = "Game " id:number() ": " cube_sets:cube_set() ++ ("; ") {
            Game::new(id, cube_sets)
        }
    pub rule parse() -> Vec<Game>
        = games:game() ++ ("\n" +) "\n" * {
            games
        }
}}

pub fn parse_data(input: &str) -> Result<Vec<Game>, AOCFileOrParseError> {
    if let Ok(ret) = day2_parser::parse(input) {
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
        let actual = day2_parser::parse(&input_str).expect("Should parse successfully");
        assert_eq!(5, actual.len())
    }
}
