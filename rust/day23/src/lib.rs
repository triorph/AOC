extern crate peg;

pub struct Day23Setup {
    locations: [Option<Amphipod>; 7],
    stack_a: [Option<Amphipod>; 2],
    stack_b: [Option<Amphipod>; 2],
    stack_c: [Option<Amphipod>; 2],
    stack_d: [Option<Amphipod>; 2],
    best_distance: Option<usize>,
}

#[derive(Clone, PartialOrd, PartialEq, Ord, Eq)]
struct Point(usize, usize);

#[derive(Copy, Clone, Debug, PartialEq)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
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

impl Amphipod {
    fn get_energy_per_step(&self) -> usize {
        match self {
            Amphipod::Amber => 1,
            Amphipod::Bronze => 10,
            Amphipod::Copper => 100,
            Amphipod::Desert => 1000,
        }
    }

    fn get_x_for_stack(&self) -> usize {
        match self {
            Amphipod::Amber => 2,
            Amphipod::Bronze => 4,
            Amphipod::Copper => 6,
            Amphipod::Desert => 8,
        }
    }

    fn from_x_location(x: usize) -> Amphipod {
        match x {
            2 => Amphipod::Amber,
            4 => Amphipod::Bronze,
            6 => Amphipod::Copper,
            8 => Amphipod::Desert,
            _ => unreachable!(),
        }
    }

