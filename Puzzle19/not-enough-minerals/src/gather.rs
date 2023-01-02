use super::blueprint::{Blueprint, MineralCost, Robot};
use std::collections::BinaryHeap;

const MAX_TIME: usize = 24;
#[derive(Debug, PartialEq, Eq, Clone)]
struct Resources<'a> {
    opened_geodes: usize,
    obsidian: usize,
    clay: usize,
    ores: usize,
    blueprint: &'a Blueprint,
}
#[derive(Debug, PartialEq, Eq, Clone)]
struct State<'a> {
    minute: usize,
    resources: Resources<'a>,
    geode_robots: usize,
    obsidian_robots: usize,
    clay_robots: usize,
    ore_robots: usize,
    state_series: Vec<State<'a>>,
}

pub fn get_best_option_for_blueprint(blueprint: &Blueprint) -> usize {
    // We keep record of how close we're to getting a new geode open
    let mut max_opened_geodes: Vec<Resources> = (0..=MAX_TIME)
        .map(|_| Resources {
            opened_geodes: usize::MIN,
            obsidian: usize::MIN,
            clay: usize::MIN,
            ores: usize::MIN,
            blueprint: &blueprint,
        })
        .collect();
    // We'll use a heap to get the best state
    let mut heap = BinaryHeap::new();
    // We define the initial state
    let initial_state = State {
        minute: 0,
        resources: Resources {
            opened_geodes: 0,
            obsidian: 0,
            clay: 0,
            ores: 0,
            blueprint: &blueprint,
        },
        geode_robots: 0,
        obsidian_robots: 0,
        clay_robots: 0,
        ore_robots: 1,
        state_series: vec![],
    };
    heap.push(initial_state);
    while let Some(State {
        minute,
        resources,
        geode_robots,
        obsidian_robots,
        clay_robots,
        ore_robots,
        state_series,
    }) = heap.pop()
    {
        if minute >= MAX_TIME {
            println!("{:?}", state_series);
            return resources.opened_geodes;
        }
        // We need to handle each option, that is, insert in the heap all options that can be done with the current resources.
        // Of course, we only insert them if they are better that any other we have seen before
        if let Some(max_opened_for_time) = max_opened_geodes.get(minute + 1) {
            // Geode robot
            let blueprint_geode_robot = blueprint.geode_robot();
            if let Some(new_state) = get_new_state(
                &resources,
                blueprint_geode_robot,
                minute,
                geode_robots,
                obsidian_robots,
                clay_robots,
                ore_robots,
                &blueprint,
            ) {
                if max_opened_for_time > &new_state.resources {
                    continue;
                }
                let mut new_series = state_series.clone();
                new_series.push(State {
                    minute: new_state.minute,
                    resources: new_state.resources.clone(),
                    geode_robots: new_state.geode_robots + 1,
                    obsidian_robots: new_state.obsidian_robots,
                    clay_robots: new_state.clay_robots,
                    ore_robots: new_state.ore_robots,
                    state_series: vec![],
                });
                heap.push(State {
                    minute: new_state.minute,
                    resources: new_state.resources,
                    geode_robots: new_state.geode_robots + 1,
                    obsidian_robots: new_state.obsidian_robots,
                    clay_robots: new_state.clay_robots,
                    ore_robots: new_state.ore_robots,
                    state_series: new_series,
                })
            }
            // Obsidian robot
            let blueprint_obsidian_robot = blueprint.obsidian_robot();
            if let Some(new_state) = get_new_state(
                &resources,
                blueprint_obsidian_robot,
                minute,
                geode_robots,
                obsidian_robots,
                clay_robots,
                ore_robots,
                &blueprint,
            ) {
                if max_opened_for_time > &new_state.resources {
                    continue;
                }
                let mut new_series = state_series.clone();
                new_series.push(State {
                    minute: new_state.minute,
                    resources: new_state.resources.clone(),
                    geode_robots: new_state.geode_robots,
                    obsidian_robots: new_state.obsidian_robots + 1,
                    clay_robots: new_state.clay_robots,
                    ore_robots: new_state.ore_robots,
                    state_series: vec![],
                });
                heap.push(State {
                    minute: new_state.minute,
                    resources: new_state.resources,
                    geode_robots: new_state.geode_robots,
                    obsidian_robots: new_state.obsidian_robots + 1,
                    clay_robots: new_state.clay_robots,
                    ore_robots: new_state.ore_robots,
                    state_series: new_series,
                })
            }
            // Clay robot
            let blueprint_clay_robot = blueprint.clay_robot();
            if let Some(new_state) = get_new_state(
                &resources,
                blueprint_clay_robot,
                minute,
                geode_robots,
                obsidian_robots,
                clay_robots,
                ore_robots,
                &blueprint,
            ) {
                if max_opened_for_time > &new_state.resources {
                    continue;
                }
                let mut new_series = state_series.clone();
                new_series.push(State {
                    minute: new_state.minute,
                    resources: new_state.resources.clone(),
                    geode_robots: new_state.geode_robots,
                    obsidian_robots: new_state.obsidian_robots,
                    clay_robots: new_state.clay_robots + 1,
                    ore_robots: new_state.ore_robots,
                    state_series: vec![],
                });
                heap.push(State {
                    minute: new_state.minute,
                    resources: new_state.resources,
                    geode_robots: new_state.geode_robots,
                    obsidian_robots: new_state.obsidian_robots,
                    clay_robots: new_state.clay_robots + 1,
                    ore_robots: new_state.ore_robots,
                    state_series: new_series,
                })
            }
            // Ore robot
            let blueprint_ore_robot = blueprint.ore_robot();
            if let Some(new_state) = get_new_state(
                &resources,
                blueprint_ore_robot,
                minute,
                geode_robots,
                obsidian_robots,
                clay_robots,
                ore_robots,
                &blueprint,
            ) {
                if max_opened_for_time > &new_state.resources {
                    continue;
                }
                let mut new_series = state_series.clone();
                new_series.push(State {
                    minute: new_state.minute,
                    resources: new_state.resources.clone(),
                    geode_robots: new_state.geode_robots,
                    obsidian_robots: new_state.obsidian_robots,
                    clay_robots: new_state.clay_robots,
                    ore_robots: new_state.ore_robots + 1,
                    state_series: vec![],
                });
                heap.push(State {
                    minute: new_state.minute,
                    resources: new_state.resources.clone(),
                    geode_robots: new_state.geode_robots,
                    obsidian_robots: new_state.obsidian_robots,
                    clay_robots: new_state.clay_robots,
                    ore_robots: new_state.ore_robots + 1,
                    state_series: new_series,
                })
            }

            let new_resources = Resources {
                opened_geodes: resources.opened_geodes + geode_robots,
                obsidian: resources.obsidian + obsidian_robots,
                clay: resources.clay + clay_robots,
                ores: resources.ores + ore_robots,
                blueprint: &blueprint,
            };
            if max_opened_for_time > &new_resources {
                continue;
            }
            let mut new_series = state_series.clone();
            new_series.push(State {
                minute: minute + 1,
                resources: new_resources.clone(),
                geode_robots: geode_robots,
                obsidian_robots: obsidian_robots,
                clay_robots: clay_robots,
                ore_robots: ore_robots,
                state_series: vec![],
            });
            // The simple option of not spending any resources
            heap.push(State {
                minute: minute + 1,
                resources: new_resources.clone(),
                geode_robots: geode_robots,
                obsidian_robots: obsidian_robots,
                clay_robots: clay_robots,
                ore_robots: ore_robots,
                state_series: new_series,
            });
            max_opened_geodes[minute + 1] = new_resources;
        }
    }
    0
}

