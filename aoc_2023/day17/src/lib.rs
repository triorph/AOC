mod crucible;
mod parser;
use std::collections::{BinaryHeap, HashMap};

use crate::crucible::CrucibleLocation;
use crate::parser::parse_data;
use aoc_helpers::{point2d::Point2D, read_input_file, AOCCalculator, AOCFileOrParseError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day17 {
    lava_pool: Vec<Vec<usize>>,
}

impl AOCCalculator for Day17 {
    fn new(filename: &str) -> Result<Day17, AOCFileOrParseError> {
        Ok(Day17 {
            lava_pool: parse_data(&read_input_file(filename)?)?,
        })
    }

    fn print_results(&self, name: &str) {
        println!("{}a answer is {:?}", name, self.calculate_day_a());
        println!("{}b answer is {:?}", name, self.calculate_day_b());
    }
}

// we need a single object for the binaryHeap with its custom ordering.
// In this case, we just rely on the distance for ordering and ignore
// everything else.
//
// It could be an okay idea to use manhattan distance from 0,0 as the
// next ordering point
#[derive(Debug, PartialEq, Eq, Clone)]
struct PathState(CrucibleLocation, usize);

impl Ord for PathState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.cmp(&other.1).reverse()
    }
}

impl PartialOrd for PathState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Day17 {
    fn within_bounds(&self, point: &Point2D) -> bool {
        (0..self.lava_pool.len() as isize).contains(&point.y)
            && (0..self.lava_pool[0].len() as isize).contains(&point.x)
    }

    fn value_at_point(&self, point: &Point2D) -> Option<usize> {
        if self.within_bounds(point) {
            Some(self.lava_pool[point.y as usize][point.x as usize])
        } else {
            None
        }
    }

    fn is_distance_valid(
        &self,
        seen: &HashMap<CrucibleLocation, usize>,
        final_best: Option<usize>,
        crucible_location: &CrucibleLocation,
        distance: usize,
    ) -> bool {
        if seen
            .get(crucible_location)
            .map(|prev_distance| prev_distance <= &distance)
            .unwrap_or(false)
        {
            return false;
        }
        if final_best
            .map(|final_best| final_best <= distance)
            .unwrap_or(false)
        {
            return false;
        }
        true
    }

    fn find_shortest_path(&self, start: CrucibleLocation) -> usize {
        // essentially dijkstra's algorithm. The main difference
        // is that instead of assigning each point a "best" value and aborting
        // early if you exceed that point, we assign the combined
        // "point + direction + length along that direction" (conveniently labelled as a
        // crucible_location here).
        //
        // Works with a VecDeque, but is slow. Changing to a BinaryHeap sped up from 20s for the
        // whole process to 0.5 seconds.
        let mut seen: HashMap<CrucibleLocation, usize> = HashMap::new();
        let mut paths = BinaryHeap::new();
        paths.push(PathState(start, 0));
        let mut final_best = None;
        let end = Point2D {
            x: self.lava_pool[0].len() as isize - 1,
            y: self.lava_pool.len() as isize - 1,
        };
        while let Some(PathState(crucible_location, distance)) = paths.pop() {
            if !self.is_distance_valid(&seen, final_best, &crucible_location, distance) {
                continue;
            }
            seen.insert(crucible_location.clone(), distance);
            if crucible_location.is_at_end(&end) {
                final_best = Some(distance);
            }
            for neighbour in crucible_location.get_neighbours().into_iter() {
                if let Some(distance_increase) = self.value_at_point(&neighbour.location) {
                    paths.push(PathState(neighbour, distance + distance_increase));
                }
            }
        }
        final_best.unwrap_or(0)
    }

    fn calculate_day_a(&self) -> usize {
        self.find_shortest_path(CrucibleLocation::get_starting_crucible_location(1, 3))
    }

    fn calculate_day_b(&self) -> usize {
        self.find_shortest_path(CrucibleLocation::get_starting_crucible_location(4, 10))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_day_a() {
        let day17 = Day17::new("data/test_data.txt").unwrap();
        let expected = 102;
        let actual = day17.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b() {
        let day17 = Day17::new("data/test_data.txt").unwrap();
        let expected = 94;
        let actual = day17.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_day_b_example_2() {
        let day17 = Day17::new("data/test_data_b.txt").unwrap();
        let expected = 71;
        let actual = day17.calculate_day_b();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_a() {
        let day17 = Day17::new("data/input_data.txt").unwrap();
        let expected = 861;
        let actual = day17.calculate_day_a();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_input_calculate_day_b() {
        let day17 = Day17::new("data/input_data.txt").unwrap();
        let expected = 1037;
        let actual = day17.calculate_day_b();
        assert_eq!(expected, actual);
    }
}
