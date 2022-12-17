use std::collections::{HashMap, VecDeque};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Tunnel {
    pub name: String,
    pub rate: usize,
    pub neighbours: Vec<String>,
    pub opened: bool,
}

impl Tunnel {
    pub fn new(name: String, rate: usize, neighbours: Vec<String>) -> Tunnel {
        Tunnel {
            name,
            rate,
            neighbours,
            opened: false,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Tunnels {
    tunnels: HashMap<String, Tunnel>,
    cached_distances: HashMap<String, HashMap<String, usize>>,
    cached_valuable_tunnels: Vec<String>,
}

impl Tunnels {
    pub fn new(input: Vec<Tunnel>) -> Tunnels {
        let mut tunnels = HashMap::new();
        for tunnel in input.into_iter() {
            tunnels.insert(tunnel.name.clone(), tunnel);
        }
        let mut ret = Tunnels {
            tunnels,
            cached_distances: HashMap::new(),
            cached_valuable_tunnels: Vec::new(),
        };
        ret.cache_distances();
        ret.cache_valuable_tunnels();
        println!(
            "ret.cached_valuable_tunnels
                : {:?}",
            ret.cached_valuable_tunnels
        );
        ret
    }

    fn cache_distances(&mut self) {
        let mut starting = VecDeque::from(vec!["AA".to_string()]);
        let mut processed = Vec::new();
        while let Some(source) = starting.pop_back() {
            processed.push(source.clone());
            let neighbours = self.tunnels.get(&source).unwrap().neighbours.clone();
            for neighbour in neighbours.iter() {
                self.set_cached_distance(source.clone(), neighbour.to_string(), 1);
                for (k, v) in self.cached_distances.get(&source).unwrap().clone().iter() {
                    if k != neighbour {
                        self.set_cached_distance(neighbour.to_string(), k.to_string(), v + 1);
                    }
                }

                if !processed.contains(neighbour) && !starting.contains(neighbour) {
                    starting.push_back(neighbour.clone());
                }
            }
        }
    }

    fn cache_valuable_tunnels(&mut self) {
        self.cached_valuable_tunnels = self
            .tunnels
            .iter()
            .filter(|(_, v)| v.rate > 0)
            .map(|(k, _)| k.clone())
            .collect();
    }

    fn set_cached_distance(&mut self, source: String, dest: String, distance: usize) {
        self.cached_distances
            .entry(source.clone())
            .or_insert_with(HashMap::new)
            .entry(dest.clone())
            .and_modify(|v| {
                *v = (*v as usize).min(distance);
            })
            .or_insert(distance);
        self.cached_distances
            .entry(dest)
            .or_insert_with(HashMap::new)
            .entry(source)
            .and_modify(|v| {
                *v = (*v as usize).min(distance);
            })
            .or_insert(distance);
    }

    pub fn get_unvisited_valves(&self, paths_seen: &[String]) -> Vec<String> {
        self.cached_valuable_tunnels
            .iter()
            .filter(|name| !paths_seen.contains(name))
            .cloned()
            .collect()
    }

    pub fn flow_rate(&self, source: &str) -> usize {
        self.tunnels.get(source).unwrap().rate
    }

    pub fn get_distance_between(&self, source: &str, dest: &str) -> usize {
        *self
            .cached_distances
            .get(source)
            .unwrap()
            .get(dest)
            .unwrap()
    }

    pub fn get_possible_pressure_relief(
        &self,
        source: &str,
        dest: &str,
        distance_left: usize,
    ) -> usize {
        let distance_between = self.get_distance_between(source, dest);
        if distance_between + 1 > distance_left {
            0
        } else {
            let rate = self.flow_rate(dest);
            rate * (distance_left - distance_between - 1)
        }
    }
}
