extern crate peg;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
pub struct VentLine {
    start: Point,
    end: Point,
}

pub struct VentLayout {
    vent_lines: Vec<VentLine>,
}

peg::parser! { grammar day5_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule point() -> Point
        = x:number() "," y:number() { Point {x, y} }
    rule vent_line() -> VentLine
        =  start:point() " -> " end:point() { VentLine {start, end} }
    pub rule parse() -> VentLayout
        = vent_lines:vent_line() ** ("\n" +) "\n" * {
            VentLayout{ vent_lines }
        }

}
}

impl VentLayout {
    fn new(vent_layout_input_str: &str) -> VentLayout {
        day5_parser::parse(vent_layout_input_str).unwrap()
    }
    fn calculate_day_a(self: &VentLayout) -> usize {
        let mut map = HashMap::new();
        for line in self.vent_lines.iter() {
            let points = line.get_points_from_line_day_a();
            for point in points.into_iter() {
                let entry = map.entry(point).or_insert(0);
                *entry += 1;
            }
        }
        map.values().filter(|val| **val > 1).count()
    }

    fn calculate_day_b(self: &VentLayout) -> usize {
        let mut map = HashMap::new();
        for line in self.vent_lines.iter() {
            let points = line.get_points_from_line_day_b();
            for point in points.into_iter() {
                let entry = map.entry(point).or_insert(0);
                *entry += 1;
            }
        }
        map.values().filter(|val| **val > 1).count()
    }
}

impl VentLine {
    fn is_vertical(self: &VentLine) -> bool {
        self.start.x == self.end.x
    }

    fn get_vertical_points(self: &VentLine) -> Vec<Point> {
        let mut ret = vec![];
        let x = self.start.x;
        let start = min(self.start.y, self.end.y);
        let end = max(self.start.y, self.end.y);
        for y in start..=end {
            ret.push(Point { x, y });
        }
        ret
    }

    fn is_horizontal(self: &VentLine) -> bool {
        self.start.y == self.end.y
    }

    fn get_horizontal_points(self: &VentLine) -> Vec<Point> {
        let mut ret = vec![];
        let y = self.start.y;
        let start = min(self.start.x, self.end.x);
        let end = max(self.start.x, self.end.x);
        for x in start..=end {
            ret.push(Point { x, y });
        }
        ret
    }

    fn get_points_from_line_day_a(self: &VentLine) -> Vec<Point> {
        if self.is_vertical() {
            self.get_vertical_points()
        } else if self.is_horizontal() {
            self.get_horizontal_points()
        } else {
            vec![]
        }
    }
    fn is_diagonal(self: &VentLine) -> bool {
        (self.end.y as isize - self.start.y as isize).abs()
            == (self.end.x as isize - self.start.x as isize).abs()
    }

    fn get_diagonal_points(self: &VentLine) -> Vec<Point> {
        let mut ret = vec![];
        let x_vals = match self.end.x < self.start.x {
            true => (self.end.x..=self.start.x).rev().collect::<Vec<usize>>(),
            false => (self.start.x..=self.end.x).collect::<Vec<usize>>(),
        };
        let y_vals = match self.end.y < self.start.y {
            true => (self.end.y..=self.start.y).rev().collect::<Vec<usize>>(),
            false => (self.start.y..=self.end.y).collect::<Vec<usize>>(),
        };
        for (x, y) in x_vals.into_iter().zip(y_vals.into_iter()) {
            ret.push(Point { x, y })
        }
        ret
    }

    fn get_points_from_line_day_b(self: &VentLine) -> Vec<Point> {
        if self.is_vertical() {
            self.get_vertical_points()
        } else if self.is_horizontal() {
            self.get_horizontal_points()
        } else if self.is_diagonal() {
            self.get_diagonal_points()
        } else {
            vec![]
        }
    }
}

fn main() {
    let vent_layout = VentLayout::new(include_str!("../input_data.txt"));
    let day_a = vent_layout.calculate_day_a();
    println!("Day a result: {}", day_a);
    let day_b = vent_layout.calculate_day_b();
    println!("Day b result: {}", day_b);
}

#[cfg(test)]
mod test {
    use crate::Point;
    use crate::VentLayout;
    use crate::VentLine;

    #[test]
    fn test_parse() {
        let vent_layout = VentLayout::new(include_str!("../test_data.txt"));
        assert_eq!(vent_layout.vent_lines.len(), 10);
    }

