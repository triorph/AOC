extern crate peg;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Debug)]
struct Path {
    last: String,
    small_caves_visited: HashSet<String>,
    small_caves_limit_hit: bool,
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

impl Path {
    fn last(self: &Path) -> &String {
        &self.last
    }

    fn is_finished(self: &Path) -> bool {
        &self.last[..] == "end"
    }

    fn can_visit_node(self: &Path, next_node: &str) -> bool {
        Path::node_is_not_start(next_node)
            && (Path::node_is_major(next_node)
                || !self.has_small_cave_been_visited(next_node)
                || !self.no_small_caves_visited_twice())
    }

    fn has_small_cave_been_visited(self: &Path, small_cave: &str) -> bool {
        self.small_caves_visited.contains(&small_cave.to_string())
    }

    fn no_small_caves_visited_twice(self: &Path) -> bool {
        self.small_caves_limit_hit
    }

    fn insert(self: &mut Path, node: String) {
        if !self.small_caves_limit_hit
            && !Path::node_is_major(&node)
            && self.has_small_cave_been_visited(&node)
        {
            self.small_caves_limit_hit = true;
        }
        if !Path::node_is_major(&node) {
            self.small_caves_visited.insert(node.clone());
        }
        self.last = node;
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
        path: &'a Path,
    ) -> Box<dyn Iterator<Item = &'a String> + 'a> {
        Box::new(
            self.0
                .get(path.last())
                .unwrap()
                .iter()
                .filter(|node| path.can_visit_node(node)),
        )
    }

    fn build_paths_to_goal_part_a(self: &Day12Setup) -> usize {
        let mut path = Path {
            last: "start".to_string(),
            small_caves_visited: HashSet::new(),
            small_caves_limit_hit: true,
        };
        path.insert("start".to_string());
        self.recursively_find_count(path)
    }

    fn recursively_find_count(self: &Day12Setup, path: Path) -> usize {
        if path.is_finished() {
            1
        } else {
            self.get_available_next_nodes(&path)
                .map(|neighbour| {
                    let mut path = path.clone();
                    path.insert(neighbour.clone());
                    self.recursively_find_count(path)
                })
                .sum()
        }
    }

    fn build_paths_to_goal_part_b(self: &Day12Setup) -> usize {
        let mut path = Path {
            last: "start".to_string(),
            small_caves_visited: HashSet::new(),
            small_caves_limit_hit: false,
        };
        path.insert("start".to_string());
        self.recursively_find_count(path)
    }

    /// Calculate the part a response
    pub fn calculate_day_a(self: &Day12Setup) -> usize {
        self.build_paths_to_goal_part_a()
    }

    /// Calculate the part b response
    pub fn calculate_day_b(self: &Day12Setup) -> usize {
        self.build_paths_to_goal_part_b()
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
