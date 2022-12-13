#[derive(Debug, Eq, PartialEq, Clone)]
pub enum MapPoint {
    Start,
    End,
    Other(u32),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl MapPoint {
    pub fn get_value(&self) -> u32 {
        match self {
            MapPoint::Start => 0,
            MapPoint::End => 25,
            MapPoint::Other(val) => *val,
        }
    }
}

impl Point {
    pub fn get_neighbours(&self) -> Vec<Point> {
        let mut ret = vec![
            Point {
                x: self.x + 1,
                y: self.y,
            },
            Point {
                x: self.x,
                y: self.y + 1,
            },
        ];
        if self.x > 0 {
            ret.push(Point {
                x: self.x - 1,
                y: self.y,
            });
        }
        if self.y > 0 {
            ret.push(Point {
                x: self.x,
                y: self.y - 1,
            });
        }
        ret
    }
}
