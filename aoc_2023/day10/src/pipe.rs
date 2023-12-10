use crate::point::Point;
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
    pub fn all_places_when_tiled(&self) -> Vec<Point> {
        match self {
            Pipe::Empty => vec![],
            _ => [self.neighbours(), vec![Point { x: 0, y: 0 }]]
                .concat()
                .into_iter()
                .map(|x| x + Point { x: 1, y: 1 })
                .collect(),
        }
    }

    pub fn neighbours(&self) -> Vec<Point> {
        match self {
            Pipe::TopBottom => vec![Point { x: 0, y: -1 }, Point { x: 0, y: 1 }],
            Pipe::LeftRight => vec![Point { x: -1, y: 0 }, Point { x: 1, y: 0 }],
            Pipe::TopLeft => vec![Point { x: -1, y: 0 }, Point { x: 0, y: -1 }],
            Pipe::TopRight => vec![Point { x: 1, y: 0 }, Point { x: 0, y: -1 }],
            Pipe::BottomLeft => vec![Point { x: -1, y: 0 }, Point { x: 0, y: 1 }],
            Pipe::BottomRight => vec![Point { x: 1, y: 0 }, Point { x: 0, y: 1 }],
            Pipe::Empty => vec![],
            Pipe::Start => vec![
                Point { x: -1, y: 0 },
                Point { x: 1, y: 0 },
                Point { x: 0, y: -1 },
                Point { x: 0, y: 1 },
            ],
        }
    }
}
