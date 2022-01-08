use crate::amphipod::Amphipod;
use crate::point::Point;

#[derive(Debug, Clone, PartialEq)]
pub struct AmphipodMover {
    locations: [Option<Amphipod>; 7],
    stack_a: Vec<Option<Amphipod>>,
    stack_b: Vec<Option<Amphipod>>,
    stack_c: Vec<Option<Amphipod>>,
    stack_d: Vec<Option<Amphipod>>,
    best_distance: Option<usize>,
}

pub fn new_amphipod_mover(
    stack_a: Vec<Option<Amphipod>>,
    stack_b: Vec<Option<Amphipod>>,
    stack_c: Vec<Option<Amphipod>>,
    stack_d: Vec<Option<Amphipod>>,
) -> AmphipodMover {
    AmphipodMover {
        locations: [None; 7],
        stack_a,
        stack_b,
        stack_c,
        stack_d,
        best_distance: None,
    }
}

impl AmphipodMover {
    fn get_point_at_location(i: usize) -> Point {
        match i {
            0 => Point::new(0, 0),
            1 => Point::new(1, 0),
            2 => Point::new(3, 0),
            3 => Point::new(5, 0),
            4 => Point::new(7, 0),
            5 => Point::new(9, 0),
            6 => Point::new(10, 0),
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
        let y = self.get_stack_for_amphipod(amph).len() - i;
        Point::new(x, y)
    }

    fn is_amph_stack_finished(&self, amph_stack: &Amphipod) -> bool {
        self.get_stack_for_amphipod(amph_stack)
            .iter()
            .all(|amph| amph == &Some(*amph_stack))
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
                println!("Setting new best_distance: {}", distance_spent);
            }
        } else {
            self.best_distance = Some(distance_spent);
            println!("Setting new best_distance: {}", distance_spent);
        }
    }

    fn set_stack_at_location_to_null(&mut self, location: &Point) {
        let amph = Amphipod::from_x_location(location.x);
        let amph_stack = self.get_stack_for_amphipod(&amph);
        let amph_index = amph_stack.len() - location.y;
        self.get_mut_stack_for_amphipod(&amph)[amph_index] = None;
    }

    fn stack_is_only_none_or_self(&self, amph_stack: &Amphipod) -> bool {
        self.get_stack_for_amphipod(amph_stack)
            .iter()
            .all(|val| val.is_none() || val == &Some(*amph_stack))
    }

    fn can_move_amph_to_its_stack(&self, i: usize) -> bool {
        if let Some(amph) = self.locations[i] {
            if self.stack_is_only_none_or_self(&amph) {
                let point = AmphipodMover::get_point_at_location(i);
                let target_x = amph.get_x_for_stack();
                let points_to_consider = if point.x > target_x {
                    (0..self.locations.len())
                        .map(|i| (i, AmphipodMover::get_point_at_location(i)))
                        .filter(|(_, p)| p.x < point.x && p.x > target_x)
                        .map(|(i, _)| i)
                        .collect::<Vec<usize>>()
                } else {
                    (0..self.locations.len())
                        .map(|i| (i, AmphipodMover::get_point_at_location(i)))
                        .filter(|(_, p)| p.x > point.x && p.x < target_x)
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
        let index = self.set_amph_to_its_stack_and_get_index(amph);
        self.get_point_for_stack(amph, index)
    }

    fn move_to_stack_from_location(&mut self, i: usize) -> (Point, Amphipod, usize) {
        let old_amphipod = self.locations[i].expect("Already checked is Some at this point");
        let new_location = self.set_amphipod_in_stack(&old_amphipod);
        self.locations[i] = None;
        let extra_distance = new_location
            .manhattan_distance(&AmphipodMover::get_point_at_location(i))
            * old_amphipod.get_energy_per_step();
        (new_location, old_amphipod, extra_distance)
    }

    fn try_move_from_location(&mut self, i: usize, distance_spent: usize) {
        if self.can_move_amph_to_its_stack(i) && self.locations[i].is_some() {
            let (new_location, old_amphipod, extra_distance) = self.move_to_stack_from_location(i);
            self.dfs_find_amphipod_result(distance_spent + extra_distance);
            // cleanup
            self.locations[i] = Some(old_amphipod);
            self.set_stack_at_location_to_null(&new_location);
        }
    }

    fn get_locations_amph_stack_can_move_to(&self, amph_stack: &Amphipod) -> Vec<usize> {
        let x = amph_stack.get_x_for_stack();
        let mut ret = vec![];
        let points = (0..self.locations.len())
            .map(|i| (i, AmphipodMover::get_point_at_location(i)))
            .collect::<Vec<(usize, Point)>>();
        let mut points_left = points
            .iter()
            .filter(|(_, p)| p.x < x)
            .map(|(i, _)| *i)
            .collect::<Vec<usize>>();
        points_left.sort_unstable();
        points_left.reverse();
        let mut points_right = points
            .iter()
            .filter(|(_, p)| p.x > x)
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

    fn get_top_of_stack(&self, amph_stack: &Amphipod) -> Option<Amphipod> {
        let stack = self.get_stack_for_amphipod(amph_stack);
        for i in (0..stack.len()).rev() {
            if stack[i].is_some() {
                return stack[i];
            }
        }
        None
    }

    fn can_move_from_amph_stack(&self, amph_stack: &Amphipod) -> bool {
        let top = self.get_top_of_stack(amph_stack);
        !self.stack_is_only_none_or_self(amph_stack)
            && top != None
            && !self
                .get_locations_amph_stack_can_move_to(amph_stack)
                .is_empty()
    }

    fn set_amph_null_and_get_index_and_old(&mut self, amph_stack: &Amphipod) -> (usize, Amphipod) {
        let amph_stack = self.get_mut_stack_for_amphipod(amph_stack);
        let mut amph_index = amph_stack.len() - 1;
        while amph_stack[amph_index].is_none() {
            if amph_index == 0 {
                unreachable!();
            } else {
                amph_index -= 1;
            }
        }
        let amph = amph_stack[amph_index];
        amph_stack[amph_index] = None;
        (
            amph_index,
            amph.expect("Have checked the index is Some by this point"),
        )
    }

    fn set_amph_to_its_stack_and_get_index(&mut self, amph: &Amphipod) -> usize {
        let amph_stack = self.get_mut_stack_for_amphipod(amph);
        let mut amph_index = 0;
        while amph_stack[amph_index].is_some() {
            if amph_index >= amph_stack.len() - 1 {
                unreachable!();
            } else {
                amph_index += 1;
            }
        }
        amph_stack[amph_index] = Some(*amph);
        amph_index
    }

    fn move_from_amph_stack_to_location(
        &mut self,
        amph_stack: &Amphipod,
        i: usize,
    ) -> (usize, Amphipod, usize) {
        let (amph_index, amph) = self.set_amph_null_and_get_index_and_old(amph_stack);
        self.locations[i] = Some(amph);
        let location = AmphipodMover::get_point_at_location(i);
        let point = self.get_point_for_stack(amph_stack, amph_index);
        let extra_distance = location.manhattan_distance(&point) * amph.get_energy_per_step();
        (amph_index, amph, extra_distance)
    }

    fn move_from_amph_stack(&mut self, amph_stack: &Amphipod, distance_spent: usize) {
        if self.can_move_from_amph_stack(amph_stack) {
            for i in self.get_locations_amph_stack_can_move_to(amph_stack) {
                let (amph_index, amph, extra_distance) =
                    self.move_from_amph_stack_to_location(amph_stack, i);
                self.dfs_find_amphipod_result(distance_spent + extra_distance);
                //cleanup
                self.locations[i] = None;
                self.get_mut_stack_for_amphipod(amph_stack)[amph_index] = Some(amph);
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

    fn add_day_b_entries(&mut self) {
        self.stack_a = vec![
            self.stack_a[0],
            Some(Amphipod::Desert),
            Some(Amphipod::Desert),
            self.stack_a[1],
        ];
        self.stack_b = vec![
            self.stack_b[0],
            Some(Amphipod::Bronze),
            Some(Amphipod::Copper),
            self.stack_b[1],
        ];
        self.stack_c = vec![
            self.stack_c[0],
            Some(Amphipod::Amber),
            Some(Amphipod::Bronze),
            self.stack_c[1],
        ];
        self.stack_d = vec![
            self.stack_d[0],
            Some(Amphipod::Copper),
            Some(Amphipod::Amber),
            self.stack_d[1],
        ];
    }

    /// Calculate the part b response
    pub fn calculate_day_b(self: &mut AmphipodMover) -> usize {
        self.add_day_b_entries();
        self.dfs_find_amphipod_result(0);
        self.best_distance
            .expect("Should have found a distance that gets to the goal")
    }
}

#[cfg(test)]
mod test {
    use crate::amphipod::Amphipod;
    use crate::AmphipodMover;

    #[test]
    fn test_parse() {
        let day23_setup = AmphipodMover::new(include_str!("../test_data.txt"));
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
    fn test_day_a_steps() {
        let mut day23_setup = AmphipodMover::new(include_str!("../test_data.txt"));
        // Move B from stack C to position 2
        assert!(day23_setup.can_move_from_amph_stack(&Amphipod::Copper));
        assert!(day23_setup
            .get_locations_amph_stack_can_move_to(&Amphipod::Copper)
            .contains(&2));
        day23_setup.move_from_amph_stack_to_location(&Amphipod::Copper, 2);
        // Move C from stack B to Stack C
        assert!(day23_setup.can_move_from_amph_stack(&Amphipod::Bronze));
        assert!(day23_setup
            .get_locations_amph_stack_can_move_to(&Amphipod::Bronze)
            .contains(&3));
        day23_setup.move_from_amph_stack_to_location(&Amphipod::Bronze, 3);
        assert!(day23_setup.can_move_amph_to_its_stack(3));
        day23_setup.move_to_stack_from_location(3);
        //Move D from stack B to position 3
        assert!(day23_setup.can_move_from_amph_stack(&Amphipod::Bronze));
        assert!(day23_setup
            .get_locations_amph_stack_can_move_to(&Amphipod::Bronze)
            .contains(&3));
        day23_setup.move_from_amph_stack_to_location(&Amphipod::Bronze, 3);
        //Move B from position 2 to stack B
        assert!(day23_setup.can_move_amph_to_its_stack(2));
        day23_setup.move_to_stack_from_location(2);
        //Move B from stack A to stack B
        assert!(day23_setup.can_move_from_amph_stack(&Amphipod::Amber));
        assert!(day23_setup
            .get_locations_amph_stack_can_move_to(&Amphipod::Amber)
            .contains(&2));
        day23_setup.move_from_amph_stack_to_location(&Amphipod::Amber, 2);
        assert!(day23_setup.can_move_amph_to_its_stack(2));
        day23_setup.move_to_stack_from_location(2);
        //Move D from stack D to position 4
        assert!(day23_setup.can_move_from_amph_stack(&Amphipod::Desert));
        assert!(day23_setup
            .get_locations_amph_stack_can_move_to(&Amphipod::Desert)
            .contains(&4));
        day23_setup.move_from_amph_stack_to_location(&Amphipod::Desert, 4);
        //Move A from stack A to position 5
        assert!(day23_setup.can_move_from_amph_stack(&Amphipod::Desert));
        assert!(day23_setup
            .get_locations_amph_stack_can_move_to(&Amphipod::Desert)
            .contains(&5));
        day23_setup.move_from_amph_stack_to_location(&Amphipod::Desert, 5);
        //Move D from position 4 to stack D
        assert!(day23_setup.can_move_amph_to_its_stack(4));
        day23_setup.move_to_stack_from_location(4);
        //Move D from position 3 to stack D
        assert!(day23_setup.can_move_amph_to_its_stack(3));
        day23_setup.move_to_stack_from_location(3);
        //Move A from position 5 to stack A
        assert!(day23_setup.can_move_amph_to_its_stack(5));
        day23_setup.move_to_stack_from_location(5);
        // Check is finished
        assert!(day23_setup.finished());
    }

    #[test]
    fn test_day_a() {
        let mut day23_setup = AmphipodMover::new(include_str!("../test_data.txt"));
        assert_eq!(day23_setup.calculate_day_a(), 12521);
    }

    #[test]
    fn test_day_b_steps() {
        let mut day23_setup = AmphipodMover::new(include_str!("../test_data.txt"));
        day23_setup.add_day_b_entries();
        // Move D from stack D to position 6
        assert!(day23_setup.can_move_from_amph_stack(&Amphipod::Desert));
        assert!(day23_setup
            .get_locations_amph_stack_can_move_to(&Amphipod::Desert)
            .contains(&6));
        day23_setup.move_from_amph_stack_to_location(&Amphipod::Desert, 6);
        // Move A from Stack D to position 0
        assert!(day23_setup.can_move_from_amph_stack(&Amphipod::Desert));
        assert!(day23_setup
            .get_locations_amph_stack_can_move_to(&Amphipod::Desert)
            .contains(&0));
        day23_setup.move_from_amph_stack_to_location(&Amphipod::Desert, 0);
        // Move B from Stack C to position 5
        assert!(day23_setup.can_move_from_amph_stack(&Amphipod::Copper));
        assert!(day23_setup
            .get_locations_amph_stack_can_move_to(&Amphipod::Copper)
            .contains(&5));
        day23_setup.move_from_amph_stack_to_location(&Amphipod::Copper, 5);
        // Move B from Stack C to position 4
        assert!(day23_setup.can_move_from_amph_stack(&Amphipod::Copper));
        assert!(day23_setup
            .get_locations_amph_stack_can_move_to(&Amphipod::Copper)
            .contains(&4));
        day23_setup.move_from_amph_stack_to_location(&Amphipod::Copper, 4);
        // Move A from Stack C to position 1
        assert!(day23_setup.can_move_from_amph_stack(&Amphipod::Copper));
        assert!(day23_setup
            .get_locations_amph_stack_can_move_to(&Amphipod::Copper)
            .contains(&1));
        day23_setup.move_from_amph_stack_to_location(&Amphipod::Copper, 1);
        // Move C from stack B to Stack C (x2)
        assert!(day23_setup.can_move_from_amph_stack(&Amphipod::Bronze));
        assert!(day23_setup
            .get_locations_amph_stack_can_move_to(&Amphipod::Bronze)
            .contains(&3));
        day23_setup.move_from_amph_stack_to_location(&Amphipod::Bronze, 3);
        assert!(day23_setup.can_move_amph_to_its_stack(3));
        day23_setup.move_to_stack_from_location(3);
        assert!(day23_setup.can_move_from_amph_stack(&Amphipod::Bronze));
        assert!(day23_setup
            .get_locations_amph_stack_can_move_to(&Amphipod::Bronze)
            .contains(&3));
        day23_setup.move_from_amph_stack_to_location(&Amphipod::Bronze, 3);
        assert!(day23_setup.can_move_amph_to_its_stack(3));
        day23_setup.move_to_stack_from_location(3);
        // Move B from Stack B to position 3
        assert!(day23_setup.can_move_from_amph_stack(&Amphipod::Bronze));
        assert!(day23_setup
            .get_locations_amph_stack_can_move_to(&Amphipod::Bronze)
            .contains(&3));
        day23_setup.move_from_amph_stack_to_location(&Amphipod::Bronze, 3);
        // Move D from stack B to position 2
        assert!(day23_setup.can_move_from_amph_stack(&Amphipod::Bronze));
        assert!(day23_setup
            .get_locations_amph_stack_can_move_to(&Amphipod::Bronze)
            .contains(&2));
        day23_setup.move_from_amph_stack_to_location(&Amphipod::Bronze, 2);
    }

    #[test]
    fn test_day_b() {
        let mut day23_setup = AmphipodMover::new(include_str!("../test_data.txt"));
        assert_eq!(day23_setup.calculate_day_b(), 44169);
    }
}
