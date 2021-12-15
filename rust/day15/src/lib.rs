extern crate peg;
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct Point(isize, isize);

#[derive(Clone, Debug)]
struct PathExplore {
    cost: usize,
    current_location: Point,
}

pub struct Day15Setup {
    lines: Vec<Vec<usize>>,
    shortest: HashMap<Point, Option<usize>>,
}

peg::parser! { grammar day15_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule line() -> Vec<usize>
        = line:number() ++ "" { line }
    rule lines() -> Vec<Vec<usize>>
        = lines:line() ++ "\n" { lines }
    pub rule parse() -> Day15Setup
        = lines:lines() "\n" * {
            Day15Setup { lines, shortest: HashMap::<Point, Option<usize>>::new()}
        }
}}

impl Point {
    fn get_neighbours(&self) -> [Point; 4] {
        [
            Point(self.0 + 1, self.1),
            Point(self.0, self.1 + 1),
            Point(self.0 - 1, self.1),
            Point(self.0, self.1 - 1),
        ]
    }
}

impl Day15Setup {
    /// Generates a new Day15Setup object to be calculated upon
    ///
    /// Inputs: the full string contents of the input data set.
    ///
    /// Returns: A new Day15Setup object, with methods `calculate_day_a` and `calculate_day_b`
    /// available
    pub fn new(input_str: &str) -> Day15Setup {
        day15_parser::parse(input_str).unwrap()
    }

    fn get_value_at_location_day_a(&self, location: &Point) -> Option<usize> {
        if location.1 >= 0
            && location.1 < self.lines.len() as isize
            && location.0 >= 0
            && location.0 < self.lines[location.1 as usize].len() as isize
        {
            Some(self.lines[location.1 as usize][location.0 as usize])
        } else {
            None
        }
    }

    fn get_value_at_location_day_b(&self, location: &Point) -> Option<usize> {
        if location.1 >= 0
            && location.1 < (self.lines.len() * 5) as isize
            && location.0 >= 0
            && location.0 < (self.lines[location.1 as usize % self.lines.len()].len() * 5) as isize
        {
            let y_mult = location.1 as usize / self.lines.len();
            let y_val = location.1 as usize % self.lines.len();
            let x_mult = location.0 as usize / self.lines[y_val].len();
            let x_val = location.0 as usize % self.lines[y_val].len();
            Some((self.lines[y_val][x_val] + y_mult + x_mult) % 10)
        } else {
            None
        }
    }

    fn get_cost_at_location(&self, location: &Point) -> &Option<usize> {
        if let Some(val) = self.shortest.get(location) {
            val
        } else {
            &None
        }
    }

    fn set_shortest(&mut self, path: &PathExplore) {
        self.shortest
            .insert(path.current_location.clone(), Some(path.cost));
    }

    fn explore_all_paths_to_target_a(&mut self) -> usize {
        let mut explores = 0;
        let first_path = PathExplore {
            cost: 0,
            current_location: Point(0, 0),
        };
        let mut paths = VecDeque::new();
        paths.push_back(first_path);
        while let Some(path) = paths.pop_front() {
            let location_cost = self.get_cost_at_location(&path.current_location);
            if location_cost.is_none() || &Some(path.cost) < location_cost {
                explores += 1;
                self.set_shortest(&path);
                let neighbours = path.current_location.get_neighbours();
                for neighbour in neighbours {
                    if let Some(value) = self.get_value_at_location_day_a(&neighbour) {
                        paths.push_back(PathExplore {
                            current_location: neighbour,
                            cost: path.cost + value,
                        })
                    }
                }
            }
        }
        let max_y = self.lines.len() - 1;
        let max_x = self.lines[max_y].len() - 1;
        println!(
            "Took {} explores to find the shortest path to all points",
            explores
        );
        self.get_cost_at_location(&Point(max_x as isize, max_y as isize))
            .unwrap()
    }

    fn explore_all_paths_to_target_b(&mut self) -> usize {
        let mut explores = 0;
        let first_path = PathExplore {
            cost: 0,
            current_location: Point(0, 0),
        };
        let mut paths = VecDeque::new();
        paths.push_back(first_path);
        while let Some(path) = paths.pop_front() {
            let location_cost = self.get_cost_at_location(&path.current_location);
            if location_cost.is_none() || &Some(path.cost) < location_cost {
                explores += 1;
                self.set_shortest(&path);
                let neighbours = path.current_location.get_neighbours();
                for neighbour in neighbours {
                    if let Some(value) = self.get_value_at_location_day_b(&neighbour) {
                        paths.push_back(PathExplore {
                            current_location: neighbour,
                            cost: path.cost + value,
                        })
                    }
                }
            }
        }
        let max_y = self.lines.len() * 5 - 1;
        let max_x = self.lines[0].len() * 5 - 1;
        println!(
            "Took {} explores to find the shortest path to all points",
            explores
        );
        println!("Getting cost at point: {} {}", max_x, max_y);

        self.get_cost_at_location(&Point(max_x as isize, max_y as isize))
            .unwrap()
    }

    /// Calculate the part a response
    pub fn calculate_day_a(&mut self) -> usize {
        self.explore_all_paths_to_target_a()
    }

    /// Calculate the part b response
    pub fn calculate_day_b(&mut self) -> usize {
        self.explore_all_paths_to_target_b()
    }
}

#[cfg(test)]
mod test {
    use crate::Day15Setup;
    use crate::Point;

    #[test]
    fn test_parse() {
        let day15_setup = Day15Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day15_setup.lines.len(), 10);
    }

    #[test]
    fn test_day_a() {
        let mut day15_setup = Day15Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day15_setup.calculate_day_a(), 40);
    }

    #[test]
    fn test_get_value_day_b() {
        let day15_setup = Day15Setup::new(include_str!("../test_data.txt"));
        assert_eq!(
            day15_setup.get_value_at_location_day_b(&Point(49, 49)),
            Some(9)
        );
        assert_eq!(
            day15_setup.get_value_at_location_day_b(&Point(29, 9)),
            Some(3)
        );
        assert_eq!(
            day15_setup.get_value_at_location_day_b(&Point(50, 49)),
            None
        );
        assert_eq!(
            day15_setup.get_value_at_location_day_b(&Point(49, 49)),
            Some(9)
        );
    }

    #[test]
    fn test_day_b() {
        let mut day15_setup = Day15Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day15_setup.calculate_day_b(), 315);
    }
}
