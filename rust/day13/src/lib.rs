extern crate peg;

#[derive(Clone)]
enum Fold {
    FoldX(usize),
    FoldY(usize),
}

#[derive(PartialEq)]
struct Point(usize, usize);

pub struct Day13Setup {
    points: Vec<Point>,
    folds: Vec<Fold>,
}

peg::parser! { grammar day13_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule point() -> Point
        = x:number() "," y:number() { Point(x, y) }
    rule fold_x_type() -> &'input str
        = "x" {"x"}
    rule fold_y_type() -> &'input str
        = "y" {"y"}
    rule fold_type() -> &'input str
        = fold_type: (fold_x_type() / fold_y_type()) {fold_type}
    rule fold() -> Fold
        = "fold along " fold_type:fold_type() "=" n:number() {
            if fold_type == "x" {
                Fold::FoldX(n)
            } else {
                Fold::FoldY(n)
            }
        }
    pub rule parse() -> Day13Setup
        = points:point() ++ "\n" "\n" * folds:fold() ++ "\n" "\n" * {
            Day13Setup { points, folds }
        }
}}

impl Point {
    fn fold_x(self: &Point, val: usize) -> Point {
        if self.0 < val {
            Point(self.0, self.1)
        } else {
            let x_diff = self.0 - val;
            if x_diff > val {
                panic!("fold would create a negative x point");
            }
            Point(val - x_diff, self.1)
        }
    }

    fn fold_y(self: &Point, val: usize) -> Point {
        if self.1 < val {
            Point(self.0, self.1)
        } else {
            let y_diff = self.1 - val;
            if y_diff > val {
                panic!("fold would create a negative y point");
            }
            Point(self.0, val - y_diff)
        }
    }
}

impl std::fmt::Display for Day13Setup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret = String::new();
        let max = self.get_max_point();
        for y in 0..=max.1 {
            for x in 0..=max.0 {
                ret += match self.points.contains(&Point(x, y)) {
                    true => "#",
                    false => ".",
                }
            }
            ret += "\n";
        }
        write!(f, "{}", ret)
    }
}

impl Day13Setup {
    /// Generates a new Day13Setup object to be calculated upon
    ///
    /// Inputs: the full string contents of the input data set.
    ///
    /// Returns: A new Day13Setup object, with methods `calculate_day_a` and `calculate_day_b`
    /// available
    pub fn new(input_str: &str) -> Day13Setup {
        day13_parser::parse(input_str).unwrap()
    }

    fn get_max_point(self: &Day13Setup) -> Point {
        let x = self
            .points
            .iter()
            .map(|p| p.0)
            .reduce(std::cmp::max)
            .unwrap();
        let y = self
            .points
            .iter()
            .map(|p| p.1)
            .reduce(std::cmp::max)
            .unwrap();
        Point(x, y)
    }

    fn perform_fold_x(self: &Day13Setup, val: usize) -> Day13Setup {
        let mut folded_points: Vec<Point> = vec![];
        for point in self.points.iter() {
            let next_point = point.fold_x(val);
            if !folded_points.contains(&next_point) {
                folded_points.push(next_point);
            }
        }
        let next_folds: Vec<Fold> = self.folds[1..].to_vec();
        Day13Setup {
            points: folded_points,
            folds: next_folds,
        }
    }
    fn perform_fold_y(self: &Day13Setup, val: usize) -> Day13Setup {
        let mut folded_points: Vec<Point> = vec![];
        for point in self.points.iter() {
            let next_point = point.fold_y(val);
            if !folded_points.contains(&next_point) {
                folded_points.push(next_point);
            }
        }
        let next_folds: Vec<Fold> = self.folds[1..].to_vec();
        Day13Setup {
            points: folded_points,
            folds: next_folds,
        }
    }

    fn perform_fold(self: &Day13Setup, fold: &Fold) -> Day13Setup {
        match fold {
            Fold::FoldX(val) => self.perform_fold_x(*val),
            Fold::FoldY(val) => self.perform_fold_y(*val),
        }
    }

    /// Calculate the part a response
    pub fn calculate_day_a(self: &Day13Setup) -> usize {
        let next = self.perform_fold(&self.folds[0]);
        next.points.len()
    }

    /// Calculate the part b response
    pub fn calculate_day_b(self: Day13Setup) {
        let mut next = self;
        while !next.folds.is_empty() {
            next = next.perform_fold(&next.folds[0])
        }
        println!("{}", next);
    }
}

#[cfg(test)]
mod test {
    use crate::Day13Setup;

    #[test]
    fn test_parse() {
        let day13_setup = Day13Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day13_setup.points.len(), 18);
        assert_eq!(day13_setup.folds.len(), 2);
    }

    #[test]
    fn test_day_a() {
        let day13_setup = Day13Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day13_setup.calculate_day_a(), 17);
    }
}
