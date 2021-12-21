extern crate peg;
use itertools::Itertools;

#[derive(PartialEq, Clone, Debug)]
enum ExplosionState {
    NothingDone,
    ToAssignLeft(usize),
    ToAssignRight(usize),
    AllExploded,
}

#[derive(Clone, PartialEq)]
pub enum SnailNumber {
    Literal(usize),
    Tuple(Box<SnailNumber>, Box<SnailNumber>),
}
pub struct Day18Setup {
    snailfish_numbers: Vec<SnailNumber>,
}

impl std::fmt::Debug for SnailNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ret = match self {
            SnailNumber::Literal(val) => format!("{}", val),
            SnailNumber::Tuple(left, right) => format!("[{:?},{:?}]", left, right),
        };
        write!(f, "{}", ret)
    }
}

peg::parser! { grammar day18_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule snailfishliteral() -> SnailNumber
        = n:number() { SnailNumber::Literal(n) }
    rule snailfishtuple() -> SnailNumber
        = "[" left:snailfishnumber() "," right:snailfishnumber() "]" { SnailNumber::Tuple(Box::new(left), Box::new(right))}
    pub rule snailfishnumber() -> SnailNumber
        = n:(snailfishliteral() / snailfishtuple()) { n }
    pub rule parse() -> Day18Setup
        = snailfish_numbers:snailfishnumber() ++ "\n" "\n" * {
            Day18Setup { snailfish_numbers }
        }
}}

impl SnailNumber {
    fn run_mut_split(&mut self, has_split: bool) -> bool {
        if !has_split {
            match &mut *self {
                SnailNumber::Literal(val) => {
                    if *val >= 10 {
                        *self = SnailNumber::Tuple(
                            Box::new(SnailNumber::Literal(*val / 2)),
                            Box::new(SnailNumber::Literal(*val / 2 + *val % 2)),
                        );
                        true
                    } else {
                        has_split
                    }
                }
                SnailNumber::Tuple(left, right) => {
                    let has_split = left.run_mut_split(has_split);
                    right.run_mut_split(has_split)
                }
            }
        } else {
            has_split
        }
    }
    fn explode_literal_pair(
        self: &mut std::boxed::Box<SnailNumber>,
        depth: usize,
        state: ExplosionState,
    ) -> ExplosionState {
        if state != ExplosionState::NothingDone {
            return state;
        }
        match &mut **self {
            SnailNumber::Literal(_) => state,
            SnailNumber::Tuple(left, right) => {
                if depth > 3 && left.is_literal_pair() {
                    let (pair_left, pair_right) = left.get_tuple_vals();
                    *left = Box::new(SnailNumber::Literal(0));
                    right.assign_right(pair_right);
                    ExplosionState::ToAssignLeft(pair_left)
                } else if depth > 3 && right.is_literal_pair() {
                    let (pair_left, pair_right) = right.get_tuple_vals();
                    left.assign_left(pair_left);
                    *right = Box::new(SnailNumber::Literal(0));
                    ExplosionState::ToAssignRight(pair_right)
                } else {
                    self.run_explosion(depth, state)
                }
            }
        }
    }

    fn assign_left(&mut self, val: usize) -> ExplosionState {
        match &mut *self {
            SnailNumber::Literal(current) => {
                *self = SnailNumber::Literal(*current + val);
                ExplosionState::AllExploded
            }
            SnailNumber::Tuple(_, right) => right.assign_left(val),
        }
    }

    fn assign_right(&mut self, val: usize) -> ExplosionState {
        match &mut *self {
            SnailNumber::Literal(current) => {
                *self = SnailNumber::Literal(*current + val);
                ExplosionState::AllExploded
            }
            SnailNumber::Tuple(left, _) => left.assign_right(val),
        }
    }
    fn _new(input_str: &str) -> SnailNumber {
        day18_parser::snailfishnumber(input_str).expect("SnailNumber did not parse correctly")
    }

    fn calculate_magnitude(&self) -> usize {
        match self {
            SnailNumber::Literal(val) => *val,
            SnailNumber::Tuple(left, right) => {
                3 * left.calculate_magnitude() + 2 * right.calculate_magnitude()
            }
        }
    }

    fn run_explosion(&mut self, depth: usize, state: ExplosionState) -> ExplosionState {
        if let SnailNumber::Tuple(left, right) = self {
            if state == ExplosionState::NothingDone {
                let left_state = left.explode_literal_pair(depth + 1, state.clone());
                if left_state == ExplosionState::NothingDone {
                    let right_state = right.explode_literal_pair(depth + 1, state);
                    if let ExplosionState::ToAssignLeft(val) = right_state {
                        left.assign_left(val)
                    } else {
                        right_state
                    }
                } else if let ExplosionState::ToAssignRight(val) = left_state {
                    right.assign_right(val)
                } else {
                    left_state
                }
            } else {
                state
            }
        } else {
            state
        }
    }

    fn is_literal(&self) -> bool {
        matches!(self, SnailNumber::Literal(_))
    }

    fn is_literal_pair(&self) -> bool {
        match self {
            SnailNumber::Tuple(left, right) => left.is_literal() && right.is_literal(),
            _ => false,
        }
    }

    fn get_literal(&self) -> usize {
        if let SnailNumber::Literal(val) = self {
            *val
        } else {
            panic!("Tried to get a literal value from a tuple");
        }
    }

