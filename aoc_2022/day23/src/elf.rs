use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Elf {
    pub x: isize,
    pub y: isize,
}

enum Move {
    North,
    South,
    West,
    East,
}

const MOVES: [Move; 4] = [Move::North, Move::South, Move::West, Move::East];

impl Elf {
    pub fn new(x: usize, y: usize) -> Elf {
        Elf {
            x: x as isize,
            y: y as isize,
        }
    }

    fn all_empty(&self, other_elves: &HashSet<Elf>) -> bool {
        [
            Elf {
                x: self.x - 1,
                y: self.y - 1,
            },
            Elf {
                x: self.x,
                y: self.y - 1,
            },
            Elf {
                x: self.x + 1,
                y: self.y - 1,
            },
            Elf {
                x: self.x - 1,
                y: self.y,
            },
            Elf {
                x: self.x + 1,
                y: self.y,
            },
            Elf {
                x: self.x - 1,
                y: self.y + 1,
            },
            Elf {
                x: self.x,
                y: self.y + 1,
            },
            Elf {
                x: self.x + 1,
                y: self.y + 1,
            },
        ]
        .into_iter()
        .all(|neighbour| !other_elves.contains(&neighbour))
    }

    pub fn can_move_north(&self, other_elves: &HashSet<Elf>) -> bool {
        [
            Elf {
                x: self.x - 1,
                y: self.y - 1,
            },
            Elf {
                x: self.x,
                y: self.y - 1,
            },
            Elf {
                x: self.x + 1,
                y: self.y - 1,
            },
        ]
        .into_iter()
        .all(|neighbour| !other_elves.contains(&neighbour))
    }

    pub fn can_move_south(&self, other_elves: &HashSet<Elf>) -> bool {
        [
            Elf {
                x: self.x - 1,
                y: self.y + 1,
            },
            Elf {
                x: self.x,
                y: self.y + 1,
            },
            Elf {
                x: self.x + 1,
                y: self.y + 1,
            },
        ]
        .into_iter()
        .all(|neighbour| !other_elves.contains(&neighbour))
    }

    pub fn can_move_west(&self, other_elves: &HashSet<Elf>) -> bool {
        [
            Elf {
                x: self.x - 1,
                y: self.y - 1,
            },
            Elf {
                x: self.x - 1,
                y: self.y,
            },
            Elf {
                x: self.x - 1,
                y: self.y + 1,
            },
        ]
        .into_iter()
        .all(|neighbour| !other_elves.contains(&neighbour))
    }

    pub fn can_move_east(&self, other_elves: &HashSet<Elf>) -> bool {
        [
            Elf {
                x: self.x + 1,
                y: self.y - 1,
            },
            Elf {
                x: self.x + 1,
                y: self.y,
            },
            Elf {
                x: self.x + 1,
                y: self.y + 1,
            },
        ]
        .into_iter()
        .all(|neighbour| !other_elves.contains(&neighbour))
    }

    pub fn propose_move(&self, time: usize, other_elves: &HashSet<Elf>) -> Elf {
        if self.all_empty(other_elves) {
            return *self;
        }
        for i in time..time + 4 {
            match MOVES[i % 4] {
                Move::North => {
                    if self.can_move_north(other_elves) {
                        return Elf {
                            x: self.x,
                            y: self.y - 1,
                        };
                    }
                }
                Move::South => {
                    if self.can_move_south(other_elves) {
                        return Elf {
                            x: self.x,
                            y: self.y + 1,
                        };
                    }
                }
                Move::West => {
                    if self.can_move_west(other_elves) {
                        return Elf {
                            x: self.x - 1,
                            y: self.y,
                        };
                    }
                }
                Move::East => {
                    if self.can_move_east(other_elves) {
                        return Elf {
                            x: self.x + 1,
                            y: self.y,
                        };
                    }
                }
            }
        }
        *self
    }
}