    fn all() -> [Amphipod; 4] {
        [
            Amphipod::Amber,
            Amphipod::Bronze,
            Amphipod::Copper,
            Amphipod::Desert,
        ]
    }
}

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

    fn get_mut_stack_for_amphipod(&mut self, amph: &Amphipod) -> &mut [Option<Amphipod>] {
        match amph {
            Amphipod::Amber => &mut self.stack_a,
            Amphipod::Bronze => &mut self.stack_b,
            Amphipod::Copper => &mut self.stack_c,
            Amphipod::Desert => &mut self.stack_d,
        }
    }

    fn get_stack_for_amphipod(&self, amph: &Amphipod) -> &[Option<Amphipod>] {
        match amph {
            Amphipod::Amber => &self.stack_a,
            Amphipod::Bronze => &self.stack_b,
            Amphipod::Copper => &self.stack_c,
            Amphipod::Desert => &self.stack_d,
        }
    }

    fn get_point_for_stack(&self, amph: &Amphipod, i: usize) -> Point {
        let x = amph.get_x_for_stack();
        let y = match i {
            // i is allowed to be 0 or 1, to be either bottom or top of its stack.
            0 => 2,
            1 => 1,
            _ => unreachable!(),
        };
        Point(x, y)
    }

    fn is_amph_stack_finished(&self, amph_stack: &Amphipod) -> bool {
        self.get_stack_for_amphipod(amph_stack) == [Some(*amph_stack), Some(*amph_stack)]
    }

    fn is_amph_stack_empty(&self, amph_stack: &Amphipod) -> bool {
        self.get_stack_for_amphipod(amph_stack) == [None, None]
    }

    fn finished(&self) -> bool {
        Amphipod::all()
            .iter()
            .all(|amph_stack| self.is_amph_stack_finished(amph_stack))
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

    fn set_stack_at_location_to_null(&mut self, location: &Point) {
        let amph = Amphipod::from_x_location(location.0);
        let amph_index = match location.1 {
            1 => 1,
            2 => 0,
            _ => unreachable!(),
        };
        self.get_mut_stack_for_amphipod(&amph)[amph_index] = None;
    }

    fn stack_can_be_moved_to(&self, amph_stack: &Amphipod) -> bool {
        self.get_stack_for_amphipod(amph_stack)
            .iter()
            .all(|val| val.is_none() || val == &Some(*amph_stack))
    }

    fn can_move_amph_to_its_stack(&self, i: usize) -> bool {
        if let Some(amph) = self.locations[i] {
            if self.stack_can_be_moved_to(&amph) {
                let point = Day23Setup::get_point_at_location(i);
                let target_x = amph.get_x_for_stack();
                let points_to_consider = if point.0 > target_x {
                    (0..self.locations.len())
                        .map(|i| (i, Day23Setup::get_point_at_location(i)))
                        .filter(|(_, p)| p.0 < point.0 && p.0 > target_x)
                        .map(|(i, _)| i)
                        .collect::<Vec<usize>>()
                } else {
                    (0..self.locations.len())
                        .map(|i| (i, Day23Setup::get_point_at_location(i)))
                        .filter(|(_, p)| p.0 > point.0 && p.0 < target_x)
                        .map(|(i, _)| i)
                        .collect::<Vec<usize>>()
                };
                points_to_consider
                    .into_iter()
                    .all(|i| self.locations[i].is_none())
            } else {
                false
            }
        } else {
            false
        }
    }

    fn set_amphipod_in_stack(&mut self, amph: &Amphipod) -> Point {
        let stack = self.get_mut_stack_for_amphipod(amph);
        if stack[0].is_none() {
            stack[0] = Some(*amph);
            self.get_point_for_stack(amph, 0)
        } else if stack[1].is_none() {
            stack[1] = Some(*amph);
            self.get_point_for_stack(amph, 1)
        } else {
            unreachable!();
        }
    }

    fn try_move_from_location(&mut self, i: usize, distance_spent: usize) {
        if self.can_move_amph_to_its_stack(i) {
            if let Some(amph) = self.locations[i] {
                let old_amphipod = amph;
                let new_location = self.set_amphipod_in_stack(&amph);
                self.locations[i] = None;
                let extra_distance_spent = new_location
                    .manhattan_distance(&Day23Setup::get_point_at_location(i))
                    * amph.get_energy_per_step();
                self.dfs_find_amphipod_result(distance_spent + extra_distance_spent);
                // cleanup
                self.locations[i] = Some(old_amphipod);
                self.set_stack_at_location_to_null(&new_location);
            }
        }
    }

    fn get_locations_amph_stack_can_move_to(&self, amph_stack: &Amphipod) -> Vec<usize> {
        let x = amph_stack.get_x_for_stack();
        let mut ret = vec![];
        let points = (0..self.locations.len())
            .map(|i| (i, Day23Setup::get_point_at_location(i)))
            .collect::<Vec<(usize, Point)>>();
        let mut points_left = points
            .iter()
            .filter(|(_, p)| p.0 < x)
            .map(|(i, _)| *i)
            .collect::<Vec<usize>>();
        points_left.sort_unstable();
        points_left.reverse();
        let mut points_right = points
            .iter()
            .filter(|(_, p)| p.0 > x)
            .map(|(i, _)| *i)
            .collect::<Vec<usize>>();
        points_right.sort_unstable();
        for i in points_left.into_iter() {
            if self.locations[i].is_none() {
                ret.push(i);
            } else {
                break;
            }
        }
        for i in points_right.into_iter() {
            if self.locations[i].is_none() {
                ret.push(i);
            } else {
                break;
            }
        }
        ret
    }

    fn get_top_of_stack(&mut self, amph_stack: &Amphipod) -> Option<Amphipod> {
        todo!()
    }

    fn can_amph_move_directly_to_stack(&self, amph_stack: &Amphipod) -> bool {
        todo!()
    }

    fn can_move_from_amph_stack(&self, amph_stack: &Amphipod) -> bool {
        self.get_top_of_stack(amph_stack) != Some(amph_stack)
            && !self
                .get_locations_amph_stack_can_move_to(amph_stack)
                .is_empty()
            && !self.can_amph_move_directly_to_stack(amph_stack)
    }

    fn move_from_amph_stack(&mut self, amph_stack: &Amphipod, distance_spent: usize) {
        if self.can_move_from_amph_stack(amph_stack) {
            todo!(); // try moving to other amph stacks directly
            for i in self.get_locations_amph_stack_can_move_to(amph_stack) {
                let (amph_index, amph) = {
                    let amph_stack = self.get_mut_stack_for_amphipod(amph_stack);
                    let amph_index = if amph_stack[1].is_some() {
                        1
                    } else if (amph_stack[0]).is_some() {
                        0
                    } else {
                        unreachable!()
                    };
                    let amph = amph_stack[amph_index];
                    amph_stack[amph_index] = None;
                    (amph_index, amph)
                };

                self.locations[i] = amph;
                todo!(); // calculate the additional distance spent in the move
                self.dfs_find_amphipod_result(distance_spent);
                //cleanup
                self.locations[i] = None;
                self.get_mut_stack_for_amphipod(amph_stack)[amph_index] = amph;
            }
        }
    }

    fn dfs_find_amphipod_result(&mut self, distance_spent: usize) {
        if self.finished() {
            self.set_finished(distance_spent);
            return;
        }
        if let Some(best_distance) = self.best_distance {
            if distance_spent > best_distance {
                return;
            }
        }
        for i in 0..self.locations.len() {
            if self.locations[i].is_some() {
                self.try_move_from_location(i, distance_spent);
            }
        }
        for amph_stack in Amphipod::all() {
            self.move_from_amph_stack(&amph_stack, distance_spent);
        }
    }

    /// Calculate the part a response
    pub fn calculate_day_a(&mut self) -> usize {
        self.dfs_find_amphipod_result(0);
        self.best_distance
            .expect("Should have found a distance that gets to the goal")
    }

    /// Calculate the part b response
    pub fn calculate_day_b(self: &mut Day23Setup) -> usize {
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
        let mut day23_setup = Day23Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day23_setup.calculate_day_a(), 12521);
    }

    #[test]
    fn test_day_b() {
        let mut day23_setup = Day23Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day23_setup.calculate_day_b(), 0);
    }
}
