extern crate peg;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const _ROTATIONS_TO_CHECK: [(usize, usize, usize); 24] = [
    (0, 0, 0),
    (0, 0, 1),
    (0, 0, 2),
    (0, 1, 0),
    (0, 1, 1),
    (0, 1, 2),
    (0, 2, 0),
    (0, 2, 1),
    (0, 2, 2),
    (0, 3, 0),
    (0, 3, 1),
    (0, 3, 2),
    (1, 0, 0),
    (1, 0, 1),
    (1, 0, 2),
    (1, 1, 0),
    (1, 2, 0),
    (1, 2, 1),
    (1, 2, 2),
    (1, 3, 2),
    (2, 0, 1),
    (2, 2, 1),
    (3, 0, 1),
    (3, 2, 1),
];

#[derive(Clone, Debug, PartialEq, Hash, Eq, PartialOrd, Ord)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Debug, Clone)]
struct Scanner {
    beacons: Vec<Point>,
    offset: Point,
    rot: usize,
}

pub struct Day19Setup {
    scanners: Vec<Scanner>,
}

peg::parser! { grammar day19_parser() for str {
    rule positive_number() -> isize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule negative_number() -> isize
        = "-" n:positive_number() { -n }
    rule number() -> isize
        = n:(negative_number() / positive_number()) { n }
    rule beacon() -> Point
        = x:number() "," y:number() "," z:number() { Point{x, y, z} }
    rule scanner() -> Scanner
        = "--- scanner " number() " ---\n" beacons:beacon() ++ "\n"  { Scanner { beacons, offset: Point{ x:0, y:0, z:0 } , rot: 0} }
    pub rule parse() -> Day19Setup
        = scanners:scanner() ++ "\n\n" "\n" * {
            Day19Setup { scanners }
        }
}}

impl Scanner {
    fn add_point(&mut self, p: &Point) {
        for beacon in self.beacons.iter_mut() {
            beacon.add_point(p)
        }
        self.offset.add_point(p);
    }

    fn multi_rotate(
        &self,
        around_x_count: usize,
        around_y_count: usize,
        around_z_count: usize,
    ) -> Scanner {
        let beacons: Vec<Point> = self
            .beacons
            .iter()
            .map(|beacon| beacon.multi_rotate(around_x_count, around_y_count, around_z_count))
            .collect();
        Scanner {
            beacons,
            offset: self
                .offset
                .multi_rotate(around_x_count, around_y_count, around_z_count),
            rot: 0,
        }
    }

    fn rotate_scanner(&mut self) {
        for beacon in self.beacons.iter_mut() {
            beacon.rotate(self.rot);
        }
        self.rot += 1;
    }

    fn find_difference_between_common_points(&self, other: &Scanner) -> Option<Point> {
        let mut distances: HashMap<Point, usize> = HashMap::with_capacity(self.beacons.len() * 2);
        for distance in self
            .beacons
            .iter()
            .cartesian_product(&other.beacons)
            .map(|(p1, p2)| p2.sub_point(p1))
        {
            let entry = distances.entry(distance.clone()).or_insert(0);
            *entry += 1;
            if *entry >= 12 {
                return Some(distance);
            }
        }
        None
    }

    fn rotate_scanner_and_apply_offset(&mut self, other: &Scanner) -> bool {
        for _ in 0..24 {
            self.rotate_scanner();
            if let Some(diff) = self.find_difference_between_common_points(other) {
                self.add_point(&diff);
                return true;
            }
        }
        false
    }

    fn transform_based_on_good_scanners(&mut self, others: &[Scanner]) -> bool {
        for other in others.iter() {
            if self.rotate_scanner_and_apply_offset(other) {
                return true;
            }
            self.rotate_scanner(); // Resets back to 0 rotation;
            self.rot = 0;
        }
        false
    }
}

