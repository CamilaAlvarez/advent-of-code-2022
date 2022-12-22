use super::valve::Valve;
use regex::Regex;
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};

pub fn parse_file_to_valves(filecontent: String) -> HashMap<String, Rc<RefCell<Valve>>> {
    let mut valves: HashMap<String, Rc<RefCell<Valve>>> = HashMap::new();
    let mut neighbors_map = HashMap::new();

    let valve_re =
        Regex::new(r"Valve ([A-Za-z]{2}) has flow rate=([0-9]+); tunnels? leads? to valves? (.*)")
            .unwrap();

    for cap in valve_re.captures_iter(&filecontent) {
        let valve_name = &cap[1].to_string();
        let valve_flow = &cap[2].parse::<i32>().unwrap();
        let neighbors_string = &cap[3].to_string();
        let neighbors = neighbors_string
            .split(",")
            .map(|name| name.trim().to_string())
            .collect::<Vec<_>>();
        neighbors_map.insert(valve_name.clone(), neighbors);
        let valve = Valve::new(valve_name.clone(), *valve_flow);
        valves.insert(valve_name.clone(), valve);
    }

    for (name, valve) in valves.iter() {
        if let Some(neighbors_names) = neighbors_map.get(name) {
            for neighbor in neighbors_names.iter() {
                if let Some(neighbor_valve) = valves.get(neighbor) {
                    valve.borrow_mut().add_neighbor(neighbor_valve);
                }
            }
        }
    }

    valves
}
