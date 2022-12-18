use std::fmt::Display;

use crate::{shapes::Shape, types::Point};

#[derive(Debug, Eq, PartialEq)]
pub struct Chamber {
    width: usize,
    height_offset: usize,
    map: Vec<Vec<bool>>,
}

impl Chamber {
    pub fn new(width: usize) -> Chamber {
        Chamber {
            width,
            height_offset: 0,
            map: vec![vec![false; width]; 50],
        }
    }

    fn is_location_free(&self, point: &Point) -> bool {
        if !(0..self.width).contains(&(point.x as usize)) || point.y < self.height_offset as isize {
            return false;
        }
        !self.map[(point.y - (self.height_offset as isize)) as usize][point.x as usize]
    }

    fn set_point(&mut self, point: &Point) {
        self.map[(point.y - (self.height_offset as isize)) as usize][point.x as usize] = true;
        if point.y as usize > self.height_offset + 40 {
            self.offset_height(1);
        }
    }

    pub fn can_shape_move(&self, shape: &Shape, displacement: &Point) -> bool {
        shape
            .get_points()
            .iter()
            .all(|point| self.is_location_free(&(point + displacement)))
    }

    pub fn set_shape_down(&mut self, shape: &Shape) {
        for point in shape.get_points().iter() {
            self.set_point(point);
        }
    }

    fn offset_height(&mut self, offset: usize) {
        let height = self.map.len();
        for y in 0..(height - offset) {
            for x in 0..self.width {
                self.map[y][x] = self.map[y + offset][x];
            }
        }
        for y in (height - offset)..height {
            for x in 0..self.width {
                self.map[y][x] = false;
            }
        }
        self.height_offset += offset;
    }

    pub fn get_maximum_height(&self) -> usize {
        for y in 0..self.map.len() {
            if (0..self.width).all(|x| {
                self.is_location_free(&Point {
                    x: x as isize,
                    y: (y + self.height_offset) as isize,
                })
            }) {
                return y + self.height_offset;
            }
        }
        panic!("No maximum height found")
    }
}

impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in (0..self.map.len()).rev() {
            for x in 0..self.width {
                write!(f, "{}", if self.map[y][x] { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        writeln!(
            f,
            "Chamber above with height offset: {}",
            self.height_offset,
        )
    }
}
