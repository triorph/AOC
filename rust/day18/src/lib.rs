extern crate peg;
use itertools::Itertools;

#[derive(PartialEq, Clone, Debug)]
enum ExplosionState {
    NothingDone,
    ToAssignLeft(usize),
    ToAssignRight(usize),
    AllExploded,
}

#[derive(Clone, Debug)]
struct ExplosionRun {
    number: SnailNumber,
    state: ExplosionState,
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

impl ExplosionRun {
    fn run_normal_left_explode(&self, depth: usize) -> ExplosionRun {
        let original_explosion_state = self.state.clone();
        let explosion_state_to_use = match self.state.clone() {
            ExplosionState::ToAssignRight(_) => ExplosionState::AllExploded,
            state => state,
        };

        let ret = ExplosionRun {
            number: self.number.clone(),
            state: explosion_state_to_use.clone(),
        }
        .find_explode_point(depth + 1);
        if original_explosion_state == explosion_state_to_use {
            ret
        } else {
            // We used a fake AllExploded, so we should return the original ToAssignRight back
            ExplosionRun {
                number: ret.number,
                state: original_explosion_state,
            }
        }
    }

    fn run_normal_right_explode(&self, depth: usize) -> ExplosionRun {
        let original_explosion_state = self.state.clone();
        let explosion_state_to_use = match self.state.clone() {
            ExplosionState::ToAssignLeft(_) => ExplosionState::AllExploded,
            state => state,
        };
        let ret = ExplosionRun {
            number: self.number.clone(),
            state: explosion_state_to_use.clone(),
        }
        .find_explode_point(depth + 1);
        if original_explosion_state == explosion_state_to_use {
            ret
        } else {
            // We used a fake AllExploded, so we should return the original ToAssignLeft back
            ExplosionRun {
                number: ret.number,
                state: original_explosion_state,
            }
        }
    }

    fn rerun_left_to_assign_if_needed(
        &self,
        depth: usize,
        explosion_state_from_right: ExplosionState,
    ) -> ExplosionRun {
        let ret = ExplosionRun {
            number: self.number.clone(),
            state: explosion_state_from_right.clone(),
        };
        match (self.state.clone(), explosion_state_from_right) {
            (ExplosionState::ToAssignLeft(_), _) => ret,
            (_, ExplosionState::ToAssignLeft(_)) => ret.find_explode_point(depth + 1),
            _ => ret,
        }
    }

    fn handle_non_explode_tuples(
        left: &SnailNumber,
        right: &SnailNumber,
        state: ExplosionState,
        depth: usize,
    ) -> ExplosionRun {
        let left = ExplosionRun {
            number: left.clone(),
            state,
        }
        .run_normal_left_explode(depth);
        let right = ExplosionRun {
            number: right.clone(),
            state: left.state.clone(),
        }
        .run_normal_right_explode(depth);
        let left = left.rerun_left_to_assign_if_needed(depth, right.state);
        ExplosionRun {
            number: SnailNumber::Tuple(Box::new(left.number), Box::new(right.number)),
            state: left.state,
        }
    }

    fn assign_number(&self, val: usize, to_assign: usize) -> ExplosionRun {
        ExplosionRun {
            number: SnailNumber::Literal(val + to_assign),
            state: ExplosionState::AllExploded,
        }
    }

    fn handle_tuple_no_explosion(
        &self,
        left: &SnailNumber,
        right: &SnailNumber,
        depth: usize,
    ) -> ExplosionRun {
        if depth > 3 && (left.is_literal_pair() || right.is_literal_pair()) {
            if left.is_literal_pair() {
                let (left_to_copy, right_to_copy) = left.get_tuple_vals();
                let new_val = ExplosionRun {
                    number: right.clone(),
                    state: ExplosionState::ToAssignRight(right_to_copy),
                }
                .find_explode_point(depth + 1)
                .number;
                ExplosionRun {
                    number: SnailNumber::Tuple(
                        Box::new(SnailNumber::Literal(0)),
                        Box::new(new_val),
                    ),
                    state: ExplosionState::ToAssignLeft(left_to_copy),
                }
            } else {
                let (left_to_copy, right_to_copy) = right.get_tuple_vals();
                let new_val = ExplosionRun {
                    number: left.clone(),
                    state: ExplosionState::ToAssignLeft(left_to_copy),
                }
                .find_explode_point(depth + 1)
                .number;
                ExplosionRun {
                    number: SnailNumber::Tuple(
                        Box::new(new_val),
                        Box::new(SnailNumber::Literal(0)),
                    ),
                    state: ExplosionState::ToAssignRight(right_to_copy),
                }
            }
        } else {
            ExplosionRun::handle_non_explode_tuples(left, right, ExplosionState::NothingDone, depth)
        }
    }

