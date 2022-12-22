use super::valve::Valve;
use std::{cell::RefCell, collections::BinaryHeap, rc::Rc};

#[derive(PartialEq, Eq)]
struct State {
    time: i32,
    total_value: i32,
    value_per_minute: i32,
    position: Rc<RefCell<Valve>>,
    opened_valves: Vec<String>,
    previous_valve_name: Option<String>,
}

pub fn get_best_path(start: &Rc<RefCell<Valve>>, max_time: i32) -> Option<i32> {
    // Binary heaps by default are max-heaps
    let mut heap = BinaryHeap::new();
    heap.push(State {
        time: 0,
        total_value: 0,
        value_per_minute: 0,
        position: Rc::clone(start),
        opened_valves: vec![],
        previous_valve_name: None,
    });
    // All performed operations must be included in the heap, we just try to maximize the score obtained by increasing the timer 30 times
    while let Some(State {
        time,
        total_value,
        value_per_minute,
        position,
        opened_valves,
        previous_valve_name,
    }) = heap.pop()
    {
        if time >= max_time {
            return Some(total_value);
        } else {
            let position_ref = position.borrow();
            // we can open the valve and consume an extra minute
            if !opened_valves.contains(&position_ref.name()) && position_ref.can_be_opened() {
                let mut check_opened_valves = opened_valves.clone();
                if let Some(previous_valve) = &previous_valve_name {
                    check_opened_valves.push(previous_valve.clone());
                }
                let neighbors_available_flow =
                    position_ref.neighbors_remaining_flow(&check_opened_valves);
                if neighbors_available_flow < position_ref.flow_rate() {
                    let mut opened_valves = opened_valves.clone();
                    opened_valves.push(position_ref.name());
                    heap.push(State {
                        time: time + 1,
                        total_value: total_value + value_per_minute,
                        value_per_minute: value_per_minute + position_ref.flow_rate(),
                        position: Rc::clone(&position),
                        opened_valves,
                        previous_valve_name: previous_valve_name.clone(),
                    });
                }
            }
            // Or we can move to the best neighbor
            // we place both alternatives, by adding them to the heap we will end up extracting the one that maximizes the pressure
            let mut neighbors_with_max_flow = vec![];
            let mut current_max_flow = i32::MIN;
            for neighbor in position_ref.neighbors().iter() {
                if let Some(neighbor) = neighbor.upgrade() {
                    let neighbor_ref = neighbor.borrow();
                    // we try to avoid meaningless cycles unless there is no other way
                    if let Some(previous) = &previous_valve_name {
                        if *previous == neighbor_ref.name() && position_ref.neighbors().len() > 1 {
                            continue;
                        }
                    }
                    let total_flow = neighbor_ref.neighbors_remaining_flow(&opened_valves)
                        + if !opened_valves.contains(&neighbor_ref.name()) {
                            neighbor_ref.flow_rate()
                        } else {
                            0
                        };
                    // We continue through the neighbor with the best flow
                    if current_max_flow <= total_flow {
                        neighbors_with_max_flow.push(Rc::clone(&neighbor));
                        current_max_flow = total_flow;
                    }
                }
            }
            for neighbor in neighbors_with_max_flow.iter() {
                heap.push(State {
                    time: time + 1,
                    total_value: total_value + value_per_minute,
                    value_per_minute: value_per_minute,
                    position: Rc::clone(&neighbor),
                    opened_valves: opened_valves.clone(),
                    previous_valve_name: Some(position_ref.name()),
                });
            }
        }
    }
    None
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return other
            .time
            .cmp(&self.time)
            .then_with(|| self.total_value.cmp(&other.total_value))
            .then_with(|| self.value_per_minute.cmp(&other.value_per_minute));
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_path_value() {
        const MAX_TIME: i32 = 30;
        const EXPECTED_SCORE: i32 = 1651;
        let valves = create_valves();
        let best_path_score = get_best_path(&valves[0], MAX_TIME);
        assert!(best_path_score.is_some(), "Obtained None value!");
        if let Some(score) = best_path_score {
            assert_eq!(score, EXPECTED_SCORE);
        }
    }
    fn create_valves() -> Vec<Rc<RefCell<Valve>>> {
        let valve_aa = Valve::new("AA".to_string(), 0);
        let valve_bb = Valve::new("BB".to_string(), 13);
        let valve_cc = Valve::new("CC".to_string(), 2);
        let valve_dd = Valve::new("DD".to_string(), 20);
        let valve_ee = Valve::new("EE".to_string(), 3);
        let valve_ff = Valve::new("FF".to_string(), 0);
        let valve_gg = Valve::new("GG".to_string(), 0);
        let valve_hh = Valve::new("HH".to_string(), 22);
        let valve_ii = Valve::new("II".to_string(), 0);
        let valve_jj = Valve::new("JJ".to_string(), 21);

        // Add AA neighbors
        {
            let mut borrowed_aa = valve_aa.borrow_mut();
            borrowed_aa.add_neighbor(&valve_dd);
            borrowed_aa.add_neighbor(&valve_ii);
            borrowed_aa.add_neighbor(&valve_bb);
        }

        // Add BB neighbors
        {
            let mut borrowed_bb = valve_bb.borrow_mut();
            borrowed_bb.add_neighbor(&valve_cc);
            borrowed_bb.add_neighbor(&valve_aa);
        }

        // Add CC neighbors
        {
            let mut borrowed_cc = valve_cc.borrow_mut();
            borrowed_cc.add_neighbor(&valve_dd);
            borrowed_cc.add_neighbor(&valve_bb);
        }

        // Add DD neighbors
        {
            let mut borrowed_dd = valve_dd.borrow_mut();
            borrowed_dd.add_neighbor(&valve_cc);
            borrowed_dd.add_neighbor(&valve_aa);
            borrowed_dd.add_neighbor(&valve_ee);
        }

        // Add EE neighbors
        {
            let mut borrowed_ee = valve_ee.borrow_mut();
            borrowed_ee.add_neighbor(&valve_ff);
            borrowed_ee.add_neighbor(&valve_dd);
        }

        // Add FF neighbors
        {
            let mut borrowed_ff = valve_ff.borrow_mut();
            borrowed_ff.add_neighbor(&valve_ee);
            borrowed_ff.add_neighbor(&valve_gg);
        }
        // Add GG neighbors
        {
            let mut borrowed_gg = valve_gg.borrow_mut();
            borrowed_gg.add_neighbor(&valve_ff);
            borrowed_gg.add_neighbor(&valve_hh);
        }

        // Add HH neighbors
        {
            let mut borrowed_hh = valve_hh.borrow_mut();
            borrowed_hh.add_neighbor(&valve_gg);
        }

        // Add II neighbors
        {
            let mut borrowed_ii = valve_ii.borrow_mut();
            borrowed_ii.add_neighbor(&valve_aa);
            borrowed_ii.add_neighbor(&valve_jj);
        }

        // Add JJ neighbors
        {
            let mut borrowed_jj = valve_jj.borrow_mut();
            borrowed_jj.add_neighbor(&valve_ii);
        }

        vec![
            Rc::clone(&valve_aa),
            Rc::clone(&valve_bb),
            Rc::clone(&valve_cc),
            Rc::clone(&valve_dd),
            Rc::clone(&valve_ee),
            Rc::clone(&valve_ff),
            Rc::clone(&valve_gg),
            Rc::clone(&valve_hh),
            Rc::clone(&valve_ii),
            Rc::clone(&valve_jj),
        ]
    }
}
