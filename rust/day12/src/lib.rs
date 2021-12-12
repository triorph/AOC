extern crate peg;
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct PathExploreState {
    last: String,
    cave_visit_count: HashMap<String, usize>,
    small_caves_limit_hit: bool,
    finish_count: usize,
}

pub struct Day12Setup(HashMap<String, Vec<String>>);

peg::parser! { grammar day12_parser() for str {
    rule lower_node() -> &'input str
        = node:$(['a'..='z']+) {node }
    rule upper_node() -> &'input str
        = node:$(['A'..='Z']+) {node}
    rule number() -> usize
        = n:$(['0'..='9']) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule node() -> &'input str
        = node:(lower_node() / upper_node()) { node }
    rule edge() -> (&'input str, &'input str)
        = left:node() "-" right:node() { (left, right) }
    pub rule parse() -> Vec<(&'input str, &'input str)>
        = edges:edge() ++ "\n" "\n" * {
            edges
        }
}}

impl PathExploreState {
    fn from_start(limit_hit: bool) -> PathExploreState {
        let mut path = PathExploreState {
            last: "start".to_string(),
            cave_visit_count: HashMap::<String, usize>::new(),
            small_caves_limit_hit: limit_hit,
            finish_count: 0,
        };
        path.insert("start");
        path
    }

    fn last(self: &PathExploreState) -> &String {
        &self.last
    }

    fn is_finished(self: &PathExploreState) -> bool {
        &self.last[..] == "end"
    }

    fn can_visit_node(self: &PathExploreState, next_node: &str) -> bool {
        PathExploreState::node_is_not_start(next_node)
            && (PathExploreState::node_is_major(next_node)
                || !self.has_small_cave_been_visited(next_node)
                || !self.no_small_caves_visited_twice())
    }

    fn has_small_cave_been_visited(self: &PathExploreState, small_cave: &str) -> bool {
        matches!(self.cave_visit_count.get(small_cave), Some(i) if i > &0)
    }

    fn no_small_caves_visited_twice(self: &PathExploreState) -> bool {
        self.small_caves_limit_hit
    }

    fn insert(self: &mut PathExploreState, node: &str) -> (String, bool) {
        let old_last = self.last.clone();
        let old_limit = self.small_caves_limit_hit;

        if !self.small_caves_limit_hit
            && !PathExploreState::node_is_major(node)
            && self.has_small_cave_been_visited(node)
        {
            self.small_caves_limit_hit = true;
        }
        if !PathExploreState::node_is_major(node) {
            *self.cave_visit_count.entry(node.to_string()).or_insert(0) += 1;
        }
        self.last = node.to_string();
        (old_last, old_limit)
    }

    fn reduce_count(self: &mut PathExploreState, node: &str) {
        if !PathExploreState::node_is_major(node) {
            *self.cave_visit_count.entry(node.to_string()).or_insert(1) -= 1;
        }
    }

    fn node_is_not_start(node: &str) -> bool {
        node != "start"
    }

    fn node_is_major(node: &str) -> bool {
        if let Some(c) = node.chars().next() {
            ('A'..'Z').contains(&c)
        } else {
            false
        }
    }
}

impl Day12Setup {
    /// Generates a new Day12Setup object to be calculated upon
    ///
    /// Inputs: the full string contents of the input data set.
    ///
    /// Returns: A new Day12Setup object, with methods `calculate_day_a` and `calculate_day_b`
    /// available
    pub fn new(input_str: &str) -> Day12Setup {
        let edges = day12_parser::parse(input_str).unwrap();
        let mut hashmap = HashMap::<String, Vec<String>>::new();
        for (left, right) in edges.into_iter() {
            let left_string = String::from(left);
            let right_string = String::from(right);
            let left_entry = hashmap.entry(left_string.clone()).or_insert_with(Vec::new);
            (*left_entry).push(right_string.clone());
            let right_entry = hashmap.entry(right_string).or_insert_with(Vec::new);
            (*right_entry).push(left_string);
        }
        Day12Setup(hashmap)
    }

    fn get_available_next_nodes<'a>(
        self: &'a Day12Setup,
        path: &'a PathExploreState,
    ) -> Box<dyn Iterator<Item = &'a String> + 'a> {
        Box::new(
            self.0
                .get(path.last())
                .unwrap()
                .iter()
                .filter(|node| path.can_visit_node(node)),
        )
    }

    fn recursively_explore_path(self: &Day12Setup, mut path: PathExploreState) -> PathExploreState {
        if path.is_finished() {
            path.finish_count += 1;
        } else {
            let neighbours: Vec<String> = self.get_available_next_nodes(&path).cloned().collect();
            for neighbour in neighbours.iter() {
                let (old_last, old_limit) = path.insert(&neighbour[..]);
                path = self.recursively_explore_path(path);
                path.reduce_count(&neighbour[..]);
                path.last = old_last;
                path.small_caves_limit_hit = old_limit;
            }
        }
        path
    }

    /// Calculate the part a response
    pub fn calculate_day_a(self: &Day12Setup) -> usize {
        let mut path = PathExploreState::from_start(true);
        path = self.recursively_explore_path(path);
        path.finish_count
    }

    /// Calculate the part b response
    pub fn calculate_day_b(self: &Day12Setup) -> usize {
        let mut path = PathExploreState::from_start(false);
        path = self.recursively_explore_path(path);
        path.finish_count
    }
}

#[cfg(test)]
mod test {
    use crate::Day12Setup;

    #[test]
    fn test_parse() {
        let _day12_setup = Day12Setup::new(include_str!("../test_data.txt"));
    }

    #[test]
    fn test_day_a() {
        let day12_setup = Day12Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day12_setup.calculate_day_a(), 10);
        let day12_setup = Day12Setup::new(include_str!("../test_data_2.txt"));
        assert_eq!(day12_setup.calculate_day_a(), 19);
        let day12_setup = Day12Setup::new(include_str!("../test_data_3.txt"));
        assert_eq!(day12_setup.calculate_day_a(), 226);
    }

    #[test]
    fn test_day_b() {
        let day12_setup = Day12Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day12_setup.calculate_day_b(), 36);
        let day12_setup = Day12Setup::new(include_str!("../test_data_2.txt"));
        assert_eq!(day12_setup.calculate_day_b(), 103);
        let day12_setup = Day12Setup::new(include_str!("../test_data_3.txt"));
        assert_eq!(day12_setup.calculate_day_b(), 3509);
    }
}
