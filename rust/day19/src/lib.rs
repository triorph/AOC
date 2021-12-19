extern crate peg;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const ROTATIONS_TO_CHECK: [(usize, usize, usize); 24] = [
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
        = "--- scanner " number() " ---\n" beacons:beacon() ++ "\n"  { Scanner { beacons, offset: Point{ x:0, y:0, z:0 } } }
    pub rule parse() -> Day19Setup
        = scanners:scanner() ++ "\n\n" "\n" * {
            Day19Setup { scanners }
        }
}}

impl Scanner {
    fn add_point(self, p: &Point) -> Scanner {
        Scanner {
            beacons: self
                .beacons
                .iter()
                .cloned()
                .map(|x| x.add_point(p))
                .collect(),
            offset: self.offset.add_point(p),
        }
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
        }
    }

    fn get_all_scanner_rotations(&self) -> [Scanner; 24] {
        ROTATIONS_TO_CHECK
            .iter()
            .map(|(around_x_count, around_y_count, around_z_count)| {
                self.multi_rotate(*around_x_count, *around_y_count, *around_z_count)
            })
            .collect::<Vec<Scanner>>()
            .try_into()
            .expect("Should be 24 entries in both ROTATIONS_TO_CHECK and get_all_scanner_rotations.result")
    }

    fn find_difference_between_common_points(&self, other: &Scanner) -> Option<Point> {
        let mut distances: HashMap<Point, usize> = HashMap::new();
        for distance in self
            .beacons
            .iter()
            .cartesian_product(&other.beacons)
            .map(|(p1, p2)| p1.sub_point(p2))
        {
            *distances.entry(distance).or_insert(0) += 1;
        }
        for (distance, &count) in distances.iter() {
            if count >= 12 {
                return Some(distance.clone());
            }
        }
        None
    }

    fn rotate_scanner_and_apply_offset(&self, other: &Scanner) -> Option<Scanner> {
        for (i, scanner) in other.get_all_scanner_rotations().into_iter().enumerate() {
            if let Some(diff) = self.find_difference_between_common_points(&scanner) {
                let p = scanner.beacons[0].clone();
                let mut ret = scanner.add_point(&diff);
                /* println!(
                    "Found distance {:?}, {:?}. built off {:?}, creating point {:?}",
                    diff, p, self.beacons[0], ret.beacons[0]
                ); */
                // ret.offset = ret.offset.add_point(&self.offset);
                return Some(ret);
            }
        }
        None
    }
}

impl Point {
    fn add_point(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
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

    pub fn perform_scanner_mapping(self: &Day19Setup) -> Day19Setup {
        let mut transformed_scanners = vec![];
        let mut found: Vec<bool> = vec![false; self.scanners.len()];
        found[0] = true;
        transformed_scanners.push(self.scanners[0].clone());
        while transformed_scanners.len() < self.scanners.len() {
            for i in 0..self.scanners.len() {
                if found[i] {
                    continue;
                }
                for j in 0..transformed_scanners.len() {
                    if let Some(transformed_scanner) = (&transformed_scanners[j])
                        .rotate_scanner_and_apply_offset(&self.scanners[i])
                    {
                        transformed_scanners.push(transformed_scanner);
                        found[i] = true;
                        break;
                    }
                }
            }
        }
        Day19Setup {
            scanners: transformed_scanners,
        }
    }

    /// Calculate the part a response
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
    pub fn calculate_day_b(self: &Day19Setup) -> isize {
        let mut points = self
            .scanners
            .iter()
            .map(|scanner| scanner.offset.clone())
            .map(|offset| [offset.x, offset.y, offset.z])
            .collect::<Vec<[isize; 3]>>();
        points.sort_unstable();
        println!("{:?}", &points);
        let diff_offset_magnitudes = self
            .scanners
            .iter()
            .tuple_combinations()
            .map(|(scan_1, scan_2)| scan_1.offset.sub_point(&scan_2.offset))
            .map(|offset| offset.magnitude())
            .collect::<Vec<isize>>();

        // println!("{:?}", &diff_offset_magnitudes);
        diff_offset_magnitudes
            .into_iter()
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
    use crate::ROTATIONS_TO_CHECK;

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
                    if all_points.contains(&found_val) {
                        println!("Duplicate found on {} {} {}", x, y, z);
                        println!("Duplicate of {:?}", rotation_mapping.get(&found_val));
                    } else {
                        all_points.push(found_val.clone());
                        rotation_mapping.insert(found_val.clone(), (x, y, z));
                    }

                    println!("{}, {}, {} rotates to {:?}", x, y, z, found_val);
                }
            }
        }
        println!("Found {} points total", all_points.len());
        println!("Found points are: {:?}", rotation_mapping.values())
    }

    #[test]
    fn test_scanner_rotations() {
        let scanner = Scanner {
            beacons: vec![Point { x: 1, y: 2, z: 3 }, Point { x: -3, y: 5, z: -8 }],
            offset: Point { x: 0, y: 0, z: 0 },
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
        let expected_rots = [
            [[-3, -2, -1]],
            [[-3, -1, 2]],
            [[-3, 1, -2]],
            [[-3, 2, 1]],
            [[-2, -3, 1]],
            [[-2, -1, -3]],
            [[-2, 1, 3]],
            [[-2, 3, -1]],
            [[-1, -3, -2]],
            [[-1, -2, 3]],
            [[-1, 2, -3]],
            [[-1, 3, 2]],
            [[1, -3, 2]],
            [[1, -2, -3]],
            [[1, 2, 3]],
            [[1, 3, -2]],
            [[2, -3, -1]],
            [[2, -1, 3]],
            [[2, 1, -3]],
            [[2, 3, 1]],
            [[3, -2, 1]],
            [[3, -1, -2]],
            [[3, 1, 2]],
            [[3, 2, -1]],
        ];

        let mut calculated_rots = ROTATIONS_TO_CHECK
            .iter()
            .map(|(x, y, z)| Point { x: 1, y: 2, z: 3 }.multi_rotate(*x, *y, *z))
            .collect::<Vec<Point>>();
        calculated_rots.sort_unstable();
        for (calculated, expected) in calculated_rots.iter().zip(expected_rots) {
            assert_eq!(calculated.x, expected[0][0]);
            assert_eq!(calculated.y, expected[0][1]);
            assert_eq!(calculated.z, expected[0][2]);
        }
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
        };
        let scanner_2 = Scanner {
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
        };
        let resulting_scanner = scanner_1
            .rotate_scanner_and_apply_offset(&scanner_2)
            .expect("Test should map correctly");
        assert_eq!(resulting_scanner.beacons[0], Point { x: 1, y: 2, z: 3 });
        assert_eq!(resulting_scanner.offset, Point { x: 0, y: 0, z: -1 });
    }

    #[test]
    fn test_day_a() {
        let day19_setup = Day19Setup::new(include_str!("../test_data.txt"));
        let transformed_setup = day19_setup.perform_scanner_mapping();
        assert_eq!(transformed_setup.calculate_day_a(), 79);
    }

    #[test]
    fn test_day_b() {
        let day19_setup = Day19Setup::new(include_str!("../test_data.txt"));
        let transformed_setup = day19_setup.perform_scanner_mapping();
        assert_eq!(transformed_setup.calculate_day_b(), 3621);
    }
}
