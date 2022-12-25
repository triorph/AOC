use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.x + self.y > other.x + other.y {
            Some(Ordering::Greater)
        } else if self.x + self.y < other.x + other.y {
            Some(Ordering::Less)
        } else if self.y > other.y {
            Some(Ordering::Greater)
        } else if self.y < other.y {
            Some(Ordering::Less)
        } else {
            None
        }
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.x + self.y > other.x + other.y {
            Ordering::Greater
        } else if self.x + self.y < other.x + other.y {
            Ordering::Less
        } else if self.y > other.y {
            Ordering::Greater
        } else if self.y < other.y {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl Point {
    pub fn next_moves(&self) -> Vec<Point> {
        vec![
            Point {
                x: self.x + 1,
                y: self.y,
            },
            Point {
                x: self.x,
                y: self.y + 1,
            },
            Point {
                x: self.x,
                y: self.y,
            },
            Point {
                x: self.x - 1,
                y: self.y,
            },
            Point {
                x: self.x,
                y: self.y - 1,
            },
        ]
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub enum Tile {
    Wall,
    Floor,
    Blizzard(Direction),
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Blizzard {
    direction: Direction,
    pub location: Point,
}

impl Blizzard {
    fn next(&self, max_x: usize, max_y: usize) -> Blizzard {
        let mut location = match self.direction {
            Direction::Up => Point {
                x: self.location.x,
                y: self.location.y - 1,
            },
            Direction::Left => Point {
                x: self.location.x - 1,
                y: self.location.y,
            },
            Direction::Down => Point {
                x: self.location.x,
                y: self.location.y + 1,
            },
            Direction::Right => Point {
                x: self.location.x + 1,
                y: self.location.y,
            },
        };
        if location.x < 0 {
            location.x = max_x as isize - 1;
        }
        if location.y < 0 {
            location.y = max_y as isize - 1;
        }
        if location.x >= max_x as isize {
            location.x = 0;
        };
        if location.y >= max_y as isize {
            location.y = 0;
        }
        Blizzard {
            location,
            direction: self.direction,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct AllBlizzards {
    blizzards_at_time: Vec<Vec<Blizzard>>,
    pub width: usize,
    pub height: usize,
}

impl AllBlizzards {
    pub fn new(tiles: &[Vec<Tile>]) -> AllBlizzards {
        let mut blizzards = Vec::new();
        for y in 0..tiles.len() {
            for x in 0..tiles[0].len() {
                if let Tile::Blizzard(dir) = tiles[y][x] {
                    blizzards.push(Blizzard {
                        direction: dir,
                        location: Point {
                            x: x as isize - 1,
                            y: y as isize - 1,
                        },
                    })
                }
            }
        }
        AllBlizzards {
            blizzards_at_time: vec![blizzards],
            width: tiles[0].len() - 2,
            height: tiles.len() - 2,
        }
    }

    pub fn get_blizzards_at_time(&mut self, time: usize) -> &[Blizzard] {
        if time >= self.blizzards_at_time.len() {
            let (width, height) = (self.width, self.height);
            let prev_blizzards = self.get_blizzards_at_time(time - 1);
            let next_blizzards: Vec<Blizzard> = prev_blizzards
                .iter()
                .map(|blizzard| blizzard.next(width, height))
                .collect();
            // println!("next blizzards for time {:?} are:", time);
            // for y in 0..self.height {
            //     for x in 0..self.width {
            //         if next_blizzards.iter().map(|b| b.location).any(|p| {
            //             p == Point {
            //                 x: x as isize,
            //                 y: y as isize,
            //             }
            //         }) {
            //             print!("B");
            //         } else {
            //             print!(".");
            //         }
            //     }
            //     println!();
            // }
            self.blizzards_at_time.push(next_blizzards);
        }
        self.blizzards_at_time.last().unwrap()
    }

    pub fn move_in_blizzard(&self, next_move: &Point) -> bool {
        next_move == &Point { x: 0, y: -1 }
            || next_move
                == &Point {
                    x: self.width as isize - 1,
                    y: self.height as isize,
                }
            || ((0..self.width as isize).contains(&next_move.x)
                && (0..self.height as isize).contains(&next_move.y))
    }

    pub fn is_move_valid(&mut self, next_move: &Point, at_time: usize) -> bool {
        !self
            .get_blizzards_at_time(at_time)
            .iter()
            .map(|blizzard| blizzard.location)
            .any(|x| x == *next_move)
            && self.move_in_blizzard(next_move)
    }
}
