use std::ops::Range;

use aoc_helpers::point3d::Point3D;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub start: Point3D,
    pub end: Point3D,
    pub num_moves: usize,
}

pub trait Overlaps {
    fn overlaps(&self, other: &Self) -> bool;
}

impl Overlaps for Range<isize> {
    fn overlaps(&self, other: &Self) -> bool {
        !(self.start > other.end || other.start > self.end)
            && self.end > other.start
            && other.end > self.start
    }
}

impl Overlaps for Block {
    fn overlaps(&self, other: &Self) -> bool {
        (self.start.x..(self.end.x + 1)).overlaps(&(other.start.x..(other.end.x + 1)))
            && (self.start.y..(self.end.y + 1)).overlaps(&(other.start.y..(other.end.y + 1)))
            && (self.start.z..(self.end.z + 1)).overlaps(&(other.start.z..(other.end.z + 1)))
    }
}

impl Block {
    pub fn new(start: Point3D, end: Point3D) -> Block {
        Block {
            start,
            end,
            num_moves: 0,
        }
    }
    pub fn next(&self) -> Block {
        Block {
            start: self.start + Point3D { x: 0, y: 0, z: -1 },
            end: self.end + Point3D { x: 0, y: 0, z: -1 },
            num_moves: self.num_moves + 1,
        }
    }

    pub fn is_at_bottom(&self) -> bool {
        self.start.z == 1 || self.end.z == 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_range_overlaps() {
        assert!((0..8).overlaps(&(2..4)));
        assert!((2..4).overlaps(&(0..8)));
        assert!((0..6).overlaps(&(4..8)));
        assert!((4..8).overlaps(&(0..6)));
        assert!(!(0..2).overlaps(&(2..4)));
        assert!(!(2..4).overlaps(&(0..2)));
    }
}
