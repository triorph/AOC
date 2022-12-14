use crate::point::Point;

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

    pub fn intersects_with(&self, point: &Point) -> bool {
        self.edges.iter().any(|edge| edge.intersects_with(point))
    }

    pub fn get_max_y(&self) -> isize {
        self.edges.iter().map(Edge::get_max_y).max().unwrap()
    }
}

impl Edge {
    fn intersects_with(&self, point: &Point) -> bool {
        let min_x = self.start.x.min(self.end.x);
        let max_x = self.start.x.max(self.end.x);
        let min_y = self.start.y.min(self.end.y);
        let max_y = self.start.y.max(self.end.y);
        (min_x..=max_x).contains(&point.x) && (min_y..=max_y).contains(&point.y)
    }

    fn get_max_y(&self) -> isize {
        self.start.y.max(self.end.y)
    }
}