fn get_new_state<'a>(
    resources: &Resources,
    robot: &Robot,
    minute: usize,
    geode_robots: usize,
    obsidian_robots: usize,
    clay_robots: usize,
    ore_robots: usize,
    blueprint: &'a Blueprint,
) -> Option<State<'a>> {
    let mut has_resources = true;
    let mut new_ore = resources.ores;
    let mut new_clay = resources.clay;
    let mut new_obsidian = resources.obsidian;
    for cost in robot.costs() {
        match cost {
            MineralCost::Ore(qty) => {
                if &resources.ores < qty {
                    has_resources = false;
                } else {
                    new_ore -= qty;
                }
            }
            MineralCost::Clay(qty) => {
                if &resources.clay < qty {
                    has_resources = false;
                } else {
                    new_clay -= qty;
                }
            }
            MineralCost::Obsidian(qty) => {
                if &resources.obsidian < qty {
                    has_resources = false;
                } else {
                    new_obsidian -= qty;
                }
            }
        }
    }
    if has_resources {
        return Some(State {
            minute: minute + 1,
            resources: Resources {
                opened_geodes: resources.opened_geodes + geode_robots,
                obsidian: new_obsidian + obsidian_robots,
                clay: new_clay + clay_robots,
                ores: new_ore + ore_robots,
                blueprint: &blueprint,
            },
            geode_robots: geode_robots,
            obsidian_robots: obsidian_robots,
            clay_robots: clay_robots,
            ore_robots: ore_robots,
            state_series: vec![],
        });
    }
    None
}
impl<'a> Resources<'a> {
    fn distance_to_geode(&self) -> usize {
        let costs = self.blueprint.geode_robot().costs();
        self.get_distance_to_costs(costs)
    }

