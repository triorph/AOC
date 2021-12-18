extern crate peg;
use itertools::Itertools;

#[derive(PartialEq)]
enum ExplosionAssignation {
    NothingDone,
    ToAssignLeft(usize),
    ToAssignRight(usize),
    AllExploded,
}
#[derive(Clone)]
pub enum SnailfishNumber {
    Literal(usize),
    Tuple(Box<SnailfishNumber>, Box<SnailfishNumber>),
}
pub struct Day18Setup {
    snailfish_numbers: Vec<SnailfishNumber>,
}

impl std::fmt::Debug for SnailfishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ret = match self {
            SnailfishNumber::Literal(val) => format!("{}", val),
            SnailfishNumber::Tuple(left, right) => format!("[{:?},{:?}]", left, right),
        };
        write!(f, "{}", ret)
    }
}

peg::parser! { grammar day18_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule snailfishliteral() -> SnailfishNumber
        = n:number() { SnailfishNumber::Literal(n) }
    rule snailfishtuple() -> SnailfishNumber
        = "[" left:snailfishnumber() "," right:snailfishnumber() "]" { SnailfishNumber::Tuple(Box::new(left), Box::new(right))}
    pub rule snailfishnumber() -> SnailfishNumber
        = n:(snailfishliteral() / snailfishtuple()) { n }
    pub rule parse() -> Day18Setup
        = snailfish_numbers:snailfishnumber() ++ "\n" "\n" * {
            Day18Setup { snailfish_numbers }
        }
}}

impl SnailfishNumber {
    fn new(input_str: &str) -> SnailfishNumber {
        day18_parser::snailfishnumber(input_str).expect("SnailfishNumber did not parse correctly")
    }

    fn calculate_magnitude(&self) -> usize {
        match self {
            SnailfishNumber::Literal(val) => *val,
            SnailfishNumber::Tuple(left, right) => {
                3 * left.calculate_magnitude() + 2 * right.calculate_magnitude()
            }
        }
    }

    fn get_leftmost(&mut self) -> &mut usize {
        match self {
            SnailfishNumber::Literal(val) => val,
            SnailfishNumber::Tuple(left, _) => left.get_leftmost(),
        }
    }

    fn get_rightmost(&mut self) -> &mut usize {
        match self {
            SnailfishNumber::Literal(val) => val,
            SnailfishNumber::Tuple(_, right) => right.get_rightmost(),
        }
    }

    fn is_tuple(&self) -> bool {
        matches!(self, SnailfishNumber::Tuple(_, _))
    }

    fn get_literal(&self) -> usize {
        if let SnailfishNumber::Literal(val) = self {
            *val
        } else {
            panic!("Tried to get a literal value from a tuple");
        }
    }

    fn get_tuple_vals(&self) -> (usize, usize) {
        if let SnailfishNumber::Tuple(left, right) = self {
            (left.get_literal(), right.get_literal())
        } else {
            panic!("Cannot get tuple pairs. Tuple still has nested objects");
        }
    }

