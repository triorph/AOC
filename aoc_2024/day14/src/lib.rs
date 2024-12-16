mod parser;
use crate::parser::parse_data;
use aoc_helpers::{point2d::Point2D, read_input_file, AOCFileOrParseError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day14 {
    robots: Vec<(Point2D, Point2D)>,
    width: isize,
    height: isize,
}

impl Day14 {
    pub fn new(filename: &str, width: isize, height: isize) -> Result<Day14, AOCFileOrParseError> {
        Ok(Day14 {
            robots: parse_data(&read_input_file(filename)?)?,
            width,
            height,
        })
    }

    pub fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day14 {
    fn moved_robot(&self, robot: &(Point2D, Point2D)) -> (Point2D, Point2D) {
        (
            Point2D {
                x: (robot.0.x + robot.1.x + self.width) % self.width,
                y: (robot.0.y + robot.1.y + self.height) % self.height,
            },
            robot.1,
        )
    }

    fn next_moved_robots(&self, robots: &[(Point2D, Point2D)]) -> Vec<(Point2D, Point2D)> {
        robots.iter().map(|robot| self.moved_robot(robot)).collect()
    }

    fn calculate_score(&self, robots: &[(Point2D, Point2D)]) -> usize {
        let quadrants = [
            (0..self.width / 2, 0..self.height / 2),
            ((self.width / 2 + 1)..self.width, 0..self.height / 2),
            (0..self.width / 2, (self.height / 2 + 1)..self.height),
            (
                (self.width / 2 + 1)..self.width,
                (self.height / 2 + 1)..self.height,
            ),
        ];
        quadrants
            .into_iter()
            .map(|(x_range, y_range)| {
                robots
                    .iter()
                    .filter(|(p, _)| x_range.contains(&p.x) && y_range.contains(&p.y))
                    .count()
            })
            .product()
    }

    fn calculate_day_a(&self) -> usize {
        let moved_robots = (0..100).fold(self.robots.clone(), |robots, _| {
            self.next_moved_robots(&robots)
        });
        self.calculate_score(&moved_robots)
    }

    fn print_robots(&self, robots: &[(Point2D, Point2D)]) -> String {
        let mut ret = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if robots.iter().any(|(p, _)| p.x == x && p.y == y) {
                    ret += "#"
                } else {
                    ret += " "
                }
            }
            ret += "\n"
        }
        ret
    }

    fn calculate_day_b(&self) -> usize {
        let mut robots = self.robots.clone();
        for i in 0..10000 {
            let robot_str = self.print_robots(&robots);
            if robot_str.contains("#########") {
                println!("Tree :");
                println!("{}", self.print_robots(&robots));
                println!();
                return i;
            }
            robots = self.next_moved_robots(&robots);
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day14 = Day14::new("data/test_data.txt", 11, 7).unwrap();
        let expected = 12;
        let actual = day14.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day14 = Day14::new("data/input_data.txt", 101, 103).unwrap();
        let expected = 211692000;
        let actual = day14.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day14 = Day14::new("data/input_data.txt", 101, 103).unwrap();
        let expected = 6587;
        let actual = day14.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
