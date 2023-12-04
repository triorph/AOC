use std::cmp::{max, min};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchematicValue {
    Blank,
    Symbol,
    GearSymbol,
    Digit(usize),
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn get_neighbours_to_other(&self, other: &Point) -> HashSet<Point> {
        let min_x = max(0, min(self.x as isize, other.x as isize) - 1) as usize;
        let min_y = max(0, min(self.y as isize, other.y as isize) - 1) as usize;
        let max_x = max(self.x, other.x) + 1;
        let max_y = max(self.y, other.y) + 1;
        let mut ret = HashSet::new();
        for x in min_x..=max_x {
            // top
            ret.insert(Point { x, y: min_y });
            // bottom
            ret.insert(Point { x, y: max_y });
        }
        for y in min_y..=max_y {
            // left
            ret.insert(Point { x: min_x, y });
            // right
            ret.insert(Point { x: max_x, y });
        }
        ret
    }

    fn get_neighbours(&self) -> HashSet<Point> {
        let min_x = max(0, self.x as isize - 1) as usize;
        let max_x = self.x + 1;
        let min_y = max(0, self.y as isize - 1) as usize;
        let max_y = self.y + 1;
        let mut ret = HashSet::new();
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                if x != self.x || y != self.y {
                    ret.insert(Point { x, y });
                }
            }
        }
        return ret;
    }
}

#[derive(Debug)]
pub struct SchematicFullNumber {
    pub number: usize,
    start: Point,
    end: Point,
}

impl SchematicFullNumber {
    pub fn get_neighbours(&self) -> HashSet<Point> {
        self.start.get_neighbours_to_other(&self.end)
    }

    pub fn is_adjacent(&self, point: &Point) -> bool {
        point.get_neighbours().iter().any(|n| {
            n.x >= self.start.x && n.x <= self.end.x && n.y >= self.start.y && n.y <= self.end.y
        })
    }
}

pub struct Schematic {
    pub numbers: Vec<SchematicFullNumber>,
    pub symbols: HashSet<Point>,
    pub gear_symbols: HashSet<Point>,
}

impl Schematic {
    pub fn new(input: Vec<Vec<SchematicValue>>) -> Schematic {
        let mut current_number = 0;
        let mut number_len = 0;
        let mut symbols = HashSet::new();
        let mut gear_symbols = HashSet::new();
        let mut numbers = Vec::new();
        for y in 0..input.len() {
            for x in 0..input[y].len() {
                if let SchematicValue::Digit(digit) = input[y][x] {
                    current_number *= 10;
                    current_number += digit;
                    number_len += 1;
                } else {
                    if number_len > 0 {
                        numbers.push(SchematicFullNumber {
                            number: current_number,
                            start: Point {
                                x: x - number_len,
                                y,
                            },
                            end: Point { x: x - 1, y },
                        });
                        current_number = 0;
                        number_len = 0;
                    }
                    if SchematicValue::Symbol == input[y][x] {
                        symbols.insert(Point { x, y });
                    } else if SchematicValue::GearSymbol == input[y][x] {
                        symbols.insert(Point { x, y });
                        gear_symbols.insert(Point { x, y });
                    }
                }
            }
            if number_len > 0 {
                numbers.push(SchematicFullNumber {
                    number: current_number,
                    start: Point {
                        x: input[y].len() - number_len,
                        y,
                    },
                    end: Point {
                        x: input[y].len() - 1,
                        y,
                    },
                });
                current_number = 0;
                number_len = 0;
            }
        }
        Schematic {
            numbers,
            symbols,
            gear_symbols,
        }
    }

    pub fn get_number_neighbours_to_point(&self, point: &Point) -> Vec<usize> {
        self.numbers
            .iter()
            .filter(|number| number.is_adjacent(&point))
            .map(|number| number.number)
            .collect()
    }
}
