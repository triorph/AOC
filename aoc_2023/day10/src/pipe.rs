use aoc_helpers::point2d::Point2D;

#[derive(Debug, PartialEq, Eq)]
pub enum Pipe {
    TopBottom,
    LeftRight,
    BottomLeft,
    TopLeft,
    BottomRight,
    TopRight,
    Empty,
    Start,
}

impl Pipe {
    pub fn all_places_when_tiled(&self) -> Vec<Point2D> {
        match self {
            Pipe::Empty => vec![],
            _ => [self.neighbours(), vec![Point2D { x: 0, y: 0 }]]
                .concat()
                .into_iter()
                .map(|x| x + Point2D { x: 1, y: 1 })
                .collect(),
        }
    }

    pub fn neighbours(&self) -> Vec<Point2D> {
        match self {
            Pipe::TopBottom => vec![Point2D { x: 0, y: -1 }, Point2D { x: 0, y: 1 }],
            Pipe::LeftRight => vec![Point2D { x: -1, y: 0 }, Point2D { x: 1, y: 0 }],
            Pipe::TopLeft => vec![Point2D { x: -1, y: 0 }, Point2D { x: 0, y: -1 }],
            Pipe::TopRight => vec![Point2D { x: 1, y: 0 }, Point2D { x: 0, y: -1 }],
            Pipe::BottomLeft => vec![Point2D { x: -1, y: 0 }, Point2D { x: 0, y: 1 }],
            Pipe::BottomRight => vec![Point2D { x: 1, y: 0 }, Point2D { x: 0, y: 1 }],
            Pipe::Empty => vec![],
            Pipe::Start => vec![
                Point2D { x: -1, y: 0 },
                Point2D { x: 1, y: 0 },
                Point2D { x: 0, y: -1 },
                Point2D { x: 0, y: 1 },
            ],
        }
    }
}