impl Point {
    fn rotate(&mut self, rot_point: usize) {
        match rot_point {
            // targets are
            0 => (),
            //x, y, z
            1 => {
                self.y = -self.y;
                self.z = -self.z;
            }
            //x, -y, -z,
            2 => {
                self.x = -self.x;
                self.z = -self.z;
            }
            //-x, -y, z,
            3 => {
                self.y = -self.y;
                self.z = -self.z;
            }
            //-x, y, -z,
            4 => {
                let old_y = self.y;
                self.y = self.z;
                self.z = -old_y;
            }
            //-x, -z, -y,
            5 => {
                self.y = -self.y;
                self.z = -self.z;
            }
            //-x, z, y,
            6 => {
                self.x = -self.x;
                self.z = -self.z;
            }
            //x, z, -y,
            7 => {
                self.y = -self.y;
                self.z = -self.z;
            }
            //x, -z, y,
            8 => {
                let old_x = self.x;
                self.x = self.z;
                self.z = self.y;
                self.y = old_x;
            }
            //y, x, -z,
            9 => {
                self.y = -self.y;
                self.z = -self.z;
            }
            //y, -x, z,
            10 => {
                self.x = -self.x;
                self.z = -self.z;
            }
            //-y, -x, -z,
            11 => {
                self.y = -self.y;
                self.z = -self.z;
            }
            //-y, x, z,
            12 => {
                self.x = -self.x;
                std::mem::swap(&mut self.y, &mut self.z);
            }
            //y, z, x,
            13 => {
                self.y = -self.y;
                self.z = -self.z;
            }
            //y, -z, -x,
            14 => {
                self.x = -self.x;
                self.z = -self.z;
            }
            //-y, -z, x,
            15 => {
                self.y = -self.y;
                self.z = -self.z;
            }
            //-y, z, -x,
            16 => {
                let old_x = self.x;
                self.x = self.y;
                self.y = -self.z;
                self.z = -old_x;
            }
            //z, x, y,
            17 => {
                self.y = -self.y;
                self.z = -self.z;
            }
            //z, -x, -y,
            18 => {
                self.x = -self.x;
                self.z = -self.z;
            }
            //-z, -x, y,
            19 => {
                self.y = -self.y;
                self.z = -self.z;
            }
            //-z, x, -y,
            20 => {
                self.x = -self.x;
                let old_y = self.y;
                self.y = -self.z;
                self.z = -old_y;
            }
            //z, y, -x,
            21 => {
                self.y = -self.y;
                self.z = -self.z;
            }
            //z, -y, x,
            22 => {
                self.x = -self.x;
                self.z = -self.z;
            }
            //-z, -y, -x,
            23 => {
                self.y = -self.y;
                self.z = -self.z;
            }
            //-z, y, x,
            24 => {
                let old_x = self.x;
                self.x = self.z;
                self.z = -old_x;
            }
            // x, y, z
            _ => (),
        }
    }
    fn add_point(&mut self, other: &Point) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }

    fn sub_point(&self, other: &Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn magnitude(&self) -> isize {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    fn rotate_around_z(&self) -> Point {
        Point {
            x: self.y,
            y: -self.x,
            z: self.z,
        }
    }

    fn rotate_around_y(&self) -> Point {
        Point {
            x: self.z,
            y: self.y,
            z: -self.x,
        }
    }

    fn rotate_around_x(&self) -> Point {
        Point {
            x: self.x,
            y: self.z,
            z: -self.y,
        }
    }

    fn multi_rotate(
        &self,
        around_x_count: usize,
        around_y_count: usize,
        around_z_count: usize,
    ) -> Point {
        let mut curr = self.clone();
        for _ in 0..around_x_count {
            curr = curr.rotate_around_x();
        }
        for _ in 0..around_y_count {
            curr = curr.rotate_around_y();
        }
        for _ in 0..around_z_count {
            curr = curr.rotate_around_z();
        }
        curr
    }
}

impl Day19Setup {
    /// Generates a new Day19Setup object to be calculated upon
    ///
    /// Inputs: the full string contents of the input data set.
    ///
    /// Returns: A new Day19Setup object, with methods `calculate_day_a` and `calculate_day_b`
    /// available
    pub fn new(input_str: &str) -> Day19Setup {
        day19_parser::parse(input_str).unwrap()
    }

    /// Maps all the scanners ontop of each other, relative to the
    /// first scanner in the list.
    ///
    /// Returns: A new Day19Setup with all the scanners in the correct orientation and position
    pub fn perform_scanner_mapping(&mut self) {
        let mut transformed_scanners = vec![];
        let mut next_transformed_scanners = vec![];
        let mut found: Vec<bool> = vec![false; self.scanners.len()];
        found[0] = true;
        transformed_scanners.push(self.scanners[0].clone());
        while !found.iter().all(|x| *x) {
            for (i, f) in found.iter_mut().enumerate().take(self.scanners.len()) {
                if !*f && self.scanners[i].transform_based_on_good_scanners(&transformed_scanners) {
                    next_transformed_scanners.push(self.scanners[i].clone());
                    *f = true;
                }
            }
            transformed_scanners = next_transformed_scanners;
            next_transformed_scanners = vec![];
        }
    }

    /// Calculate the part a response
    ///
    /// Requires the mapping to already have been done.
    ///
    /// How many unique beacons are there among all scanners.
    pub fn calculate_day_a(self: &Day19Setup) -> usize {
        let mut all_points: HashSet<&Point> = HashSet::new();
        for scanner in self.scanners.iter() {
            for beacon in scanner.beacons.iter() {
                all_points.insert(beacon);
            }
        }
        all_points.len()
    }

    /// Calculate the part b response
    ///
    /// Requires the mapping to already have been done.
    ///
    /// What is the maximum manhattan distance between any 2 scanners
    pub fn calculate_day_b(self: &Day19Setup) -> isize {
        self.scanners
            .iter()
            .tuple_combinations()
            .map(|(scan_1, scan_2)| scan_1.offset.sub_point(&scan_2.offset))
            .map(|offset| offset.magnitude())
            .reduce(std::cmp::max)
            .expect("Expected reduce to work as there are more than 2 scans comparing at all times")
    }
}

#[cfg(test)]
mod test {
    use crate::Day19Setup;
    use crate::HashMap;
    use crate::Point;
    use crate::Scanner;
    use crate::_ROTATIONS_TO_CHECK;

    #[test]
    fn test_parse() {
        let day19_setup = Day19Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day19_setup.scanners.len(), 5);
    }

    #[test]
    fn test_generating_point_rotations() {
        let mut all_points = vec![];
        let mut rotation_mapping: HashMap<Point, (usize, usize, usize)> = HashMap::new();
        for x in 0..4 {
            for y in 0..4 {
                for z in 0..3 {
                    let found_val = Point { x: 1, y: 2, z: 3 }.multi_rotate(x, y, z);
                    if !all_points.contains(&found_val) {
                        all_points.push(found_val.clone());
                        rotation_mapping.insert(found_val.clone(), (x, y, z));
                    }
                }
            }
        }
        let mut rotation_values: Vec<(usize, usize, usize)> =
            rotation_mapping.values().copied().collect();
        rotation_values.sort_unstable();
        // likely to pass since ROTATIONS_TO_CHECK was built from these values, but still good to
        // test that we haven't broken it
        assert_eq!(&_ROTATIONS_TO_CHECK[0..24], &rotation_values[0..24]);
    }

    #[test]
    fn test_scanner_rotations() {
        let scanner = Scanner {
            beacons: vec![Point { x: 1, y: 2, z: 3 }, Point { x: -3, y: 5, z: -8 }],
            offset: Point { x: 0, y: 0, z: 0 },
            rot: 0,
        };
        let scanner_rot = scanner.multi_rotate(1, 0, 0);
        assert_eq!(scanner_rot.beacons[0], Point { x: 1, y: 3, z: -2 });
        assert_eq!(
            scanner_rot.beacons[1],
            Point {
                x: -3,
                y: -8,
                z: -5
            }
        );
    }

    #[test]
    fn test_point_rots() {
        // The rotations from the solution I borrowed from reddit, just to confirm I was doing this
        // correctly (hint: I was)
        let mut expected_rots = [
            Point {
                x: -3,
                y: -2,
                z: -1,
            },
            Point { x: -3, y: -1, z: 2 },
            Point { x: -3, y: 1, z: -2 },
            Point { x: -3, y: 2, z: 1 },
            Point { x: -2, y: -3, z: 1 },
            Point {
                x: -2,
                y: -1,
                z: -3,
            },
            Point { x: -2, y: 1, z: 3 },
            Point { x: -2, y: 3, z: -1 },
            Point {
                x: -1,
                y: -3,
                z: -2,
            },
            Point { x: -1, y: -2, z: 3 },
            Point { x: -1, y: 2, z: -3 },
            Point { x: -1, y: 3, z: 2 },
            Point { x: 1, y: -3, z: 2 },
            Point { x: 1, y: -2, z: -3 },
            Point { x: 1, y: 2, z: 3 },
            Point { x: 1, y: 3, z: -2 },
            Point { x: 2, y: -3, z: -1 },
            Point { x: 2, y: -1, z: 3 },
            Point { x: 2, y: 1, z: -3 },
            Point { x: 2, y: 3, z: 1 },
            Point { x: 3, y: -2, z: 1 },
            Point { x: 3, y: -1, z: -2 },
            Point { x: 3, y: 1, z: 2 },
            Point { x: 3, y: 2, z: -1 },
        ];

        expected_rots.sort_unstable();

        let mut p = Point { x: 1, y: 2, z: 3 };
        let mut calculated_rots = (0..24)
            .map(|rotate_index| {
                p.rotate(rotate_index);
                p.clone()
            })
            .collect::<Vec<Point>>();
        calculated_rots.sort_unstable();
        assert_eq!(expected_rots.to_vec(), calculated_rots);
    }
    #[test]
    fn test_scanner_diffs() {
        let scanner_1 = Scanner {
            beacons: vec![
                Point { x: 1, y: 2, z: 3 },
                Point { x: 2, y: 2, z: 3 },
                Point { x: 3, y: 2, z: 3 },
                Point { x: 4, y: 2, z: 3 },
                Point { x: 5, y: 2, z: 3 },
                Point { x: 6, y: 2, z: 3 },
                Point { x: 7, y: 2, z: 3 },
                Point { x: 8, y: 2, z: 3 },
                Point { x: 9, y: 2, z: 3 },
                Point { x: 10, y: 2, z: 3 },
                Point { x: 11, y: 2, z: 3 },
                Point { x: 12, y: 2, z: 3 },
                Point { x: 13, y: 6, z: 6 },
            ],
            offset: Point { x: 0, y: 0, z: 0 },
            rot: 0,
        };
        let mut scanner_2 = Scanner {
            beacons: vec![
                Point { x: 1, y: 2, z: 4 },
                Point { x: 2, y: 2, z: 4 },
                Point { x: 3, y: 2, z: 4 },
                Point { x: 4, y: 2, z: 4 },
                Point { x: 5, y: 2, z: 4 },
                Point { x: 6, y: 2, z: 4 },
                Point { x: 7, y: 2, z: 4 },
                Point { x: 8, y: 2, z: 4 },
                Point { x: 9, y: 2, z: 4 },
                Point { x: 10, y: 2, z: 4 },
                Point { x: 11, y: 2, z: 4 },
                Point { x: 12, y: 2, z: 4 },
                Point { x: 7, y: -3, z: 0 },
            ],
            offset: Point { x: 0, y: 0, z: 0 },
            rot: 0,
        };
        assert!(scanner_2.rotate_scanner_and_apply_offset(&scanner_1));
        assert_eq!(scanner_2.beacons[0], Point { x: 1, y: 2, z: 3 });
        assert_eq!(scanner_2.offset, Point { x: 0, y: 0, z: -1 });
    }

    #[test]
    fn test_day_a() {
        let mut day19_setup = Day19Setup::new(include_str!("../test_data.txt"));
        day19_setup.perform_scanner_mapping();
        assert_eq!(day19_setup.calculate_day_a(), 79);
    }

    #[test]
    fn test_day_b() {
        let mut day19_setup = Day19Setup::new(include_str!("../test_data.txt"));
        day19_setup.perform_scanner_mapping();
        assert_eq!(day19_setup.calculate_day_b(), 3621);
    }
}
