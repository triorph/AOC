#[derive(Debug, Eq, PartialEq)]
pub struct Blueprint {
    pub costs: [[usize; 3]; 4],
    pub max_reqs: [usize; 3],
    pub index: usize,
}

#[derive(Debug, Eq, PartialEq)]
pub enum RobotType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Eq, PartialEq)]
enum Decision {
    DoNothing,
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl RobotType {
    fn get_as_index(&self) -> usize {
        match self {
            RobotType::Ore => 0,
            RobotType::Clay => 1,
            RobotType::Obsidian => 2,
            RobotType::Geode => 3,
        }
    }

    fn get_as_decision(&self) -> Decision {
        match self {
            RobotType::Ore => Decision::Ore,
            RobotType::Clay => Decision::Clay,
            RobotType::Obsidian => Decision::Obsidian,
            RobotType::Geode => Decision::Geode,
        }
    }
}

impl Blueprint {
    pub fn new(index: usize, robots: &[(RobotType, Vec<(usize, RobotType)>)]) -> Blueprint {
        let mut costs = [[0; 3]; 4];
        for (robot, this_robots_costs) in robots.iter() {
            for (quantity, cost_type) in this_robots_costs.iter() {
                costs[robot.get_as_index()][cost_type.get_as_index()] = *quantity;
            }
        }
        let max_reqs: [usize; 3] = (0..=2)
            .map(|j| (0..=3).map(|i| costs[i][j]).max().unwrap())
            .collect::<Vec<usize>>()
            .try_into()
            .unwrap();
        Blueprint {
            costs,
            index,
            max_reqs,
        }
    }

    fn can_afford(&self, resource_count: &[usize; 4], robot_type: &RobotType) -> bool {
        (0..=2).all(|i| resource_count[i] >= self.costs[robot_type.get_as_index()][i])
    }

    fn decisions_available(
        &self,
        robot_count: &[usize; 4],
        resource_count: &[usize; 4],
        time_left: usize,
    ) -> Vec<Decision> {
        let mut ret = Vec::new();
        if self.can_afford(resource_count, &RobotType::Geode) {
            ret.push(Decision::Geode);
        } else {
            if robot_count[2] < self.max_reqs[2]
                && resource_count[2] < self.max_reqs[2] * time_left
                && self.can_afford(resource_count, &RobotType::Obsidian)
            {
                ret.push(Decision::Obsidian);
            }
            if robot_count[0] < self.max_reqs[0]
                && resource_count[0] < self.max_reqs[0] * time_left
                && self.max_reqs[0] < time_left
                && self.can_afford(resource_count, &RobotType::Ore)
            {
                ret.push(Decision::Ore);
            }
            if robot_count[1] < self.max_reqs[1]
                && resource_count[1] < self.max_reqs[1] * time_left
                && self.can_afford(resource_count, &RobotType::Clay)
            {
                ret.push(Decision::Clay);
            }
        }

        if !ret.contains(&Decision::Geode)
            && !ret.contains(&Decision::Obsidian)
            && (robot_count[0] < self.max_reqs[0] && !ret.contains(&Decision::Ore)
                || robot_count[1] < self.max_reqs[1] && !ret.contains(&Decision::Clay)
                || robot_count[2] < self.max_reqs[2] && robot_count[1] > 0)
        {
            ret.push(Decision::DoNothing);
        }
        ret
    }

    fn make_decision(
        &self,
        robot_count: &[usize; 4],
        resource_count: &[usize; 4],
        decision: &Decision,
    ) -> ([usize; 4], [usize; 4]) {
        let mut robot_count = robot_count.clone();
        let mut resource_count = resource_count.clone();
        match decision {
            Decision::DoNothing => (),
            Decision::Ore => {
                for (i, resource) in self.costs[0].iter().enumerate() {
                    resource_count[i] -= resource;
                }
            }
            Decision::Clay => {
                for (i, resource) in self.costs[1].iter().enumerate() {
                    resource_count[i] -= resource;
                }
            }
            Decision::Obsidian => {
                for (i, resource) in self.costs[2].iter().enumerate() {
                    resource_count[i] -= resource;
                }
            }
            Decision::Geode => {
                for (i, resource) in self.costs[3].iter().enumerate() {
                    resource_count[i] -= resource;
                }
            }
        }
        for i in 0..robot_count.len() {
            resource_count[i] += robot_count[i];
        }
        match decision {
            Decision::DoNothing => (),
            Decision::Ore => {
                robot_count[0] += 1;
            }
            Decision::Clay => {
                robot_count[1] += 1;
            }
            Decision::Obsidian => {
                robot_count[2] += 1;
            }
            Decision::Geode => {
                robot_count[3] += 1;
            }
        }
        (robot_count, resource_count)
    }
    pub fn calculate_quality_level(&self) -> usize {
        let max = self.find_optimal_geode(24, &[1, 0, 0, 0], &[0, 0, 0, 0]);
        max * self.index
    }

    pub fn dayb_most_geodes(&self) -> usize {
        self.find_optimal_geode(32, &[1, 0, 0, 0], &[0, 0, 0, 0])
    }

    fn find_optimal_geode(
        &self,
        time_left: usize,
        robot_count: &[usize; 4],
        resource_count: &[usize; 4],
    ) -> usize {
        let mut best_geodes = 0;
        if time_left == 0 {
            return resource_count[3];
        }
        for decision in self
            .decisions_available(robot_count, resource_count, time_left)
            .iter()
        {
            let (robot_count, resource_count) =
                self.make_decision(robot_count, resource_count, decision);
            let this_route = self.find_optimal_geode(time_left - 1, &robot_count, &resource_count);

            best_geodes = best_geodes.max(this_route);
        }
        best_geodes
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_blueprint_optimal_geode() {
        let blueprint = Blueprint {
            index: 1,
            max_reqs: [4, 14, 7],
            costs: [[4, 0, 0], [2, 0, 0], [3, 14, 0], [2, 0, 7]],
        };
        assert_eq!(
            blueprint.find_optimal_geode(24, &[1, 0, 0, 0], &[0, 0, 0, 0]),
            9
        );
        let blueprint = Blueprint {
            index: 2,
            max_reqs: [3, 8, 12],
            costs: [[2, 0, 0], [3, 0, 0], [3, 8, 0], [3, 0, 12]],
        };
        assert_eq!(
            blueprint.find_optimal_geode(24, &[1, 0, 0, 0], &[0, 0, 0, 0]),
            12
        );
    }

    #[test]
    fn test_blueprint_optimal_geode_dayb() {
        let blueprint = Blueprint {
            index: 1,
            max_reqs: [4, 14, 7],
            costs: [[4, 0, 0], [2, 0, 0], [3, 14, 0], [2, 0, 7]],
        };
        assert_eq!(
            blueprint.find_optimal_geode(32, &[1, 0, 0, 0], &[0, 0, 0, 0]),
            56
        );
        let blueprint = Blueprint {
            index: 2,
            max_reqs: [3, 8, 12],
            costs: [[2, 0, 0], [3, 0, 0], [3, 8, 0], [3, 0, 12]],
        };
        assert_eq!(
            blueprint.find_optimal_geode(32, &[1, 0, 0, 0], &[0, 0, 0, 0]),
            62
        );
    }
}
