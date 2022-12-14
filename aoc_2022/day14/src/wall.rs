use std::collections::HashSet;

use itertools::Itertools;

use crate::point::Point;

#[derive(Debug, Eq, PartialEq)]
pub struct Walls {
    points: HashSet<Point>,
}

impl Walls {
    pub fn new(walls: &[Wall]) -> Walls {
        let mut points = HashSet::new();
        for wall in walls.iter() {
            for edge in wall.edges.iter() {
                for point in edge.iter_points() {
                    points.insert(point);
                }
            }
        }
        Walls { points }
    }

    pub fn intersects_with(&self, point: &Point) -> bool {
        self.points.contains(point)
    }

    pub fn get_max_y(&self) -> isize {
        self.points.iter().map(|p| p.y).max().unwrap()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Wall {
    edges: Vec<Edge>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Edge {
    pub start: Point,
    pub end: Point,
}

impl Wall {
    pub fn new(points: &[Point]) -> Wall {
        if points.len() <= 1 {
            panic!("Must have at least 2 points for a wall");
        }
        let edges = (0..(points.len() - 1))
            .map(|i| Edge {
                start: points[i].clone(),
                end: points[i + 1].clone(),
            })
            .collect();
        Wall { edges }
    }
}

impl Edge {
    fn iter_points(&self) -> Box<dyn Iterator<Item = Point>> {
        let min_x = self.start.x.min(self.end.x);
        let max_x = self.start.x.max(self.end.x);
        let min_y = self.start.y.min(self.end.y);
        let max_y = self.start.y.max(self.end.y);
        Box::new(
            (min_x..=max_x)
                .cartesian_product(min_y..=max_y)
                .map(|(x, y)| Point { x, y }),
        )
    }
}
