extern crate peg;
use itertools::Itertools;
use std::collections::HashSet;

#[derive(PartialEq, Hash, Eq, Clone)]
struct Point {
    x: isize,
    y: isize,
}
#[derive(PartialEq, Clone, Debug)]
enum Binary {
    One,
    Zero,
}

#[derive(Clone)]
struct Boundary {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    surrounding: Binary,
}

#[derive(Clone)]
pub struct Day20Setup {
    key_data: Vec<Binary>,
    lit_image: HashSet<Point>,
    boundary: Boundary,
}

peg::parser! { grammar day20_parser() for str {
    rule binary_one() -> Binary
        = "#" { Binary::One }
    rule binary_zero() -> Binary
        = "." { Binary::Zero }
    rule binary_value() -> Binary
        = b:(binary_one() / binary_zero()) { b }
    rule binary_lines() -> Vec<Binary>
        = b:binary_value() ++ "" { b }
    pub rule parse() -> Day20Setup
        = key_data:binary_value() **<512,512> "" "\n\n" lines:binary_lines() ++ "\n" "\n" * {
            let mut lit_image: HashSet<Point> = HashSet::new();
            let boundary = Boundary{ min_x: 0, max_x: lines[lines.len() -1].len() as isize, min_y: 0, max_y: lines.len() as isize, surrounding: Binary::Zero };
            for (y, line) in lines.iter().enumerate() {
                for (x, val) in line.iter().enumerate() {
                    if val == &Binary::One {
                        lit_image.insert(Point{x: x as isize, y: y as isize});
                    }
                }
            }
            Day20Setup {key_data, lit_image, boundary}
        }
}}

impl Point {
    fn get_all_neighbours<'a>(&'a self) -> Box<dyn Iterator<Item = Point> + 'a> {
        Box::new(
            [
                (-1, -1),
                (0, -1),
                (1, -1),
                (-1, 0),
                (0, 0),
                (1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
            ]
            .iter()
            .map(|(x, y)| Point {
                x: self.x + x,
                y: self.y + y,
            }),
        )
    }
}

impl Boundary {
    fn get_current_at_point(&self, point: &Point) -> Binary {
        if point.x < self.min_x
            || point.x >= self.max_x
            || point.y < self.min_y
            || point.y >= self.max_y
        {
            self.surrounding.clone()
        } else {
            Binary::Zero
        }
    }

    fn iter_all_points<'a>(&'a self) -> Box<dyn Iterator<Item = Point> + 'a> {
        Box::new(
            ((self.min_x - 1)..=(self.max_x + 1))
                .cartesian_product((self.min_y - 1)..=(self.max_y + 1))
                .map(|(x, y)| Point { x, y }),
        )
    }

    fn build_next_boundary(&self, key_value_at_zero: &Binary) -> Boundary {
        let surrounding = match (&self.surrounding, key_value_at_zero) {
            (&Binary::One, &Binary::One) => Binary::Zero,
            (&Binary::Zero, &Binary::One) => Binary::One,
            _ => Binary::Zero,
        };
        Boundary {
            min_x: self.min_x - 1,
            max_x: self.max_x + 1,
            min_y: self.min_y - 1,
            max_y: self.max_y + 1,
            surrounding,
        }
    }
}

impl Day20Setup {
    /// Generates a new Day20Setup object to be calculated upon
    ///
    /// Inputs: the full string contents of the input data set.
    ///
    /// Returns: A new Day20Setup object, with methods `calculate_day_a` and `calculate_day_b`
    /// available
    pub fn new(input_str: &str) -> Day20Setup {
        day20_parser::parse(input_str).unwrap()
    }

    fn get_current_at_point(&self, point: &Point) -> Binary {
        if self.lit_image.contains(point) {
            Binary::One
        } else {
            self.boundary.get_current_at_point(point)
        }
    }

    fn get_index_val(&self, point: &Point) -> usize {
        let mut index = 0;
        for p in point.get_all_neighbours() {
            index <<= 1;
            if self.get_current_at_point(&p) == Binary::One {
                index |= 1;
            }
        }
        index
    }

    fn get_next_at_point(&self, point: &Point) -> &Binary {
        let index = self.get_index_val(point);
        &self.key_data[index]
    }

    fn get_next_image(&self) -> Day20Setup {
        let mut next: HashSet<Point> = HashSet::with_capacity(self.lit_image.len());
        for point in self.boundary.iter_all_points() {
            if self.get_next_at_point(&point) == &Binary::One {
                next.insert(point);
            }
        }
        Day20Setup {
            lit_image: next,
            key_data: self.key_data.clone(),
            boundary: self.boundary.build_next_boundary(&self.key_data[0]),
        }
    }

    fn iterate_to_next_n_times(&self, n: usize) -> Day20Setup {
        let mut next = self.clone();
        for _ in 0..n {
            next = next.get_next_image();
        }
        next
    }

    /// Calculate the part a response
    ///
    /// How many squares are lit up after 2 iterations
    pub fn calculate_day_a(self: &Day20Setup) -> usize {
        self.iterate_to_next_n_times(2).lit_image.len()
    }

    /// Calculate the part b response
    ///
    /// How many squares are lit up after 50 iterations
    pub fn calculate_day_b(self: &Day20Setup) -> usize {
        self.iterate_to_next_n_times(50).lit_image.len()
    }
}

#[cfg(test)]
mod test {
    use crate::Binary;
    use crate::Day20Setup;
    use crate::Point;

    #[test]
    fn test_parse() {
        let day20_setup = Day20Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day20_setup.lit_image.len(), 10);
    }

    #[test]
    fn test_get_index_val() {
        let day20_setup = Day20Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day20_setup.get_index_val(&Point { x: 2, y: 2 }), 34);
        assert_eq!(
            day20_setup.get_next_at_point(&Point { x: 2, y: 2 }),
            &Binary::One
        );
        assert_eq!(day20_setup.get_index_val(&Point { x: -2, y: -2 }), 0);
        assert_eq!(
            day20_setup.get_next_at_point(&Point { x: -2, y: -2 }),
            &Binary::Zero
        );
    }

    #[test]
    fn test_one_iteration() {
        let day20_setup = Day20Setup::new(include_str!("../test_data.txt"));
        let next = day20_setup.get_next_image();
        assert_eq!(next.lit_image.len(), 24);
        assert!(next.lit_image.contains(&Point { x: 0, y: -1 }));
        assert!(next.lit_image.contains(&Point { x: 1, y: -1 }));
        assert!(!next.lit_image.contains(&Point { x: 2, y: -1 }));
        assert!(next.lit_image.contains(&Point { x: 3, y: -1 }));
        assert!(next.lit_image.contains(&Point { x: 4, y: -1 }));
    }

    #[test]
    fn test_day_a() {
        let day20_setup = Day20Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day20_setup.calculate_day_a(), 35);
    }

    #[test]
    fn test_day_b() {
        let day20_setup = Day20Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day20_setup.calculate_day_b(), 3351);
    }
}