    fn find_explode_point(
        &self,
        depth: usize,
        found: ExplosionAssignation,
    ) -> (SnailfishNumber, ExplosionAssignation) {
        match (self, found) {
            (SnailfishNumber::Literal(val), ExplosionAssignation::ToAssignLeft(to_assign)) => (
                SnailfishNumber::Literal(*val + to_assign),
                ExplosionAssignation::NothingDone,
            ),
            (SnailfishNumber::Literal(val), ExplosionAssignation::ToAssignRight(to_assign)) => (
                SnailfishNumber::Literal(*val + to_assign),
                ExplosionAssignation::NothingDone,
            ),
            (SnailfishNumber::Literal(val), found) => (SnailfishNumber::Literal(*val), found),
            (SnailfishNumber::Tuple(left, right), ExplosionAssignation::NothingDone) => {
                let found = ExplosionAssignation::NothingDone;
                if depth == 4 {
                    if left.is_tuple() {
                        println!("left node is exploding, {:?}, {:?}", left, right);
                        let (left_to_copy, right_to_copy) = left.get_tuple_vals();
                        let (new_val, _) = right.find_explode_point(
                            depth + 1,
                            ExplosionAssignation::ToAssignRight(right_to_copy),
                        );
                        (
                            SnailfishNumber::Tuple(
                                Box::new(SnailfishNumber::Literal(0)),
                                Box::new(new_val),
                            ),
                            ExplosionAssignation::ToAssignLeft(left_to_copy),
                        )
                    } else if right.is_tuple() {
                        println!("right node is exploding, {:?}, {:?}", left, right);
                        let (left_to_copy, right_to_copy) = left.get_tuple_vals();
                        let (new_val, _) = left.find_explode_point(
                            depth + 1,
                            ExplosionAssignation::ToAssignRight(left_to_copy),
                        );
                        (new_val, ExplosionAssignation::ToAssignRight(right_to_copy))
                    } else {
                        (self.clone(), found)
                    }
                } else {
                    let found_to_use = match &found {
                        ExplosionAssignation::ToAssignRight(_) => ExplosionAssignation::AllExploded,
                        found => *found,
                    };
                    let (left, found) = left.find_explode_point(depth + 1, found_to_use);
                    let found_to_use = match &found {
                        ExplosionAssignation::ToAssignRight(_) => ExplosionAssignation::AllExploded,
                        found => *found,
                    };
                    let (right, found) = right.find_explode_point(depth + 1, found);
                    if let ExplosionAssignation::ToAssignLeft(found) = found {
                        let (left, found) = left.find_explode_point(
                            depth + 1,
                            ExplosionAssignation::ToAssignLeft(found),
                        );
                        (
                            SnailfishNumber::Tuple(Box::new(left), Box::new(right)),
                            found,
                        )
                    } else {
                        (
                            SnailfishNumber::Tuple(Box::new(left), Box::new(right)),
                            found,
                        )
                    }
                }
            }
            (
                SnailfishNumber::Tuple(left, right),
                ExplosionAssignation::ToAssignLeft(assign_val),
            ) => {
                let found = ExplosionAssignation::ToAssignLeft(assign_val);
                let (right, _) = right.find_explode_point(depth + 1, found);
                let found = ExplosionAssignation::AllExploded;
                let (left, found) = left.find_explode_point(depth + 1, found);
                (
                    SnailfishNumber::Tuple(Box::new(left), Box::new(right)),
                    found,
                )
            }
            (
                SnailfishNumber::Tuple(left, right),
                ExplosionAssignation::ToAssignRight(assign_val),
            ) => {
                let found = ExplosionAssignation::ToAssignRight(assign_val);
                let (left, _) = left.find_explode_point(depth + 1, found);
                let found = ExplosionAssignation::AllExploded;
                let (right, found) = right.find_explode_point(depth + 1, found);
                (
                    SnailfishNumber::Tuple(Box::new(left), Box::new(right)),
                    found,
                )
            }
            (SnailfishNumber::Tuple(_, _), found) => (self.clone(), found),
        }
    }

    fn run_explosion(&self) -> (SnailfishNumber, bool) {
        let (next, explosion_assignation) =
            self.find_explode_point(1, ExplosionAssignation::NothingDone);
        (
            next,
            explosion_assignation != ExplosionAssignation::NothingDone,
        )
    }

    fn run_split(&self) -> bool {
        false
    }

    fn reduce_once(&self) -> (SnailfishNumber, bool) {
        let (next, exploded) = self.run_explosion();
        if !exploded {
            (next, self.run_split())
        } else {
            (next, true)
        }
    }

    fn reduce(&self) -> SnailfishNumber {
        let mut curr = self.clone();
        let mut can_continue = true;
        while can_continue {
            let (next, next_can_continue) = curr.reduce_once();
            curr = next;
            can_continue = next_can_continue;
        }
        curr
    }
}