    fn get_distance_to_costs(&self, costs: &Vec<MineralCost>) -> usize {
        let mut distance = 0;
        // we punish having too much!
        for cost in costs {
            match cost {
                MineralCost::Ore(qty) => {
                    distance += (*qty as i32 - self.ores as i32).abs();
                }
                MineralCost::Clay(qty) => {
                    distance += (*qty as i32 - self.clay as i32).abs();
                }
                MineralCost::Obsidian(qty) => {
                    distance += (*qty as i32 - self.obsidian as i32).abs();
                }
            }
        }
        distance as usize
    }
    fn distance_to_obsidian(&self) -> usize {
        let costs = self.blueprint.obsidian_robot().costs();
        self.get_distance_to_costs(costs)
    }
    fn distance_to_clay(&self) -> usize {
        let costs = self.blueprint.clay_robot().costs();
        self.get_distance_to_costs(costs)
    }
}
impl<'a> Ord for Resources<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .distance_to_geode()
            .cmp(&self.distance_to_geode())
            .then_with(|| {
                other
                    .distance_to_obsidian()
                    .cmp(&self.distance_to_obsidian())
            })
            .then_with(|| other.distance_to_clay().cmp(&self.distance_to_clay()))
            .then_with(|| self.opened_geodes.cmp(&other.opened_geodes))
            .then_with(|| self.obsidian.cmp(&other.obsidian))
            .then_with(|| self.clay.cmp(&other.clay))
            .then_with(|| self.ores.cmp(&other.ores))
    }
}
impl<'a> PartialOrd for Resources<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}
impl<'a> Ord for State<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .minute
            .cmp(&self.minute)
            .then_with(|| {
                self.resources
                    .opened_geodes
                    .cmp(&other.resources.opened_geodes)
            })
            .then_with(|| self.resources.cmp(&other.resources))
            .then_with(|| self.geode_robots.cmp(&other.geode_robots))
            .then_with(|| (self.obsidian_robots).cmp(&(self.obsidian_robots)))
            .then_with(|| (self.clay_robots).cmp(&(other.clay_robots)))
            .then_with(|| (self.ore_robots).cmp(&(other.ore_robots)))
    }
}
impl<'a> PartialOrd for State<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_best_blueprint_path() {
        let blueprint = Blueprint::new(
            Robot::new(vec![MineralCost::Ore(4)]),
            Robot::new(vec![MineralCost::Ore(2)]),
            Robot::new(vec![MineralCost::Ore(3), MineralCost::Clay(14)]),
            Robot::new(vec![MineralCost::Ore(2), MineralCost::Obsidian(7)]),
        );
        let opened_geodes = get_best_option_for_blueprint(&blueprint);
        assert_eq!(9, opened_geodes);
    }
}
