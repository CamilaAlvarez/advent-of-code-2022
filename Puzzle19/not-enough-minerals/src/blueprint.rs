#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MineralCost {
    Ore(usize),
    Clay(usize),
    Obsidian(usize),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Robot {
    costs: Vec<MineralCost>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Blueprint {
    ore_robot: Robot,
    clay_robot: Robot,
    obsidian_robot: Robot,
    geode_robot: Robot,
}
impl Robot {
    pub fn new(costs: Vec<MineralCost>) -> Self {
        Self { costs }
    }
    pub fn costs(&self) -> &Vec<MineralCost> {
        &self.costs
    }
}
impl Blueprint {
    pub fn new(
        ore_robot: Robot,
        clay_robot: Robot,
        obsidian_robot: Robot,
        geode_robot: Robot,
    ) -> Self {
        Self {
            ore_robot,
            clay_robot,
            obsidian_robot,
            geode_robot,
        }
    }
    pub fn ore_robot(&self) -> &Robot {
        &self.ore_robot
    }
    pub fn clay_robot(&self) -> &Robot {
        &self.clay_robot
    }
    pub fn obsidian_robot(&self) -> &Robot {
        &self.obsidian_robot
    }
    pub fn geode_robot(&self) -> &Robot {
        &self.geode_robot
    }
}
