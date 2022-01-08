extern crate peg;
use itertools::Itertools;

pub struct Day17Setup {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
}

peg::parser! { grammar day17_parser() for str {
    rule positive_number() -> isize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule negative_number() -> isize
        = "-" n:positive_number() { -n }
    rule number() -> isize
        = n: (negative_number() / positive_number()) { n }
    pub rule parse() -> Day17Setup
        = "target area: x="  min_x:number() ".."  max_x:number() ", y=" min_y:number() ".." max_y:number()  "\n" * {
            Day17Setup {min_x, max_x, min_y, max_y}
        }
}}

impl Day17Setup {
    /// Generates a new Day17Setup object to be calculated upon
    ///
    /// Inputs: the full string contents of the input data set.
    ///
    /// Returns: A new Day17Setup object, with methods `calculate_day_a` and `calculate_day_b`
    /// available
    pub fn new(input_str: &str) -> Day17Setup {
        day17_parser::parse(input_str).unwrap()
    }

    fn in_area(&self, curr_x: isize, curr_y: isize) -> bool {
        (self.min_x..=self.max_x).contains(&curr_x) && (self.min_y..=self.max_y).contains(&curr_y)
    }

    fn test_trajectory(&self, speed_x: &isize, speed_y: &isize) -> bool {
        let mut curr_x = 0;
        let mut curr_y = 0;
        let mut speed_x = *speed_x;
        let mut speed_y = *speed_y;
        while curr_x <= self.max_x && curr_y >= self.min_y {
            curr_x += speed_x;
            curr_y += speed_y;
            if speed_x > 0 {
                speed_x -= 1
            };
            speed_y -= 1;
            if self.in_area(curr_x, curr_y) {
                return true;
            }
        }
        false
    }
    fn find_x_vals_to_check(&self) -> std::ops::RangeInclusive<isize> {
        // It should be possible to straight up calculate the final endpoint of x, using the gauss
        // calculation earlier, and backwards calculate that to the starting trajectory.
        // That is, if final_location = n(n+1)/2, then n ~= sqrt(destination*2)
        // let min = (self.min_x as f64 * 2.0).sqrt().floor() as isize;
        let max = (self.max_x as f64 * 2.0).sqrt().ceil() as isize;
        0..=(max * 40)
    }

    fn find_y_vals_to_check(&self) -> std::ops::RangeInclusive<isize> {
        // Assumption: no need to check any values greater than the diff in the y size. (Not quite
        // sure this holds generally, but supposedly its the reason in the example why the test
        // fails)
        let max = (self.max_y - self.min_y).abs() * 40;
        -max..=max
    }

    /// Calculate the part a response
    pub fn calculate_day_a(self: &Day17Setup) -> Option<isize> {
        let x_vals = self.find_x_vals_to_check();
        let y_vals = self.find_y_vals_to_check();
        let best_y = x_vals
            .cartesian_product(y_vals)
            .filter(|(x, y)| self.test_trajectory(x, y))
            .map(|(_, y)| y)
            .reduce(std::cmp::max)?;
        Some((best_y * (best_y + 1)) / 2)
    }

    /// Calculate the part b response
    pub fn calculate_day_b(self: &Day17Setup) -> usize {
        let x_vals = self.find_x_vals_to_check();
        let y_vals = self.find_y_vals_to_check();
        x_vals
            .cartesian_product(y_vals)
            .filter(|(x, y)| self.test_trajectory(x, y))
            .count()
    }
}

#[cfg(test)]
mod test {
    use crate::Day17Setup;

    #[test]
    fn test_parse() {
        let day17_setup = Day17Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day17_setup.min_x, 20);
        assert_eq!(day17_setup.max_x, 30);
        assert_eq!(day17_setup.min_y, -10);
        assert_eq!(day17_setup.max_y, -5);
    }

    #[test]
    fn test_trajectories_pass_or_fail() {
        let day17_setup = Day17Setup::new(include_str!("../test_data.txt"));
        assert!(day17_setup.test_trajectory(&7, &2));
        assert!(day17_setup.test_trajectory(&6, &3));
        assert!(day17_setup.test_trajectory(&9, &0));
        assert!(!day17_setup.test_trajectory(&17, &-4));
    }

    #[test]
    fn test_day_a() {
        let day17_setup = Day17Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day17_setup.calculate_day_a(), Some(45));
    }

    #[test]
    fn test_day_b() {
        let day17_setup = Day17Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day17_setup.calculate_day_b(), 112);
    }
}