    fn get_tuple_vals(&self) -> (usize, usize) {
        if let SnailNumber::Tuple(left, right) = self {
            (left.get_literal(), right.get_literal())
        } else {
            panic!("Cannot get tuple pairs. Tuple still has nested objects");
        }
    }

    fn run_mut_split_on_head(&mut self) -> bool {
        match self {
            SnailNumber::Literal(_) => false,
            SnailNumber::Tuple(left, right) => {
                let has_split = left.run_mut_split(false);
                right.run_mut_split(has_split)
            }
        }
    }

    fn reduce_once(&mut self) -> bool {
        let exploded =
            self.run_explosion(1, ExplosionState::NothingDone) != ExplosionState::NothingDone;
        if !exploded {
            self.run_mut_split_on_head()
        } else {
            true
        }
    }

    fn reduce(&mut self) {
        while self.reduce_once() {}
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
            .reduce(|a, b| {
                let mut ret = SnailNumber::Tuple(Box::new(a), Box::new(b));
                ret.reduce();
                ret
            })
            .expect("more than 1 snailline given")
            .calculate_magnitude()
    }

    /// Calculate the part b response
    pub fn calculate_day_b(self: &Day18Setup) -> usize {
        self.snailfish_numbers
            .iter()
            .cartesian_product(self.snailfish_numbers.iter())
            .filter(|(a, b)| a != b)
            .map(|(a, b)| {
                let mut ret = SnailNumber::Tuple(Box::new(a.clone()), Box::new(b.clone()));
                ret.reduce();
                ret.calculate_magnitude()
            })
            .reduce(std::cmp::max)
            .expect("more than 1 snailline given")
    }
}

#[cfg(test)]
mod test {
    use crate::Day18Setup;
    use crate::ExplosionState;
    use crate::SnailNumber;

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
    fn test_run_mut_explosion() {
        let mut sn = SnailNumber::_new("[[[[[9,8],1],2],3],4]");
        let f = sn.run_explosion(1, ExplosionState::NothingDone) != ExplosionState::NothingDone;
        assert_eq!(format!("{:?}", sn), String::from("[[[[0,9],2],3],4]"));
        assert!(f);
        let mut sn = SnailNumber::_new("[7,[6,[5,[4,[3,2]]]]]");
        let f = sn.run_explosion(1, ExplosionState::NothingDone) != ExplosionState::NothingDone;
        assert_eq!(format!("{:?}", sn), String::from("[7,[6,[5,[7,0]]]]"));
        assert!(f);
        let mut sn = SnailNumber::_new("[[6,[5,[4,[3,2]]]],1]");
        let f = sn.run_explosion(1, ExplosionState::NothingDone) != ExplosionState::NothingDone;
        assert_eq!(format!("{:?}", sn), String::from("[[6,[5,[7,0]]],3]"));
        assert!(f);
        let mut sn = SnailNumber::_new("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        let f = sn.run_explosion(1, ExplosionState::NothingDone) != ExplosionState::NothingDone;
        assert_eq!(
            format!("{:?}", sn),
            String::from("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")
        );
        assert!(f);
        let mut sn = SnailNumber::_new("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        let f = sn.run_explosion(1, ExplosionState::NothingDone) != ExplosionState::NothingDone;
        assert_eq!(
            format!("{:?}", sn),
            String::from("[[3,[2,[8,0]]],[9,[5,[7,0]]]]")
        );
        assert!(f);
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(
            SnailNumber::_new("[[1,2],[[3,4],5]]").calculate_magnitude(),
            143
        );
        assert_eq!(
            SnailNumber::_new("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").calculate_magnitude(),
            1384
        );
        assert_eq!(
            SnailNumber::_new("[[[[1,1],[2,2]],[3,3]],[4,4]]").calculate_magnitude(),
            445
        );
        assert_eq!(
            SnailNumber::_new("[[[[3,0],[5,3]],[4,4]],[5,5]]").calculate_magnitude(),
            791
        );
        assert_eq!(
            SnailNumber::_new("[[[[5,0],[7,4]],[5,5]],[6,6]]").calculate_magnitude(),
            1137
        );
        assert_eq!(
            SnailNumber::_new("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
                .calculate_magnitude(),
            3488
        );
    }

    #[test]
    fn test_reduce_steps() {
        let mut next = SnailNumber::_new("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        let has_split = next.reduce_once();
        assert!(has_split);
        assert_eq!(
            format!("{:?}", next),
            String::from("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]")
        );
        let has_split = next.reduce_once();
        assert!(has_split);
        assert_eq!(
            format!("{:?}", next),
            String::from("[[[[0,7],4],[15,[0,13]]],[1,1]]")
        );
        let has_split = next.reduce_once();
        assert!(has_split);
        assert_eq!(
            format!("{:?}", next),
            String::from("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]")
        );
        let has_split = next.reduce_once();
        assert!(has_split);
        assert_eq!(
            format!("{:?}", next),
            String::from("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]")
        );
        let has_split = next.reduce_once();
        assert!(has_split);
        assert_eq!(
            format!("{:?}", next),
            String::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
        );
        let has_split = next.reduce_once();
        assert!(!has_split);
        assert_eq!(
            format!("{:?}", next),
            String::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
        );
    }

    #[test]
    fn test_reduce() {
        let mut sn = SnailNumber::_new("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        sn.reduce();
        assert_eq!(
            format!("{:?}", sn),
            String::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
        );
    }

    #[test]
    fn test_day_b() {
        let day18_setup = Day18Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day18_setup.calculate_day_b(), 3993);
    }
}