    #[test]
    fn test_horizontal_points() {
        let vent_line = VentLine {
            start: Point { x: 3, y: 6 },
            end: Point { x: 5, y: 6 },
        };
        let expected = [
            Point { x: 3, y: 6 },
            Point { x: 4, y: 6 },
            Point { x: 5, y: 6 },
        ];
        assert_eq!(vent_line.get_points_from_line_day_a(), expected);
        assert_eq!(vent_line.get_points_from_line_day_b(), expected);
        let vent_line = VentLine {
            start: Point { x: 5, y: 6 },
            end: Point { x: 3, y: 6 },
        };
        assert_eq!(vent_line.get_points_from_line_day_a(), expected);
        assert_eq!(vent_line.get_points_from_line_day_b(), expected);
    }

    #[test]
    fn test_vertical_points() {
        let vent_line = VentLine {
            start: Point { x: 3, y: 4 },
            end: Point { x: 3, y: 6 },
        };
        let expected = [
            Point { x: 3, y: 4 },
            Point { x: 3, y: 5 },
            Point { x: 3, y: 6 },
        ];
        assert_eq!(vent_line.get_points_from_line_day_a(), expected);
        assert_eq!(vent_line.get_points_from_line_day_b(), expected);
        let vent_line = VentLine {
            start: Point { x: 3, y: 6 },
            end: Point { x: 3, y: 4 },
        };
        assert_eq!(vent_line.get_points_from_line_day_a(), expected);
        assert_eq!(vent_line.get_points_from_line_day_b(), expected);
    }

    #[test]
    fn test_other_points() {
        let vent_line = VentLine {
            start: Point { x: 3, y: 4 },
            end: Point { x: 6, y: 6 },
        };
        assert_eq!(vent_line.get_points_from_line_day_a(), []);
        assert_eq!(vent_line.get_points_from_line_day_b(), []);
    }

    #[test]
    fn test_diagonal_points_a() {
        let vent_line = VentLine {
            start: Point { x: 3, y: 3 },
            end: Point { x: 6, y: 6 },
        };
        assert_eq!(vent_line.get_points_from_line_day_a(), []);
        let vent_line = VentLine {
            start: Point { x: 3, y: 6 },
            end: Point { x: 6, y: 3 },
        };
        assert_eq!(vent_line.get_points_from_line_day_a(), []);
        let vent_line = VentLine {
            start: Point { x: 6, y: 3 },
            end: Point { x: 3, y: 6 },
        };
        assert_eq!(vent_line.get_points_from_line_day_a(), []);
        let vent_line = VentLine {
            start: Point { x: 6, y: 6 },
            end: Point { x: 3, y: 3 },
        };
        assert_eq!(vent_line.get_points_from_line_day_a(), []);
    }

    #[test]
    fn test_diagonal_points_b() {
        let expected = [
            Point { x: 3, y: 3 },
            Point { x: 4, y: 4 },
            Point { x: 5, y: 5 },
            Point { x: 6, y: 6 },
        ];
        let vent_line = VentLine {
            start: Point { x: 3, y: 3 },
            end: Point { x: 6, y: 6 },
        };
        assert_eq!(vent_line.get_points_from_line_day_b(), expected);
        let expected = [
            Point { x: 3, y: 6 },
            Point { x: 4, y: 5 },
            Point { x: 5, y: 4 },
            Point { x: 6, y: 3 },
        ];
        let vent_line = VentLine {
            start: Point { x: 3, y: 6 },
            end: Point { x: 6, y: 3 },
        };
        assert_eq!(vent_line.get_points_from_line_day_b(), expected);
        let expected = [
            Point { x: 6, y: 3 },
            Point { x: 5, y: 4 },
            Point { x: 4, y: 5 },
            Point { x: 3, y: 6 },
        ];
        let vent_line = VentLine {
            start: Point { x: 6, y: 3 },
            end: Point { x: 3, y: 6 },
        };
        assert_eq!(vent_line.get_points_from_line_day_b(), expected);
        let expected = [
            Point { x: 6, y: 6 },
            Point { x: 5, y: 5 },
            Point { x: 4, y: 4 },
            Point { x: 3, y: 3 },
        ];
        let vent_line = VentLine {
            start: Point { x: 6, y: 6 },
            end: Point { x: 3, y: 3 },
        };
        assert_eq!(vent_line.get_points_from_line_day_b(), expected);
    }

    #[test]
    fn test_day_a() {
        let vent_layout = VentLayout::new(include_str!("../test_data.txt"));
        assert_eq!(vent_layout.calculate_day_a(), 5);
    }

    #[test]
    fn test_day_b() {
        let vent_layout = VentLayout::new(include_str!("../test_data.txt"));
        assert_eq!(vent_layout.calculate_day_b(), 12);
    }
}
