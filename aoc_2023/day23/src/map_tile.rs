use aoc_helpers::point2d::Point2D;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MapTile {
    Forest,
    Path,
    SlopeTopBottom,
    SlopeBottomTop,
    SlopeLeftRight,
    SlopeRightLeft,
}

impl MapTile {
    pub fn get_neighbour_deltas(&self) -> Vec<Point2D> {
        match self {
            MapTile::Forest => vec![],
            MapTile::Path => vec![
                Point2D { x: -1, y: 0 },
                Point2D { x: 1, y: 0 },
                Point2D { x: 0, y: -1 },
                Point2D { x: 0, y: 1 },
            ],
            MapTile::SlopeTopBottom => vec![Point2D { x: 0, y: 1 }],
            MapTile::SlopeBottomTop => vec![Point2D { x: 0, y: -1 }],
            MapTile::SlopeLeftRight => vec![Point2D { x: 1, y: 0 }],
            MapTile::SlopeRightLeft => vec![Point2D { x: -1, y: 0 }],
        }
    }

    pub fn is_slope_tile(&self) -> bool {
        matches!(
            self,
            MapTile::SlopeTopBottom
                | MapTile::SlopeBottomTop
                | MapTile::SlopeLeftRight
                | MapTile::SlopeRightLeft
        )
    }

    pub fn is_path(&self) -> bool {
        matches!(self, MapTile::Path)
    }

    pub fn is_path_or_slope_tile(&self) -> bool {
        self.is_path() || self.is_slope_tile()
    }
}
