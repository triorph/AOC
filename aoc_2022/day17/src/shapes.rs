use crate::types::Point;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Shape {
    Bar(Bar),
    Plus(Plus),
    Pipe(Pipe),
    Corner(Corner),
    Square(Square),
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Bar {
    pub location: Point,
}
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Plus {
    pub location: Point,
}
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Pipe {
    pub location: Point,
}
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Corner {
    pub location: Point,
}
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Square {
    pub location: Point,
}

impl Plus {
    pub fn get_points(&self) -> Vec<Point> {
        vec![
            Point {
                x: self.location.x + 1,
                y: self.location.y,
            },
            Point {
                x: self.location.x,
                y: self.location.y + 1,
            },
            Point {
                x: self.location.x + 1,
                y: self.location.y + 1,
            },
            Point {
                x: self.location.x + 2,
                y: self.location.y + 1,
            },
            Point {
                x: self.location.x + 1,
                y: self.location.y + 2,
            },
        ]
    }
}

impl Bar {
    pub fn get_points(&self) -> Vec<Point> {
        vec![
            Point {
                x: self.location.x,
                y: self.location.y,
            },
            Point {
                x: self.location.x + 1,
                y: self.location.y,
            },
            Point {
                x: self.location.x + 2,
                y: self.location.y,
            },
            Point {
                x: self.location.x + 3,
                y: self.location.y,
            },
        ]
    }
}

impl Pipe {
    pub fn get_points(&self) -> Vec<Point> {
        vec![
            Point {
                x: self.location.x,
                y: self.location.y,
            },
            Point {
                x: self.location.x,
                y: self.location.y + 1,
            },
            Point {
                x: self.location.x,
                y: self.location.y + 2,
            },
            Point {
                x: self.location.x,
                y: self.location.y + 3,
            },
        ]
    }
}

impl Corner {
    pub fn get_points(&self) -> Vec<Point> {
        vec![
            Point {
                x: self.location.x,
                y: self.location.y,
            },
            Point {
                x: self.location.x + 1,
                y: self.location.y,
            },
            Point {
                x: self.location.x + 2,
                y: self.location.y,
            },
            Point {
                x: self.location.x + 2,
                y: self.location.y + 1,
            },
            Point {
                x: self.location.x + 2,
                y: self.location.y + 2,
            },
        ]
    }
}

impl Square {
    pub fn get_points(&self) -> Vec<Point> {
        vec![
            Point {
                x: self.location.x,
                y: self.location.y,
            },
            Point {
                x: self.location.x + 1,
                y: self.location.y,
            },
            Point {
                x: self.location.x,
                y: self.location.y + 1,
            },
            Point {
                x: self.location.x + 1,
                y: self.location.y + 1,
            },
        ]
    }
}

impl Shape {
    pub fn new(new_starting_position: Point) -> Shape {
        Shape::Bar(Bar {
            location: new_starting_position,
        })
    }

    pub fn next_shape(&self, new_starting_position: Point) -> Shape {
        match self {
            Shape::Bar(_) => Shape::Plus(Plus {
                location: new_starting_position,
            }),
            Shape::Plus(_) => Shape::Corner(Corner {
                location: new_starting_position,
            }),
            Shape::Corner(_) => Shape::Pipe(Pipe {
                location: new_starting_position,
            }),
            Shape::Pipe(_) => Shape::Square(Square {
                location: new_starting_position,
            }),
            Shape::Square(_) => Shape::Bar(Bar {
                location: new_starting_position,
            }),
        }
    }

    pub fn get_points(&self) -> Vec<Point> {
        match self {
            Shape::Bar(inner) => inner.get_points(),
            Shape::Plus(inner) => inner.get_points(),
            Shape::Corner(inner) => inner.get_points(),
            Shape::Pipe(inner) => inner.get_points(),
            Shape::Square(inner) => inner.get_points(),
        }
    }

    pub fn move_shape(&mut self, displacement: &Point) {
        match self {
            Shape::Bar(inner) => inner.location = &inner.location + displacement,
            Shape::Plus(inner) => inner.location = &inner.location + displacement,
            Shape::Corner(inner) => inner.location = &inner.location + displacement,
            Shape::Pipe(inner) => inner.location = &inner.location + displacement,
            Shape::Square(inner) => inner.location = &inner.location + displacement,
        }
    }

    pub fn is_same_shape_type(&self, other: &Shape) -> bool {
        matches!(
            (self, other),
            (Shape::Bar(_), Shape::Bar(_))
                | (Shape::Plus(_), Shape::Plus(_))
                | (Shape::Corner(_), Shape::Corner(_))
                | (Shape::Pipe(_), Shape::Pipe(_))
                | (Shape::Square(_), Shape::Square(_))
        )
    }
}