    fn handle_to_assign_left(
        left: &SnailNumber,
        right: &SnailNumber,
        depth: usize,
        assign_val: usize,
    ) -> ExplosionRun {
        let right = ExplosionRun {
            number: right.clone(),
            state: ExplosionState::ToAssignLeft(assign_val),
        }
        .find_explode_point(depth + 1)
        .number;
        let left = ExplosionRun {
            number: left.clone(),
            state: ExplosionState::AllExploded,
        }
        .find_explode_point(depth + 1)
        .number;
        ExplosionRun {
            number: SnailNumber::Tuple(Box::new(left), Box::new(right)),
            state: ExplosionState::AllExploded,
        }
    }

    fn handle_to_assign_right(
        left: &SnailNumber,
        right: &SnailNumber,
        depth: usize,
        assign_val: usize,
    ) -> ExplosionRun {
        let left = ExplosionRun {
            number: left.clone(),
            state: ExplosionState::ToAssignRight(assign_val),
        }
        .find_explode_point(depth + 1)
        .number;
        let right = ExplosionRun {
            number: right.clone(),
            state: ExplosionState::AllExploded,
        }
        .find_explode_point(depth + 1)
        .number;
        ExplosionRun {
            number: SnailNumber::Tuple(Box::new(left), Box::new(right)),
            state: ExplosionState::AllExploded,
        }
    }

    fn find_explode_point(&self, depth: usize) -> ExplosionRun {
        match (self.number.clone(), self.state.clone()) {
            (SnailNumber::Literal(val), ExplosionState::ToAssignLeft(to_assign)) => {
                self.assign_number(val, to_assign)
            }
            (SnailNumber::Literal(val), ExplosionState::ToAssignRight(to_assign)) => {
                self.assign_number(val, to_assign)
            }
            (SnailNumber::Literal(_), _) => self.clone(),
            (SnailNumber::Tuple(left, right), ExplosionState::NothingDone) => {
                self.handle_tuple_no_explosion(&left, &right, depth)
            }
            (SnailNumber::Tuple(left, right), ExplosionState::ToAssignLeft(assign_val)) => {
                ExplosionRun::handle_to_assign_left(&left, &right, depth, assign_val)
            }
            (SnailNumber::Tuple(left, right), ExplosionState::ToAssignRight(assign_val)) => {
                ExplosionRun::handle_to_assign_right(&left, &right, depth, assign_val)
            }
            (SnailNumber::Tuple(_, _), _) => self.clone(),
        }
    }
}

impl SnailNumber {
    fn new(input_str: &str) -> SnailNumber {
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

    fn run_explosion(&self) -> (SnailNumber, bool) {
        let explosion_run = ExplosionRun {
            number: self.clone(),
            state: ExplosionState::NothingDone,
        }
        .find_explode_point(1);
        (
            explosion_run.number,
            explosion_run.state != ExplosionState::NothingDone,
        )
    }

    fn run_split(&self, has_split: bool) -> (SnailNumber, bool) {
        if has_split {
            (self.clone(), has_split)
        } else {
            match self {
                SnailNumber::Literal(val) => {
                    if *val >= 10 {
                        (
                            SnailNumber::Tuple(
                                Box::new(SnailNumber::Literal(val / 2)),
                                Box::new(SnailNumber::Literal(val / 2 + val % 2)),
                            ),
                            true,
                        )
                    } else {
                        (SnailNumber::Literal(*val), has_split)
                    }
                }
                SnailNumber::Tuple(left, right) => {
                    let (left, has_split) = left.run_split(has_split);
                    let (right, has_split) = right.run_split(has_split);
                    (
                        SnailNumber::Tuple(Box::new(left), Box::new(right)),
                        has_split,
                    )
                }
            }
        }
    }

    fn reduce_once(&self) -> (SnailNumber, bool) {
        let (next, exploded) = self.run_explosion();
        if !exploded {
            next.run_split(false)
        } else {
            (next, true)
        }
    }

    fn reduce(&self) -> SnailNumber {
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
            .reduce(|a, b| SnailNumber::Tuple(Box::new(a), Box::new(b)).reduce())
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
                SnailNumber::Tuple(Box::new(a.clone()), Box::new(b.clone()))
                    .reduce()
                    .calculate_magnitude()
            })
            .reduce(std::cmp::max)
            .expect("more than 1 snailline given")
    }
}