impl Day18Setup {
    /// Generates a new Day18Setup object to be calculated upon
    ///
    /// Inputs: the full string contents of the input data set.
    ///
    /// Returns: A new Day18Setup object, with methods `calculate_day_a` and `calculate_day_b`
    /// available
    pub fn new(input_str: &str) -> Day18Setup {
        day18_parser::parse(input_str)
            .expect("The given input did not parse to a final object for day 18")
    }

    /// Calculate the part a response
    pub fn calculate_day_a(self: &Day18Setup) -> usize {
        self.snailfish_numbers
            .clone()
            .into_iter()
            .reduce(|a, b| SnailfishNumber::Tuple(Box::new(a), Box::new(b)).reduce())
            .expect("more than 1 snailline given")
            .calculate_magnitude()
    }

    /// Calculate the part b response
    pub fn calculate_day_b(self: &Day18Setup) -> usize {
        0
    }
}

#[cfg(test)]
mod test {
    use crate::Day18Setup;
    use crate::SnailfishNumber;

    #[test]
    fn test_parse() {
        let day18_setup = Day18Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day18_setup.snailfish_numbers.len(), 10);
        assert_eq!(
            format!("{:?}", day18_setup.snailfish_numbers[0]),
            String::from("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]")
        );
        assert_eq!(
            format!("{:?}", day18_setup.snailfish_numbers[1]),
            String::from("[[[5,[2,8]],4],[5,[[9,9],0]]]")
        );
        assert_eq!(
            format!("{:?}", day18_setup.snailfish_numbers[2]),
            String::from("[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]")
        );
    }

    #[test]
    fn test_day_a() {
        let day18_setup = Day18Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day18_setup.calculate_day_a(), 4140);
    }

    #[test]
    fn test_run_explosion() {
        let (n, f) = SnailfishNumber::new("[[[[[9,8],1],2],3],4]").run_explosion();
        assert_eq!(format!("{:?}", n), String::from("[[[[0,9],2],3],4]"));
        assert!(f);
        let (n, f) = SnailfishNumber::new("[7,[6,[5,[4,[3,2]]]]]").run_explosion();
        assert!(f);
        assert_eq!(format!("{:?}", n), String::from("[7,[6,[5,[7,0]]]]"));
        let (n, f) = SnailfishNumber::new("[[6,[5,[4,[3,2]]]],1]").run_explosion();
        assert!(f);
        assert_eq!(format!("{:?}", n), String::from("[[6,[5,[7,0]]],3]"));
        let (n, f) = SnailfishNumber::new("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").run_explosion();
        assert!(f);
        assert_eq!(
            format!("{:?}", n),
            String::from("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")
        );
        let (n, f) = SnailfishNumber::new("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").run_explosion();
        assert!(f);
        assert_eq!(
            format!("{:?}", n),
            String::from("[[3,[2,[8,0]]],[9,[5,[7,0]]]]")
        );
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(
            SnailfishNumber::new("[[1,2],[[3,4],5]]").calculate_magnitude(),
            143
        );
        assert_eq!(
            SnailfishNumber::new("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").calculate_magnitude(),
            1384
        );
        assert_eq!(
            SnailfishNumber::new("[[[[1,1],[2,2]],[3,3]],[4,4]]").calculate_magnitude(),
            445
        );
        assert_eq!(
            SnailfishNumber::new("[[[[3,0],[5,3]],[4,4]],[5,5]]").calculate_magnitude(),
            791
        );
        assert_eq!(
            SnailfishNumber::new("[[[[5,0],[7,4]],[5,5]],[6,6]]").calculate_magnitude(),
            1137
        );
        assert_eq!(
            SnailfishNumber::new("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
                .calculate_magnitude(),
            3488
        );
    }

    #[test]
    fn test_day_b() {
        let day18_setup = Day18Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day18_setup.calculate_day_b(), 0);
    }
}
