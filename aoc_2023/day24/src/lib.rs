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
pub enum Solution {
    All,
    None,
    Some(f64),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Slope {
    Some(f64),
    Vertical(f64),
}

fn get_dy(delta: &Point3D) -> Slope {
    if delta.y == 0 {
        Slope::Vertical(delta.x as f64)
    } else {
        Slope::Some(delta.y as f64 / delta.x as f64)
    }
}

fn get_y(hailstone: &(Point3D, Point3D), x: f64) -> Solution {
    let dy = get_dy(&hailstone.1);
    if hailstone.1.x != 0 && (x - hailstone.0.x as f64) / (hailstone.1.x as f64) < 0.0 {
        // We need to make sure that the x value is in the future for this.
        Solution::None
    } else {
        match dy {
            Slope::Vertical(x_slope) => {
                if x == x_slope {
                    Solution::All
                } else {
                    Solution::None
                }
            }
            Slope::Some(dy) => {
                Solution::Some(hailstone.0.y as f64 + dy * (x - hailstone.0.x as f64))
            }
        }
    }
}

impl Day24 {
    fn find_intersections_day_a(
        &self,
        hailstone_1: &(Point3D, Point3D),
        hailstone_2: &(Point3D, Point3D),
    ) -> Solution {
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
        match (dy1, dy2) {
            (Slope::Vertical(x1), Slope::Vertical(x2)) => {
                if x1 == x2 {
                    Solution::All
                } else {
                    Solution::None
                }
            }
            (Slope::Vertical(x), Slope::Some(_)) => Solution::Some(x),
            (Slope::Some(_), Slope::Vertical(x)) => Solution::Some(x),
            (Slope::Some(dy1), Slope::Some(dy2)) => {
                if dy1 == dy2 {
                    if hailstone_1.0.x == hailstone_2.0.x {
                        Solution::All
                    } else {
                        Solution::None
                    }
                } else {
                    Solution::Some((y2 - y1 + dy1 * x1 - dy2 * x2) / (dy1 - dy2))
                }
            }
        }
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
            Solution::None => false,
            Solution::All => true, // not guaranteed, but tricky math so lets see if this works
            Solution::Some(x) if x > lower && x <= upper => {
                let y1 = get_y(hailstone_1, x);
                let y2 = get_y(hailstone_2, x);
                match (y1, y2) {
                    (Solution::None, _) => false,
                    (_, Solution::None) => false,
                    (Solution::All, _) => true,
                    (_, Solution::All) => true,
                    (Solution::Some(y1), Solution::Some(y2)) => {
                        assert!((y2 - y1).abs() < 0.01 * y1.abs(), "{:?} {:?}", y2, y1);
                        y1 > lower && y1 <= upper
                    }
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

    fn build_matrix_for_hailstones(
        &self,
        hailstone_1: &(Point3D, Point3D),
        hailstone_2: &(Point3D, Point3D),
    ) -> Vec<Vec<f64>> {
        // https://www.reddit.com/r/adventofcode/comments/18pnycy/2023_day_24_solutions/kepu26z/
        //
        // I was mostly on the right track, the key was multiplying the cross-product to remove
        // the `t` term.
        //
        // p0 + t * v0 = pi + t * vi
        // (p0 - pi) = (vi - v0) * t
        //
        // cross product both sides by (vi - v0)
        // (p0 - pi) x (v0 - vi) = (vi - v0) x (v0 - v1) * t, but (vi - v0) x (v0 - v1) == 0 due to
        // being parallel (0 being the 3d vector of all 0s)
        //
        // (p0 - pi) x (v0 - vi) = 0
        //
        // which comes out as p0 x v0 - pi x v0 - p0 x vi + pi x vi = 0 (I have confirmed cross
        // products are distributive. They are not quite commutative but should work -1 * magnitude
        // when reversed, which I think is fine for an `== 0` term.)
        //
        // Unfortunately there's still the p0 * v0 term, but this is common across all hailstones,
        // so if we grab another hailstone pj/vj and right the same equation
        //
        // p0 x v0 - pj x v0 - p0 x vj + pj x vj = 0
        //
        // and set them equal to each other:
        // p0 x v0 - pj x v0 - p0 x vj + pj x vj = p0 x v0 - pi x v0 - p0 x vi + pi x vi
        //
        // then the p0 x v0 terms will cancel out, and we can create 1 equation:
        // pi x v0 + p0 x vi - pi x vi - pj x v0 - p0 x vj + pj x vj = 0
        // (pi - pj) x v0 + (vi - vj) x p0 + (pj x vj - pi x vi) x 1 (scalar) = 0
        //
        // Of note: `x` refers to the cross product, that is, if c = a x b, then c.x = a.y * b.z -
        // b.y * a.z
        //
        // So translating this to x/y/z formulas dependant on x/y/z we get:
        //
        // (pi - pj).y * v0.z - (pi - pj).z * v0.y + (vi - vj).y * p0.z - (vi - vj).z * p0.y +
        //  pj.y * vj.z - pj.z * vj.y - (pi.y * vi.z - pi.z * vi.y) == 0 (0.x)
        //
        // (pi - pj).z * v0.x - (pi - pj).x * v0.z + (vi - vj).z * p0.x - (vi - vj).x * p0.z +
        //  pj.z * vj.x - pj.x * vj.z - (pi.z * vi.x - pi.x * vi.z) == 0 (0.y)
        //
        // (pi - pj).x * v0.y - (pi - pj).y * v0.x + (vi - vj).x * p0.y - (vi - vj).y * p0.x +
        //  pj.x * vj.y - pj.y * vj.x - (pi.x * vi.y - pi.y * vi.x) == 0 (0.z)
        //
        //
        // These 2 terms will give us 3 vectors, so if we add a 3rd hailstone we can get the 6
        // vectors required to do a full gaussian elimination
        let pi = hailstone_1.0;
        let pj = hailstone_2.0;
        let vi = hailstone_1.1;
        let vj = hailstone_2.1;
        vec![
            // (pi - pj).y * v0.z - (pi - pj).z * v0.y + (vi - vj).y * p0.z - (vi - vj).z * p0.y +
            //  pj.y * vj.z - pj.z * vj.y - (pi.y * vi.z - pi.z * vi.y) == 0 (0.x)
            vec![
                0.0,
                -(&vi - &vj).z as f64,
                (&vi - &vj).y as f64,
                0.0,
                -(&pi - &pj).z as f64,
                (&pi - &pj).y as f64,
                pj.y as f64 * vj.z as f64
                    - pj.z as f64 * vj.y as f64
                    - (pi.y as f64 * vi.z as f64 - pi.z as f64 * vi.y as f64),
            ],
            // (pi - pj).z * v0.x - (pi - pj).x * v0.z + (vi - vj).z * p0.x - (vi - vj).x * p0.z +
            //  pj.z * vj.x - pj.x * vj.z - (pi.z * vi.x - pi.x * vi.z) == 0 (0.y)
            vec![
                (&vi - &vj).z as f64,
                0.0,
                -(&vi - &vj).x as f64,
                (&pi - &pj).z as f64,
                0.0,
                -(&pi - &pj).x as f64,
                pj.z as f64 * vj.x as f64
                    - pj.x as f64 * vj.z as f64
                    - (pi.z as f64 * vi.x as f64 - pi.x as f64 * vi.z as f64),
            ],
            // (pi - pj).x * v0.y - (pi - pj).y * v0.x + (vi - vj).x * p0.y - (vi - vj).y * p0.x +
            //  pj.x * vj.y - pj.y * vj.x - (pi.x * vi.y - pi.y * vi.x) == 0 (0.z)
            vec![
                -(&vi - &vj).y as f64,
                (&vi - &vj).x as f64,
                0.0,
                -(&pi - &pj).y as f64,
                (&pi - &pj).x as f64,
                0.0,
                pj.x as f64 * vj.y as f64
                    - pj.y as f64 * vj.x as f64
                    - (pi.x as f64 * vi.y as f64 - pi.y as f64 * vi.x as f64),
            ],
        ]
    }

    fn format_matrix(&self, matrix: &[Vec<f64>]) -> String {
        let mut ret = String::new();
        ret += "\n";
        for line in matrix.iter() {
            for point in line.iter() {
                ret += &point.to_string();
                ret += "          ";
            }
            ret += "\n";
        }
        ret
    }

    fn gaussian_elimination(&self, matrix: Vec<Vec<f64>>) -> Vec<f64> {
        let mut matrix = matrix;
        let mut normaliser_index = [None; 6];
        for i in 0..6 {
            #[allow(clippy::needless_range_loop)]
            for j in 0..6 {
                if !normaliser_index[0..i].contains(&Some(j)) && matrix[j][i].abs() > 0.1 {
                    normaliser_index[i] = Some(j);
                    break;
                }
            }
            if normaliser_index[i].is_none() {
                panic!(
                    "Could not find a normaliser index, {}, {:?}, {}",
                    i,
                    normaliser_index,
                    self.format_matrix(&matrix)
                );
            }
            let index = normaliser_index[i].unwrap();
            let normaliser = matrix[index][i];
            for j in i..7 {
                matrix[index][j] /= normaliser;
            }
            for j in 0..6 {
                if j == index {
                    continue;
                }
                for k in (i..7).rev() {
                    matrix[j][k] -= matrix[j][i] * matrix[index][k];
                }
            }
        }
        (0..6)
            .map(|i| matrix[normaliser_index[i].unwrap()][6])
            .collect()
    }

    fn calculate_day_b(&self) -> usize {
        // I think we can mostly define this as a series of simultaneous equations and solve
        // We need to find x/dx, y/dy, z/dz for the input rock, which we'll say are a/b, c/d, e/f
        // x + dx t = a + b t
        // y + dy t = c + d t
        // z + dz t = e + f t
        //
        // for a given hailstone, we need to ensure that t can be valid for all, and then reduce
        // down.
        //
        // There are essentially 6 unknowns, so I think 6 hailstones should be sufficient to get
        // an answer, although maybe t counts as an unknown in which case you'll need some
        // way of dealing with that.
        let mut matrix = self.build_matrix_for_hailstones(&self.hailstones[0], &self.hailstones[1]);
        matrix.extend(self.build_matrix_for_hailstones(&self.hailstones[0], &self.hailstones[3]));
        let rock = self.gaussian_elimination(matrix);
        rock[0..3].iter().map(|p| p.round().abs() as usize).sum()
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
        let expected = 47;
        let actual = day24.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day24 = Day24::new("data/input_data.txt").unwrap();
        let expected = 17776;
        let actual = day24.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day24 = Day24::new("data/input_data.txt").unwrap();
        let expected = 948978092202212;
        let actual = day24.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
