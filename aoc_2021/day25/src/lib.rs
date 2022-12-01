extern crate peg;
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Clone)]
struct Point(usize, usize);

#[derive(Clone, PartialEq)]
enum Cucumber {
    South,
    East,
}

#[derive(Clone, PartialEq)]
pub struct Day25Setup {
    positions: HashMap<Point, Cucumber>,
    size: Point,
}

peg::parser! { grammar day25_parser() for str {
    rule empty() -> Option<Cucumber>
        = "." { None }
    rule east() -> Option<Cucumber>
        = ">" { Some(Cucumber::East) }
    rule south() -> Option<Cucumber>
        = "v" { Some(Cucumber::South) }
    rule space() -> Option<Cucumber>
        = space:(south() / east() / empty() ) { space }
    rule line() -> Vec<Option<Cucumber>>
        = line:space() ++ "" { line }
    pub rule parse() -> Day25Setup
        = lines:line() ++ "\n" "\n" * {
            let mut positions: HashMap<Point, Cucumber> = HashMap::new();
            for j in 0..lines.len() {
                for i in 0..lines[j].len() {
                    if let Some(val) = &lines[j][i] {
                        positions.insert(Point(i, j), val.clone());
                    }
                }
            }
            let size = Point(lines[0].len(), lines.len());
            Day25Setup { positions, size }
        }
}}

impl Day25Setup {
    /// Generates a new Day25Setup object to be calculated upon
    ///
    /// Inputs: the full string contents of the input data set.
    ///
    /// Returns: A new Day25Setup object, with methods `calculate_day_a` and `calculate_day_b`
    /// available
    pub fn new(input_str: &str) -> Day25Setup {
        day25_parser::parse(input_str).unwrap()
    }

    fn run_east_movements(&self) -> Day25Setup {
        let mut next: HashMap<Point, Cucumber> = HashMap::new();
        for (key, val) in self.positions.iter() {
            if val == &Cucumber::East {
                let p = Point((key.0 + 1) % self.size.0, key.1);
                if self.positions.contains_key(&p) {
                    next.insert(key.clone(), val.clone());
                } else {
                    next.insert(p, val.clone());
                }
            } else {
                next.insert(key.clone(), val.clone());
            }
        }
        Day25Setup {
            positions: next,
            size: self.size.clone(),
        }
    }

    fn run_south_movements(&self) -> Day25Setup {
        let mut next: HashMap<Point, Cucumber> = HashMap::new();
        for (key, val) in self.positions.iter() {
            if val == &Cucumber::South {
                let p = Point(key.0, (key.1 + 1) % self.size.1);
                if self.positions.contains_key(&p) {
                    next.insert(key.clone(), val.clone());
                } else {
                    next.insert(p, val.clone());
                }
            } else {
                next.insert(key.clone(), val.clone());
            }
        }
        Day25Setup {
            positions: next,
            size: self.size.clone(),
        }
    }

    fn get_next_iteration(&self) -> Day25Setup {
        let next = self.run_east_movements();
        next.run_south_movements()
    }

    /// Calculate the part a response
    pub fn calculate_day_a(self: &Day25Setup) -> usize {
        let mut count = 0;
        let mut curr = self.clone();
        loop {
            count += 1;
            let next = curr.get_next_iteration();
            if next == curr {
                break;
            } else {
                curr = next;
            }
        }
        count
    }

    /// Calculate the part b response
    pub fn calculate_day_b(self: &Day25Setup) -> usize {
        0
    }
}

#[cfg(test)]
mod test {
    use crate::Day25Setup;

    #[test]
    fn test_parse() {
        let _day25_setup = Day25Setup::new(include_str!("../test_data.txt"));
    }

    #[test]
    fn test_day_a() {
        let day25_setup = Day25Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day25_setup.calculate_day_a(), 58);
    }
}
