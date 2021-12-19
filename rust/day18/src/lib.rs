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
    fn run_normal_left_explode(&mut self, left: &SnailNumber, depth: usize) -> SnailNumber {
        let original_explosion_state = self.state.clone();
        let explosion_state_to_use = match self.state.clone() {
            ExplosionState::ToAssignRight(_) => ExplosionState::AllExploded,
            state => state,
        };
        self.state = explosion_state_to_use.clone();
        let ret = self.find_explode_point(left, depth + 1);
        if explosion_state_to_use != original_explosion_state {
            self.state = original_explosion_state;
        }
        ret
    }

    fn run_normal_right_explode(&mut self, right: &SnailNumber, depth: usize) -> SnailNumber {
        let original_explosion_state = self.state.clone();
        let explosion_state_to_use = match self.state.clone() {
            ExplosionState::ToAssignLeft(_) => ExplosionState::AllExploded,
            state => state,
        };
        self.state = explosion_state_to_use.clone();
        let ret = self.find_explode_point(right, depth + 1);
        if explosion_state_to_use != original_explosion_state {
            self.state = original_explosion_state;
        }
        ret
    }

    fn rerun_left_to_assign_if_needed(
        &mut self,
        left: SnailNumber,
        depth: usize,
        left_state: ExplosionState,
    ) -> SnailNumber {
        match (self.state.clone(), left_state) {
            (_, ExplosionState::ToAssignLeft(_)) => left,

            (ExplosionState::ToAssignLeft(_), _) => self.find_explode_point(&left, depth + 1),
            _ => left,
        }
    }

    fn handle_non_explode_tuples(
        &mut self,
        left: &SnailNumber,
        right: &SnailNumber,
        depth: usize,
    ) -> SnailNumber {
        let left = self.run_normal_left_explode(left, depth);
        let left_state = self.state.clone();
        let right = self.run_normal_right_explode(right, depth);
        let left = self.rerun_left_to_assign_if_needed(left, depth, left_state);
        SnailNumber::Tuple(Box::new(left), Box::new(right))
    }

    fn assign_number(&mut self, val: usize, to_assign: usize) -> SnailNumber {
        self.state = ExplosionState::AllExploded;
        SnailNumber::Literal(val + to_assign)
    }

    fn handle_tuple_no_explosion(
        &mut self,
        left: &SnailNumber,
        right: &SnailNumber,
        depth: usize,
    ) -> SnailNumber {
        if depth > 3 && (left.is_literal_pair() || right.is_literal_pair()) {
            if left.is_literal_pair() {
                let (left_to_copy, right_to_copy) = left.get_tuple_vals();
                let new_val = ExplosionRun {
                    state: ExplosionState::ToAssignRight(right_to_copy),
                }
                .find_explode_point(right, depth + 1);
                self.state = ExplosionState::ToAssignLeft(left_to_copy);
                SnailNumber::Tuple(Box::new(SnailNumber::Literal(0)), Box::new(new_val))
            } else {
                let (left_to_copy, right_to_copy) = right.get_tuple_vals();
                let new_val = ExplosionRun {
                    state: ExplosionState::ToAssignLeft(left_to_copy),
                }
                .find_explode_point(left, depth + 1);
                self.state = ExplosionState::ToAssignRight(right_to_copy);
                SnailNumber::Tuple(Box::new(new_val), Box::new(SnailNumber::Literal(0)))
            }
        } else {
            self.handle_non_explode_tuples(left, right, depth)
        }
    }

    fn handle_to_assign_left(
        &mut self,
        left: &SnailNumber,
        right: &SnailNumber,
        depth: usize,
        assign_val: usize,
    ) -> SnailNumber {
        self.state = ExplosionState::ToAssignLeft(assign_val);
        let right = self.find_explode_point(right, depth + 1);
        self.state = ExplosionState::AllExploded;
        let left = self.find_explode_point(left, depth + 1);
        SnailNumber::Tuple(Box::new(left), Box::new(right))
    }

    fn handle_to_assign_right(
        &mut self,
        left: &SnailNumber,
        right: &SnailNumber,
        depth: usize,
        assign_val: usize,
    ) -> SnailNumber {
        self.state = ExplosionState::ToAssignRight(assign_val);
        let left = self.find_explode_point(left, depth + 1);
        self.state = ExplosionState::AllExploded;
        let right = self.find_explode_point(right, depth + 1);
        SnailNumber::Tuple(Box::new(left), Box::new(right))
    }

    fn find_explode_point(&mut self, number: &SnailNumber, depth: usize) -> SnailNumber {
        match (number, self.state.clone()) {
            (&SnailNumber::Literal(val), ExplosionState::ToAssignLeft(to_assign)) => {
                self.assign_number(val, to_assign)
            }
            (&SnailNumber::Literal(val), ExplosionState::ToAssignRight(to_assign)) => {
                self.assign_number(val, to_assign)
            }
            (&SnailNumber::Literal(_), _) => number.clone(),
            (SnailNumber::Tuple(left, right), ExplosionState::NothingDone) => {
                self.handle_tuple_no_explosion(left, right, depth)
            }
            (SnailNumber::Tuple(left, right), ExplosionState::ToAssignLeft(assign_val)) => {
                self.handle_to_assign_left(left, right, depth, assign_val)
            }
            (SnailNumber::Tuple(left, right), ExplosionState::ToAssignRight(assign_val)) => {
                self.handle_to_assign_right(left, right, depth, assign_val)
            }
            (&SnailNumber::Tuple(_, _), _) => number.clone(),
        }
    }
}

impl SnailNumber {
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
        let mut explosion_run = ExplosionRun {
            state: ExplosionState::NothingDone,
        };
        let ret = explosion_run.find_explode_point(self, 1);
        (ret, explosion_run.state != ExplosionState::NothingDone)
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
        let (n, f) = SnailNumber::_new("[[[[[9,8],1],2],3],4]").run_explosion();
        assert_eq!(format!("{:?}", n), String::from("[[[[0,9],2],3],4]"));
        assert!(f);
        let (n, f) = SnailNumber::_new("[7,[6,[5,[4,[3,2]]]]]").run_explosion();
        assert_eq!(format!("{:?}", n), String::from("[7,[6,[5,[7,0]]]]"));
        assert!(f);
        let (n, f) = SnailNumber::_new("[[6,[5,[4,[3,2]]]],1]").run_explosion();
        assert_eq!(format!("{:?}", n), String::from("[[6,[5,[7,0]]],3]"));
        assert!(f);
        let (n, f) = SnailNumber::_new("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").run_explosion();
        assert_eq!(
            format!("{:?}", n),
            String::from("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")
        );
        assert!(f);
        let (n, f) = SnailNumber::_new("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").run_explosion();
        assert_eq!(
            format!("{:?}", n),
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
        let next = SnailNumber::_new("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
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
                SnailNumber::_new("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]").reduce()
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
