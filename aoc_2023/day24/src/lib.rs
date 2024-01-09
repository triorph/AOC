mod parser;
use crate::parser::parse_data;
use aoc_helpers::point3d::Point3D;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day24 {
    hailstones: Vec<(Point3D, Point3D)>,
}

impl AOCCalculator for Day24 {
    fn new(filename: &str) -> Result<Day24, AOCFileOrParseError> {
        Ok(Day24 {
            hailstones: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TSolve {
    All,
    None,
    Some(f64),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Slope {
    Some(f64),
    Vertical(f64),
}

fn tsolve(pos_1: isize, pos_2: isize, delta_1: isize, delta_2: isize) -> TSolve {
    let delta_diff = delta_1 - delta_2;
    let pos_diff = pos_1 - pos_2;
    if delta_diff == 0 {
        if pos_diff == 0 {
            return TSolve::All;
        } else {
            return TSolve::None;
        }
    }
    TSolve::Some(-pos_diff as f64 / delta_diff as f64)
}

fn get_dy(delta: &Point3D) -> Slope {
    if delta.y == 0 {
        Slope::Vertical(delta.x as f64)
    } else {
        Slope::Some(delta.y as f64 / delta.x as f64)
    }
}

fn get_y(hailstone: &(Point3D, Point3D), x: f64) -> TSolve {
    // We need to make sure that the x value is in the future for this.
    let dy = get_dy(&hailstone.1);
    if hailstone.1.x != 0 && (x - hailstone.0.x as f64) / (hailstone.1.x as f64) < 0.0 {
        TSolve::None
    } else {
        match dy {
            Slope::Vertical(x_slope) => {
                if x == x_slope {
                    TSolve::All
                } else {
                    TSolve::None
                }
            }
            Slope::Some(dy) => TSolve::Some(hailstone.0.y as f64 + dy * (x - hailstone.0.x as f64)),
        }
    }
}

impl Day24 {
    fn find_intersections_day_a(
        &self,
        hailstone_1: &(Point3D, Point3D),
        hailstone_2: &(Point3D, Point3D),
    ) -> TSolve {
        // Turns out I had this incorrect. Its if the paths cross, not the hailstones themselves.
        // An easy-ish way to fix this is to normalise the X values in both cases to 0 and then
        // find the y = case.
        //
        // y(0) = y(x) - dy * x
        // y1 - dy1 * x1 + dy1 * t = y2 - dy2 * x2 + dy2 * t
        // (dy1 - dy2) t = y2 - y1 + dy1 * x1 - dy2 * x2
        // t = (y2 - y1 + dy1 * x1 - dy2 * x2) / (dy1 - dy2)
        //
        // sometimes this won't work, because dy1 and dy2 are parallel.
        //
        // Sometimes this will be tricky, because dy1 or dy2 will be purely vertical, which means
        // they  will intersect but only at x = starting position.
        let dy1 = get_dy(&hailstone_1.1);
        let dy2 = get_dy(&hailstone_2.1);
        let x1 = hailstone_1.0.x as f64;
        let x2 = hailstone_2.0.x as f64;
        let y1 = hailstone_1.0.y as f64;
        let y2 = hailstone_2.0.y as f64;
        let x_intersect = match (dy1, dy2) {
            (Slope::Vertical(x1), Slope::Vertical(x2)) => {
                if x1 == x2 {
                    TSolve::All
                } else {
                    TSolve::None
                }
            }
            (Slope::Vertical(x), Slope::Some(_)) => TSolve::Some(x),
            (Slope::Some(_), Slope::Vertical(x)) => TSolve::Some(x),
            (Slope::Some(dy1), Slope::Some(dy2)) => {
                if dy1 == dy2 {
                    if hailstone_1.0.x == hailstone_2.0.x {
                        TSolve::All
                    } else {
                        TSolve::None
                    }
                } else {
                    TSolve::Some((y2 - y1 + dy1 * x1 - dy2 * x2) / (dy1 - dy2))
                }
            }
        };
        println!(
            "Solution to hailstone intersections {:?} vs {:?} is {:?}",
            hailstone_1, hailstone_2, x_intersect
        );
        x_intersect
    }

    fn intersects_in_bounds(
        &self,
        lower: f64,
        upper: f64,
        hailstone_1: &(Point3D, Point3D),
        hailstone_2: &(Point3D, Point3D),
    ) -> bool {
        let x = self.find_intersections_day_a(hailstone_1, hailstone_2);
        match x {
            TSolve::None => false,
            TSolve::All => true, // not guaranteed, but tricky math so lets see if this works
            TSolve::Some(x) if x > lower && x <= upper => {
                let y1 = get_y(hailstone_1, x);
                let y2 = get_y(hailstone_2, x);
                match (y1, y2) {
                    (TSolve::None, _) => false,
                    (_, TSolve::None) => false,
                    (TSolve::All, _) => true,
                    (_, TSolve::All) => true,
                    (TSolve::Some(y1), TSolve::Some(y2)) => y1 > lower && y1 <= upper,
                }
            }
            _ => false,
        }
    }
    fn calculate_day_a_for_bounds(&self, lower: f64, upper: f64) -> usize {
        self.hailstones
            .iter()
            .combinations(2)
            .filter(|hailstones| {
                self.intersects_in_bounds(lower, upper, hailstones[0], hailstones[1])
            })
            .count()
    }

    fn calculate_day_a(&self) -> usize {
        self.calculate_day_a_for_bounds(200000000000000., 400000000000000.)
    }

    fn calculate_day_b(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day24 = Day24::new("data/test_data.txt").unwrap();
        let expected = 2;
        let actual = day24.calculate_day_a_for_bounds(7., 27.);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day24 = Day24::new("data/test_data.txt").unwrap();
        let expected = 0;
        let actual = day24.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day24 = Day24::new("data/input_data.txt").unwrap();
        let expected = 0;
        let actual = day24.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day24 = Day24::new("data/input_data.txt").unwrap();
        let expected = 0;
        let actual = day24.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
