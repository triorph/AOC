extern crate peg;
use std::fs::File;
use std::io::prelude::*;

struct SyntaxLines {
    syntax_lines: Vec<String>,
}
impl SyntaxLines {
    fn new(input: &str) -> SyntaxLines {
        SyntaxLines {
            syntax_lines: input.lines().map(String::from).collect::<Vec<String>>(),
        }
    }

    fn find_first_invalid_char(line: &str) -> (Option<char>, Vec<char>) {
        let mut stack = vec![];
        for c in line.chars() {
            if ['(', '[', '<', '{'].contains(&c) {
                stack.push(c);
            } else {
                match (stack.pop(), c) {
                    (Some('('), ')') => (),
                    (Some('['), ']') => (),
                    (Some('{'), '}') => (),
                    (Some('<'), '>') => (),
                    (_, c) => {
                        return (Some(c), vec![]);
                    }
                }
            }
        }
        (None, stack)
    }

    fn turn_missing_char_to_score_a(c: Option<char>) -> usize {
        match c {
            Some(')') => 3,
            Some(']') => 57,
            Some('}') => 1197,
            Some('>') => 25137,
            _ => 0,
        }
    }

    fn calculate_day_a(self: &SyntaxLines) -> usize {
        self.syntax_lines
            .iter()
            .map(|line| SyntaxLines::find_first_invalid_char(&line[..]))
            .map(|(c, _)| SyntaxLines::turn_missing_char_to_score_a(c))
            .sum()
    }
    fn turn_char_to_score_b(c: char) -> usize {
        match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => 0,
        }
    }

    fn calculate_score_b(line: &str) -> usize {
        let mut score = 0;
        for c in line.chars() {
            score *= 5;
            score += SyntaxLines::turn_char_to_score_b(c);
        }
        score
    }

    fn get_reverse_bracket(c: char) -> Option<char> {
        match c {
            '(' => Some(')'),
            '[' => Some(']'),
            '{' => Some('}'),
            '<' => Some('>'),
            _ => None,
        }
    }

    fn build_remaining_str(line: Vec<char>) -> String {
        line.into_iter()
            .rev()
            .map(SyntaxLines::get_reverse_bracket)
            .flatten()
            .collect::<String>()
    }

    fn calculate_day_b(self: &SyntaxLines) -> usize {
        let mut scores = self
            .syntax_lines
            .iter()
            .map(|line| SyntaxLines::find_first_invalid_char(&line[..]))
            .map(|(_, stack)| SyntaxLines::build_remaining_str(stack))
            .map(|line| SyntaxLines::calculate_score_b(&line[..]))
            .filter(|score| *score != 0)
            .collect::<Vec<usize>>();
        scores.sort_unstable();
        scores[scores.len() / 2]
    }
}

fn main() {
    let mut f = File::open("input_data.txt").expect("Need input_data.txt to exist");
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    let syntax_lines = SyntaxLines::new(&buffer[..]);
    let day_a = syntax_lines.calculate_day_a();
    println!("Day a result: {}", day_a);
    let syntax_lines = SyntaxLines::new(&buffer[..]);
    let day_b = syntax_lines.calculate_day_b();
    println!("Day b result: {}", day_b);
}

#[cfg(test)]
mod test {
    use crate::SyntaxLines;

    #[test]
    fn test_parse() {
        let syntax_lines = SyntaxLines::new(include_str!("../test_data.txt"));
        assert_eq!(syntax_lines.syntax_lines.len(), 10);
    }

    #[test]
    fn test_day_a() {
        let syntax_lines = SyntaxLines::new(include_str!("../test_data.txt"));
        assert_eq!(syntax_lines.calculate_day_a(), 26397);
    }

    #[test]
    fn test_calculate_score_b() {
        assert_eq!(SyntaxLines::calculate_score_b("}}]])})]"), 288957);
        assert_eq!(SyntaxLines::calculate_score_b(")}>]})"), 5566);
        assert_eq!(SyntaxLines::calculate_score_b("}}>}>))))"), 1480781);
        assert_eq!(SyntaxLines::calculate_score_b("]]}}]}]}>"), 995444);
        assert_eq!(SyntaxLines::calculate_score_b("])}>"), 294);
    }

    #[test]
    fn test_build_remaining_score() {
        let syntax_lines = SyntaxLines::new(include_str!("../test_data.txt"));
        assert_eq!(
            SyntaxLines::build_remaining_str(
                SyntaxLines::find_first_invalid_char(&syntax_lines.syntax_lines[0][..]).1
            ),
            "}}]])})]"
        );
        assert_eq!(
            SyntaxLines::build_remaining_str(
                SyntaxLines::find_first_invalid_char(&syntax_lines.syntax_lines[1][..]).1
            ),
            ")}>]})"
        );
        assert_eq!(
            SyntaxLines::build_remaining_str(
                SyntaxLines::find_first_invalid_char(&syntax_lines.syntax_lines[3][..]).1
            ),
            "}}>}>))))"
        );
    }

    #[test]
    fn test_day_b() {
        let syntax_lines = SyntaxLines::new(include_str!("../test_data.txt"));
        assert_eq!(syntax_lines.calculate_day_b(), 288957);
    }
}
