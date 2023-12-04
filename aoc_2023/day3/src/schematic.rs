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
}

#[derive(Debug, Clone)]
pub struct SchematicFullNumber {
    pub number: usize,
    neighbours: HashSet<Point>,
}

impl SchematicFullNumber {
    fn new(number: usize, start: Point, end: Point) -> SchematicFullNumber {
        SchematicFullNumber {
            number,
            neighbours: start.get_neighbours_to_other(&end),
        }
    }
    pub fn get_neighbours(&self) -> &HashSet<Point> {
        &self.neighbours
    }

    pub fn is_adjacent(&self, point: &Point) -> bool {
        self.neighbours.contains(point)
    }
}

pub struct Schematic {
    pub numbers: Vec<SchematicFullNumber>,
    pub symbols: HashSet<Point>,
    pub gear_symbols: HashSet<Point>,
}

struct SchematicBuilder {
    current_number: usize,
    number_len: usize,
    symbols: HashSet<Point>,
    gear_symbols: HashSet<Point>,
    numbers: Vec<SchematicFullNumber>,
}

impl SchematicBuilder {
    fn new() -> SchematicBuilder {
        SchematicBuilder {
            current_number: 0,
            number_len: 0,
            symbols: HashSet::new(),
            gear_symbols: HashSet::new(),
            numbers: Vec::new(),
        }
    }

    fn increase_number(&mut self, digit: usize) {
        self.current_number *= 10;
        self.current_number += digit;
        self.number_len += 1;
    }

    fn insert_number_if_exists(&mut self, point: &Point) {
        if self.number_len > 0 {
            self.numbers.push(SchematicFullNumber::new(
                self.current_number,
                Point {
                    x: point.x - self.number_len,
                    y: point.y,
                },
                Point {
                    x: point.x - 1,
                    y: point.y,
                },
            ));
            self.current_number = 0;
            self.number_len = 0;
        }
    }

    fn process(&mut self, point: Point, value: &SchematicValue) {
        if let SchematicValue::Digit(digit) = value {
            self.increase_number(*digit);
        } else {
            self.insert_number_if_exists(&point);
            if &SchematicValue::Symbol == value {
                self.symbols.insert(point.clone());
            } else if &SchematicValue::GearSymbol == value {
                self.symbols.insert(point.clone());
                self.gear_symbols.insert(point.clone());
            }
        }
    }

    fn to_schematic(&self) -> Schematic {
        Schematic {
            numbers: self.numbers.clone(),
            symbols: self.symbols.clone(),
            gear_symbols: self.gear_symbols.clone(),
        }
    }
}

impl Schematic {
    pub fn new(input: Vec<Vec<SchematicValue>>) -> Schematic {
        let mut schematic_builder = SchematicBuilder::new();
        for (y, line) in input.iter().enumerate() {
            for (x, value) in line.iter().enumerate() {
                schematic_builder.process(Point { x, y }, value);
            }
            schematic_builder.insert_number_if_exists(&Point {
                x: input[y].len(),
                y,
            });
        }
        schematic_builder.to_schematic()
    }

    pub fn get_number_neighbours_to_point(&self, point: &Point) -> Vec<usize> {
        self.numbers
            .iter()
            .filter(|number| number.is_adjacent(point))
            .map(|number| number.number)
            .collect()
    }
}
