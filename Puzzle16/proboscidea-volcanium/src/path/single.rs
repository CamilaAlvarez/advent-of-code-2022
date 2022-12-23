use super::state::{PositionAtTime, PositionAtValveKey, State, ValueAtTime};
use crate::valve::Valve;
use std::{
    cell::RefCell,
    collections::{BinaryHeap, HashMap},
    rc::Rc,
};

pub fn get_best_path(
    start: &Rc<RefCell<Valve>>,
    valves: &Vec<Rc<RefCell<Valve>>>,
    max_time: i32,
) -> Option<i32> {
    // Binary heaps by default are max-heaps
    let mut heap = BinaryHeap::new();
    let mut most_value_per_minute_at = HashMap::new();
    for valve in valves.iter() {
        let borrowed_valve = valve.borrow();
        let key = PositionAtValveKey::new_single_key(borrowed_valve.name());
        most_value_per_minute_at.insert(key, i32::MIN);
    }
    let borrowed_start = start.borrow();
    //most_value_per_minute_at.insert(borrowed_start.name(), 0);
    heap.push(State {
        time: 0,
        total_value: 0,
        value_per_minute: ValueAtTime::Single(0),
        position: PositionAtTime::Single(Rc::clone(start)),
        opened_valves: vec![],
        steps: vec![borrowed_start.name()],
    });
    // All performed operations must be included in the heap, we just try to maximize the score obtained by increasing the timer 30 times
    while let Some(State {
        time,
        total_value,
        value_per_minute,
        position,
        opened_valves,
        steps,
    }) = heap.pop()
    {
        if time >= max_time {
            return Some(total_value);
        }
        if let PositionAtTime::Single(position) = position {
            let position_ref = position.borrow();
            // we can open the valve and consume an extra minute
            if !opened_valves.contains(&position_ref.name()) && position_ref.can_be_opened() {
                let mut new_steps = steps.clone();
                new_steps.push(position_ref.name());
                if let Some(value_per_minute_at) = most_value_per_minute_at
                    .get(&PositionAtValveKey::new_single_key(position_ref.name()))
                {
                    if let ValueAtTime::Single(value_per_minute) = value_per_minute {
                        if *value_per_minute_at <= total_value + value_per_minute {
                            let mut opened_valves = opened_valves.clone();
                            opened_valves.push(position_ref.name());
                            heap.push(State {
                                time: time + 1,
                                total_value: total_value + value_per_minute,
                                value_per_minute: ValueAtTime::Single(
                                    value_per_minute + position_ref.flow_rate(),
                                ),
                                position: PositionAtTime::Single(Rc::clone(&position)),
                                opened_valves,
                                steps: new_steps,
                            });
                        }
                    }
                }
            }
            // Or we can move to the best neighbor we place both alternatives
            for neighbor in position_ref.neighbors().iter() {
                if let Some(neighbor) = neighbor.upgrade() {
                    let neighbor_ref = neighbor.borrow();
                    if let Some(value_per_minute_at) = most_value_per_minute_at
                        .get(&PositionAtValveKey::new_single_key(neighbor_ref.name()))
                    {
                        let mut new_steps = steps.clone();
                        new_steps.push(neighbor_ref.name());
                        if let ValueAtTime::Single(value_per_minute) = value_per_minute {
                            if *value_per_minute_at <= value_per_minute {
                                heap.push(State {
                                    time: time + 1,
                                    total_value: total_value + value_per_minute,
                                    value_per_minute: ValueAtTime::Single(value_per_minute),
                                    position: PositionAtTime::Single(Rc::clone(&neighbor)),
                                    opened_valves: opened_valves.clone(),
                                    steps: new_steps,
                                });
                                most_value_per_minute_at.insert(
                                    PositionAtValveKey::new_single_key(neighbor_ref.name()),
                                    value_per_minute,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_path_value() {
        const MAX_TIME: i32 = 30;
        const EXPECTED_SCORE: i32 = 1651;
        let valves = create_valves();
        let best_path_score = get_best_path(&valves[0], &valves, MAX_TIME);
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
