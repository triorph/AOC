extern crate peg;

#[derive(Clone)]
pub struct SmokeLocations {
    smoke_locations: Vec<Vec<usize>>,
}

pub struct Basin {}

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

impl SmokeLocations {
    fn new(smoke_location_input_str: &str) -> SmokeLocations {
        day9_parser::parse(smoke_location_input_str).unwrap()
    }

    fn get_at_point(self: &SmokeLocations, x: isize, y: isize) -> Option<usize> {
        if (0..(self.smoke_locations.len() as isize)).contains(&y)
            && (0..(self.smoke_locations[y as usize].len() as isize)).contains(&x)
        {
            Some(self.smoke_locations[y as usize][x as usize])
        } else {
            None
        }
    }

    fn is_local_minima(self: &SmokeLocations, x: isize, y: isize) -> bool {
        for (x_to_check, y_to_check) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
            if let Some(val_to_check) = self.get_at_point(x_to_check, y_to_check) {
                if val_to_check <= self.smoke_locations[y as usize][x as usize] {
                    return false;
                }
            }
        }
        true
    }

    fn calculate_day_a(self: &SmokeLocations) -> usize {
        (0..self.smoke_locations.len())
            .map(|y| {
                (0..self.smoke_locations[y].len())
                    .map(|x| {
                        if self.is_local_minima(x as isize, y as isize) {
                            self.smoke_locations[y][x] + 1
                        } else {
                            0
                        }
                    })
                    .sum::<usize>()
            })
            .sum()
    }

    fn get_basin_for_point(self: &SmokeLocations, x: isize, y: isize) -> Basin {}

    fn calculate_day_b(self: &SmokeLocations) -> usize {
        0
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
    fn test_day_b() {
        let smoke_locations = SmokeLocations::new(include_str!("../test_data.txt"));
        assert_eq!(smoke_locations.calculate_day_b(), 1134);
    }
}
