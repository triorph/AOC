extern crate itertools;
extern crate peg;
use itertools::Itertools;

#[derive(Clone)]
pub struct SmokeLocations {
    smoke_locations: Vec<Vec<usize>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    x: isize,
    y: isize,
}

pub struct Basin {
    basin_locations: Vec<Point>,
}

peg::parser! { grammar day9_parser() for str {
    rule number_char() -> usize
        = n:$(['0'..='9']) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule smoke_line() -> Vec<usize>
        = smoke_line:number_char() ++ "" { smoke_line }
    pub rule parse() -> SmokeLocations
        = smoke_locations:smoke_line() ** ("\n" +) "\n" * {
            SmokeLocations { smoke_locations }
        }
}
}

impl Point {
    fn get_neighbours(self: &Point) -> Vec<Point> {
        vec![
            Point {
                x: self.x - 1,
                y: self.y,
            },
            Point {
                x: self.x + 1,
                y: self.y,
            },
            Point {
                x: self.x,
                y: self.y - 1,
            },
            Point {
                x: self.x,
                y: self.y + 1,
            },
        ]
    }
}

impl<'a> SmokeLocations {
    fn new(smoke_location_input_str: &str) -> SmokeLocations {
        day9_parser::parse(smoke_location_input_str).unwrap()
    }

    fn get_at_point(self: &SmokeLocations, point: &Point) -> Option<usize> {
        if (0..(self.smoke_locations.len() as isize)).contains(&point.y)
            && (0..(self.smoke_locations[point.y as usize].len() as isize)).contains(&point.x)
        {
            Some(self.smoke_locations[point.y as usize][point.x as usize])
        } else {
            None
        }
    }

    fn is_local_minima(self: &SmokeLocations, starting_point: &Point) -> bool {
        for point_to_check in starting_point.get_neighbours().into_iter() {
            if let Some(val_to_check) = self.get_at_point(&point_to_check) {
                if val_to_check <= self.get_at_point(starting_point).unwrap() {
                    return false;
                }
            }
        }
        true
    }
    fn get_points_iter(self: &SmokeLocations) -> Box<dyn Iterator<Item = Point>> {
        Box::new(
            (0..self.smoke_locations.len())
                .cartesian_product(0..self.smoke_locations[0].len())
                .map(|(y, x)| Point {
                    x: x as isize,
                    y: y as isize,
                }),
        )
    }

    fn calculate_day_a(self: &SmokeLocations) -> usize {
        self.get_points_iter()
            .map(|point| {
                if self.is_local_minima(&point) {
                    if let Some(val) = self.get_at_point(&point) {
                        val + 1
                    } else {
                        0
                    }
                } else {
                    0
                }
            })
            .sum()
    }

    fn get_basin_for_point(self: &SmokeLocations, starting_point: &Point) -> Basin {
        let mut to_check: Vec<Point> = vec![starting_point.clone()];
        let mut ret: Vec<Point> = vec![];
        while !to_check.is_empty() {
            if let Some(point_to_check) = to_check.pop() {
                let val = self.get_at_point(&point_to_check);
                if val != Some(9) && val != None {
                    for neighbour in point_to_check.get_neighbours().into_iter() {
                        if !ret.contains(&neighbour) && !to_check.contains(&neighbour) {
                            to_check.push(neighbour);
                        }
                    }
                    ret.push(point_to_check);
                }
            }
        }

        Basin {
            basin_locations: ret,
        }
    }

    fn calculate_day_b(self: &SmokeLocations) -> usize {
        let mut to_check: Vec<Point> = self.get_points_iter().collect();
        let mut basins: Vec<Basin> = vec![];
        while let Some(point_to_check) = to_check.pop() {
            let basin = self.get_basin_for_point(&point_to_check);
            let locations = &basin.basin_locations;
            to_check = to_check
                .into_iter()
                .filter(|point| !locations.contains(point))
                .collect();
            basins.push(basin);
        }
        let mut basin_sizes = basins
            .into_iter()
            .map(|basin| basin.basin_locations.len())
            .collect::<Vec<usize>>();
        basin_sizes.sort_unstable();
        return basin_sizes[basin_sizes.len() - 3..].iter().product();
    }
}

fn main() {
    let smoke_locations = SmokeLocations::new(include_str!("../input_data.txt"));
    let day_a = smoke_locations.calculate_day_a();
    println!("Day a result: {}", day_a);
    let smoke_locations = SmokeLocations::new(include_str!("../input_data.txt"));
    let day_b = smoke_locations.calculate_day_b();
    println!("Day b result: {}", day_b);
}

#[cfg(test)]
mod test {
    use crate::Point;
    use crate::SmokeLocations;

    #[test]
    fn test_parse() {
        let smoke_locations = SmokeLocations::new(include_str!("../test_data.txt"));
        assert_eq!(smoke_locations.smoke_locations.len(), 5);
        assert_eq!(smoke_locations.smoke_locations[0].len(), 10);
    }

    #[test]
    fn test_day_a() {
        let smoke_locations = SmokeLocations::new(include_str!("../test_data.txt"));
        assert_eq!(smoke_locations.calculate_day_a(), 15);
    }

    #[test]
    fn test_get_basin_for_point() {
        let smoke_locations = SmokeLocations::new(include_str!("../test_data.txt"));
        let top_left = smoke_locations.get_basin_for_point(&Point { x: 0, y: 0 });
        assert_eq!(top_left.basin_locations.len(), 3);
        let top_right = smoke_locations.get_basin_for_point(&Point { x: 9, y: 0 });
        assert_eq!(top_right.basin_locations.len(), 9);
        let middle = smoke_locations.get_basin_for_point(&Point { x: 2, y: 1 });
        assert_eq!(middle.basin_locations.len(), 14);
        let bottom_left = smoke_locations.get_basin_for_point(&Point { x: 0, y: 4 });
        assert_eq!(bottom_left.basin_locations.len(), 0);
        let bottom_right = smoke_locations.get_basin_for_point(&Point { x: 9, y: 4 });
        assert_eq!(bottom_right.basin_locations.len(), 9);
    }

    #[test]
    fn test_day_b() {
        let smoke_locations = SmokeLocations::new(include_str!("../test_data.txt"));
        assert_eq!(smoke_locations.calculate_day_b(), 1134);
    }
}
