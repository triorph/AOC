mod parser;
use crate::parser::parse_data;
use aoc_helpers::{point2d::Point2D, read_input_file, AOCCalculator, AOCFileOrParseError};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day4 {
    data: Vec<Vec<char>>,
}

impl AOCCalculator for Day4 {
    fn new(filename: &str) -> Result<Day4, AOCFileOrParseError> {
        Ok(Day4 {
            data: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day4 {
    fn get_at_point(&self, point: &Point2D) -> Option<char> {
        if point.y >= 0
            && (point.y as usize) < self.data.len()
            && point.x >= 0
            && (point.x as usize) < self.data[point.y as usize].len()
        {
            Some(self.data[point.y as usize][point.x as usize])
        } else {
            None
        }
    }

    fn iter_all_points(&self) -> Box<dyn Iterator<Item = Point2D>> {
        Box::new(
            (0..self.data.len())
                .cartesian_product(0..self.data[0].len())
                .map(|(y, x)| Point2D::from_usize(x, y)),
        )
    }

    fn get_all_directions_a(&self) -> Box<dyn Iterator<Item = Vec<Point2D>>> {
        Box::new(
            (-1..=1)
                .cartesian_product(-1..=1)
                .filter(|(a, b)| !(*a == 0 && *b == 0))
                .map(|(x, y)| {
                    (0..4)
                        .map(|scale| &Point2D { x, y } * scale)
                        .collect::<Vec<Point2D>>()
                }),
        )
    }

    fn get_all_directions_b(&self) -> Vec<Vec<Point2D>> {
        vec![
            vec![
                Point2D { x: -1, y: -1 },
                Point2D { x: 0, y: 0 },
                Point2D { x: 1, y: 1 },
            ],
            vec![
                Point2D { x: 1, y: -1 },
                Point2D { x: 0, y: 0 },
                Point2D { x: -1, y: 1 },
            ],
        ]
    }

    fn convert_line_to_chars(&self, point: &Point2D, direction: &[Point2D]) -> Vec<Option<char>> {
        direction
            .iter()
            .map(|direction_point| self.get_at_point(&(direction_point + point)))
            .collect::<Vec<Option<char>>>()
    }

    fn calculate_day_a(&self) -> usize {
        self.iter_all_points()
            .map(|point| {
                self.get_all_directions_a()
                    .map(|direction| self.convert_line_to_chars(&point, &direction))
                    .filter(|line| *line == vec![Some('X'), Some('M'), Some('A'), Some('S')])
                    .count()
            })
            .sum()
    }

    fn calculate_day_b(&self) -> usize {
        self.iter_all_points()
            .map(|point| {
                self.get_all_directions_b()
                    .iter()
                    .map(|direction| self.convert_line_to_chars(&point, direction))
                    .filter(|line| {
                        *line == vec![Some('M'), Some('A'), Some('S')]
                            || *line == vec![Some('S'), Some('A'), Some('M')]
                    })
                    .count()
            })
            .filter(|line_counts| *line_counts == 2)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day4 = Day4::new("data/test_data.txt").unwrap();
        let expected = 18;
        let actual = day4.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day4 = Day4::new("data/test_data.txt").unwrap();
        let expected = 9;
        let actual = day4.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day4 = Day4::new("data/input_data.txt").unwrap();
        let expected = 2551;
        let actual = day4.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day4 = Day4::new("data/input_data.txt").unwrap();
        let expected = 1985;
        let actual = day4.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
