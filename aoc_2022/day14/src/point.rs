#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn is_on_board(&self, max: isize) -> bool {
        self.y < max
    }

    pub fn get_next_contenders(&self) -> [Point; 3] {
        [
            Point {
                x: self.x,
                y: self.y + 1,
            },
            Point {
                x: self.x - 1,
                y: self.y + 1,
            },
            Point {
                x: self.x + 1,
                y: self.y + 1,
            },
        ]
    }
}