#[cfg(test)]
mod test {
    use crate::Day18Setup;
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
    fn test_run_explosion() {
        let (n, f) = SnailNumber::new("[[[[[9,8],1],2],3],4]").run_explosion();
        assert_eq!(format!("{:?}", n), String::from("[[[[0,9],2],3],4]"));
        assert!(f);
        let (n, f) = SnailNumber::new("[7,[6,[5,[4,[3,2]]]]]").run_explosion();
        assert_eq!(format!("{:?}", n), String::from("[7,[6,[5,[7,0]]]]"));
        assert!(f);
        let (n, f) = SnailNumber::new("[[6,[5,[4,[3,2]]]],1]").run_explosion();
        assert_eq!(format!("{:?}", n), String::from("[[6,[5,[7,0]]],3]"));
        assert!(f);
        let (n, f) = SnailNumber::new("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").run_explosion();
        assert_eq!(
            format!("{:?}", n),
            String::from("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")
        );
        assert!(f);
        let (n, f) = SnailNumber::new("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").run_explosion();
        assert_eq!(
            format!("{:?}", n),
            String::from("[[3,[2,[8,0]]],[9,[5,[7,0]]]]")
        );
        assert!(f);
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(
            SnailNumber::new("[[1,2],[[3,4],5]]").calculate_magnitude(),
            143
        );
        assert_eq!(
            SnailNumber::new("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").calculate_magnitude(),
            1384
        );
        assert_eq!(
            SnailNumber::new("[[[[1,1],[2,2]],[3,3]],[4,4]]").calculate_magnitude(),
            445
        );
        assert_eq!(
            SnailNumber::new("[[[[3,0],[5,3]],[4,4]],[5,5]]").calculate_magnitude(),
            791
        );
        assert_eq!(
            SnailNumber::new("[[[[5,0],[7,4]],[5,5]],[6,6]]").calculate_magnitude(),
            1137
        );
        assert_eq!(
            SnailNumber::new("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
                .calculate_magnitude(),
            3488
        );
    }

    #[test]
    fn test_reduce_steps() {
        let next = SnailNumber::new("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        let (next, has_split) = next.reduce_once();
        assert!(has_split);
        assert_eq!(
            format!("{:?}", next),
            String::from("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]")
        );
        let (next, has_split) = next.reduce_once();
        assert!(has_split);
        assert_eq!(
            format!("{:?}", next),
            String::from("[[[[0,7],4],[15,[0,13]]],[1,1]]")
        );
        let (next, has_split) = next.reduce_once();
        assert!(has_split);
        assert_eq!(
            format!("{:?}", next),
            String::from("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]")
        );
        let (next, has_split) = next.reduce_once();
        assert!(has_split);
        assert_eq!(
            format!("{:?}", next),
            String::from("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]")
        );
        let (next, has_split) = next.reduce_once();
        assert!(has_split);
        assert_eq!(
            format!("{:?}", next),
            String::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
        );
        let (next, has_split) = next.reduce_once();
        assert!(!has_split);
        assert_eq!(
            format!("{:?}", next),
            String::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
        );
    }

    #[test]
    fn test_reduce() {
        assert_eq!(
            format!(
                "{:?}",
                SnailNumber::new("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]").reduce()
            ),
            String::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
        );
    }

    #[test]
    fn test_day_b() {
        let day18_setup = Day18Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day18_setup.calculate_day_b(), 3993);
    }
}
