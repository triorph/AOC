mod chamber;
mod parser;
mod shapes;
mod types;
use crate::parser::parse_data;
use aoc_helpers::{read_input_file, AOCCalculator, AOCFileOrParseError};
use types::Direction;

use self::{chamber::Chamber, shapes::Shape, types::Point};

pub struct Day17 {
    directions: Vec<Direction>,
}

impl AOCCalculator for Day17 {
    fn new(filename: &str) -> Result<Day17, AOCFileOrParseError> {
        Ok(Day17 {
            directions: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

impl Day17 {
    fn calculate_day_a(&self) -> usize {
        self.simulate_n_steps(2022)
    }

    fn simulate_n_steps(&self, n: usize) -> usize {
        let mut chamber = Chamber::new(7);
        let mut shape = Shape::new(Point { x: 2, y: 3 });
        let mut count = 0;
        let mut shape_drop_count = 0;
        let mut state_at_end: Vec<(i32, usize, Shape, usize)> = Vec::new();
        let mut pseudo_height = 0;
        let downward_displacement = Point { x: 0, y: -1 };
        loop {
            for direction in self.directions.iter() {
                if chamber.can_shape_move(&shape, &direction.get_as_displacement()) {
                    shape.move_shape(&direction.get_as_displacement())
                }
                if chamber.can_shape_move(&shape, &downward_displacement) {
                    shape_drop_count += 1;
                    shape.move_shape(&downward_displacement);
                } else {
                    chamber.set_shape_down(&shape);
                    count += 1;
                    shape_drop_count = 0;
                    if count == n {
                        return chamber.get_maximum_height() + pseudo_height;
                    }
                    shape = shape.next_shape(Point {
                        x: 2,
                        y: 3 + chamber.get_maximum_height() as isize,
                    });
                }
            }
            for (old_drop_count, old_count, old_shape, old_height) in state_at_end.iter() {
                if *old_drop_count == shape_drop_count && old_shape.is_same_shape_type(&shape) {
                    let height_diff = chamber.get_maximum_height() - old_height;
                    let count_diff = count - old_count;
                    let counts_needed = n - count;
                    let pseudo_iterations = counts_needed / count_diff;
                    count += pseudo_iterations * count_diff;
                    pseudo_height += pseudo_iterations * height_diff;
                    break;
                }
            }
            state_at_end.push((shape_drop_count, count, shape, chamber.get_maximum_height()));
        }
    }

    fn calculate_day_b(&self) -> usize {
        self.simulate_n_steps(1000000000000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_calculate_day_a() {
        let day17 = Day17::new("data/test_data.txt").unwrap();
        let expected = 3068;
        let actual = day17.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day17 = Day17::new("data/test_data.txt").unwrap();
        let expected = 1514285714288;
        let actual = day17.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_a_real_input() {
        let day17 = Day17::new("data/input_data.txt").unwrap();
        let expected = 3184;
        let actual = day17.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b_real_input() {
        let day17 = Day17::new("data/input_data.txt").unwrap();
        let expected = 1577077363915;
        let actual = day17.calculate_day_b();
        assert_ne!(actual, 1568695652193);
        assert_eq!(expected, actual);
    }
}
