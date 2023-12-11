use crate::pipe::Pipe;
use aoc_helpers::point2d::Point2D;

pub trait Neighbours {
    fn neighbours(&self, pipe: &Pipe) -> Vec<Point2D>;
    fn all_neighbours(&self) -> Vec<Point2D>;
}

impl Neighbours for Point2D {
    fn neighbours(&self, pipe: &Pipe) -> Vec<Point2D> {
        pipe.neighbours().into_iter().map(|x| x + *self).collect()
    }

    fn all_neighbours(&self) -> Vec<Point2D> {
        self.neighbours(&Pipe::Start)
    }
}
