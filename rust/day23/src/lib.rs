extern crate peg;
use itertools::Itertools;

pub struct Day23Setup {
    locations: [Option<Amphipod>; 7],
    stack_a: [Option<Amphipod>; 2],
    stack_b: [Option<Amphipod>; 2],
    stack_c: [Option<Amphipod>; 2],
    stack_d: [Option<Amphipod>; 2],
    best_distance: Option<usize>,
}

struct Point(usize, usize);

#[derive(Copy, Clone, Debug, PartialEq)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amphipod {
    fn get_energy_per_step(&self) -> usize {
        match self {
            Amphipod::Amber => 1,
            Amphipod::Bronze => 10,
            Amphipod::Copper => 100,
            Amphipod::Desert => 1000,
        }
    }
}

peg::parser! { grammar day23_parser() for str {
    rule amber() -> Amphipod
        = "A" { Amphipod::Amber }
    rule bronze() -> Amphipod
        = "B" { Amphipod::Bronze }
    rule copper() -> Amphipod
        = "C" { Amphipod::Copper }
    rule desert() -> Amphipod
        = "D" { Amphipod::Desert }
    rule amphipod() -> Amphipod
        = amphipod:(amber() / bronze() / copper() / desert()) { amphipod }
    rule top() -> ()
        = "#############" " " * "\n" "#...........#" " " * "\n" {}
    rule bottom() -> ()
        = "  #########"
    pub rule parse() -> Day23Setup
        = top() "###" a2:amphipod() "#" b2:amphipod() "#" c2:amphipod() "#" d2:amphipod() "###" " " * "\n"
        "  #" a1:amphipod() "#" b1:amphipod() "#" c1:amphipod() "#" d1:amphipod() "#" " " * "\n" bottom()  "\n" * {
            Day23Setup {
                locations: [None; 7],
                stack_a: [Some(a1), Some(a2)],
                stack_b: [Some(b1), Some(b2)],
                stack_c: [Some(c1), Some(c2)],
                stack_d: [Some(d1), Some(d2)],
                best_distance: None
            }
        }
}}

impl Point {
    fn manhattan_distance(&self, other: &Point) -> usize {
        (self.0 as isize - other.0 as isize).abs() as usize
            + (self.1 as isize - other.1 as isize).abs() as usize
    }
}

impl Day23Setup {
    /// Generates a new Day23Setup object to be calculated upon
    ///
    /// Inputs: the full string contents of the input data set.
    ///
    /// Returns: A new Day23Setup object, with methods `calculate_day_a` and `calculate_day_b`
    /// available
    pub fn new(input_str: &str) -> Day23Setup {
        day23_parser::parse(input_str).unwrap()
    }

    fn get_point_at_location(i: usize) -> Point {
        match i {
            0 => Point(0, 0),
            1 => Point(1, 0),
            2 => Point(3, 0),
            3 => Point(5, 0),
            4 => Point(7, 0),
            5 => Point(9, 0),
            6 => Point(10, 0),
            _ => panic!("Invalid point"),
        }
    }

    fn finished(&self) -> bool {
        self.stack_a == [Some(Amphipod::Amber), Some(Amphipod::Amber)]
            && self.stack_b == [Some(Amphipod::Bronze), Some(Amphipod::Bronze)]
            && self.stack_c == [Some(Amphipod::Copper), Some(Amphipod::Copper)]
            && self.stack_d == [Some(Amphipod::Desert), Some(Amphipod::Desert)]
    }

    fn set_finished(&mut self, distance_spent: usize) {
        if let Some(other_distance) = self.best_distance {
            if distance_spent < other_distance {
                self.best_distance = Some(distance_spent);
            }
        } else {
            self.best_distance = Some(distance_spent);
        }
    }

    fn dfs_find_amphipod_setup(&mut self, distance_spent: usize) {
        if self.finished() {
            self.set_finished(distance_spent);
            return;
        }
        if let Some(best_distance) = self.best_distance {
            if distance_spent > best_distance {
                return;
            }
        }
        for (i, amphipod) in self.locations.iter_mut() {
            if let Some(amph) = amphipod {
                if self.can_move_amph_to_its_stack(amph) {
                    let new_location = self.set_amphipod_in_stack(amph);
                    *amphipod = None;
                    let extra_distance_spent =
                        new_location.manhattan_distance(&Day23Setup::get_point_at_location(i));
                    self.dfs_find_amphipod_setup(distance_spent);
                }
            }
        }
    }

    /// Calculate the part a response
    pub fn calculate_day_a(self: &Day23Setup) -> usize {
        0
    }

    /// Calculate the part b response
    pub fn calculate_day_b(self: &Day23Setup) -> usize {
        0
    }
}

#[cfg(test)]
mod test {
    use crate::Amphipod;
    use crate::Day23Setup;

    #[test]
    fn test_parse() {
        let day23_setup = Day23Setup::new(include_str!("../test_data.txt"));
        assert_eq!(
            day23_setup.stack_a,
            [Some(Amphipod::Amber), Some(Amphipod::Bronze)]
        );
        assert_eq!(
            day23_setup.stack_b,
            [Some(Amphipod::Desert), Some(Amphipod::Copper)]
        );
        assert_eq!(
            day23_setup.stack_c,
            [Some(Amphipod::Copper), Some(Amphipod::Bronze)]
        );
        assert_eq!(
            day23_setup.stack_d,
            [Some(Amphipod::Amber), Some(Amphipod::Desert)]
        );
    }

    #[test]
    fn test_day_a() {
        let day23_setup = Day23Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day23_setup.calculate_day_a(), 12521);
    }

    #[test]
    fn test_day_b() {
        let day23_setup = Day23Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day23_setup.calculate_day_b(), 0);
    }
}
