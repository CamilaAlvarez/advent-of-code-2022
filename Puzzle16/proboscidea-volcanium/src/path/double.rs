use super::state::{PositionAtTime, PositionAtValveKey, State, ValueAtTime};
use crate::valve::Valve;
use std::{
    cell::RefCell,
    collections::{BinaryHeap, HashMap},
    rc::{Rc, Weak},
};

// In this case each state has two positions
pub fn get_best_path_with_two(
    start: &Rc<RefCell<Valve>>,
    valves: &Vec<Rc<RefCell<Valve>>>,
    max_time: i32,
) -> Option<i32> {
    let openable_valves = valves.iter().fold(0, |acc, valve| {
        let borrowed_valve = valve.borrow();
        if borrowed_valve.can_be_opened() {
            return acc + 1;
        }
        return acc;
    });
    // Binary heaps by default are max-heaps
    let mut heap = BinaryHeap::new();
    let mut most_value_per_minute_at_myself = HashMap::new();
    let mut most_value_per_minute_at_elephant = HashMap::new();
    for valve in valves.iter() {
        let borrowed_valve = valve.borrow();
        let key = PositionAtValveKey::new_single_key(borrowed_valve.name());
        most_value_per_minute_at_myself.insert(key.clone(), 0);
        most_value_per_minute_at_elephant.insert(key, 0);
    }
    let borrowed_start = start.borrow();
    //most_value_per_minute_at.insert(borrowed_start.name(), 0);
    heap.push(State {
        time: 0,
        total_value: 0,
        value_per_minute: ValueAtTime::Double(0, 0),
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
        } else if opened_valves.len() >= openable_valves {
            match value_per_minute {
                ValueAtTime::Single(value_per_minute) => {
                    return Some(total_value + (max_time - time) * value_per_minute)
                }
                ValueAtTime::Double(value_minute_myself, value_minute_elephant) => {
                    return Some(
                        total_value
                            + (max_time - time) * (value_minute_myself + value_minute_elephant),
                    )
                }
            }
        }
        match position {
            // we know this happens the first time
            PositionAtTime::Single(first_position) => {
                let position_ref = first_position.borrow();
                let number_neighbors = position_ref.neighbors().len();
                // it's not relevant to add symmetric pairs (e.g: DD-BB and BB-DD), since there are valves that cannot be opened
                for i in 0..number_neighbors {
                    for j in i + 1..number_neighbors {
                        if let Some(neighbor1) = position_ref.neighbors().get(i) {
                            if let Some(neighbor1) = Weak::upgrade(neighbor1) {
                                if let Some(neighbor2) = position_ref.neighbors().get(j) {
                                    if let Some(neighbor2) = Weak::upgrade(neighbor2) {
                                        let neighbor1_ref = neighbor1.borrow();
                                        let neighbor2_ref = neighbor2.borrow();
                                        let mut new_steps = steps.clone();
                                        new_steps.push(neighbor1_ref.name());
                                        new_steps.push(neighbor2_ref.name());
                                        heap.push(State {
                                            time: time + 1,
                                            total_value: total_value,
                                            value_per_minute: ValueAtTime::Double(0, 0),
                                            position: PositionAtTime::Double(
                                                Rc::clone(&neighbor1),
                                                Rc::clone(&neighbor2),
                                            ),
                                            opened_valves: opened_valves.clone(),
                                            steps: new_steps,
                                        });
                                        let mut new_steps = steps.clone();
                                        new_steps.push(neighbor2_ref.name());
                                        new_steps.push(neighbor1_ref.name());
                                        heap.push(State {
                                            time: time + 1,
                                            total_value: total_value,
                                            value_per_minute: ValueAtTime::Double(0, 0),
                                            position: PositionAtTime::Double(
                                                Rc::clone(&neighbor2),
                                                Rc::clone(&neighbor1),
                                            ),
                                            opened_valves: opened_valves.clone(),
                                            steps: new_steps,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
            PositionAtTime::Double(my_position, elephant_position) => {
                // in this case we each time we need to place 4 different cases in the heap:
                // 1. We both open our repective valve
                // 2. I open a valve and the elephant moves
                // 3. The elephant opens a valve and I move
                // 4. Both the elephant and I move
                let my_position_ref = my_position.borrow();
                let elephant_position_ref = elephant_position.borrow();

                // 1. We both open our repective valve
                open_both_valves(
                    &opened_valves,
                    &my_position_ref,
                    &elephant_position_ref,
                    &mut most_value_per_minute_at_myself,
                    &mut most_value_per_minute_at_elephant,
                    &value_per_minute,
                    &steps,
                    &mut heap,
                    time,
                    total_value,
                    &my_position,
                    &elephant_position,
                );

                // 2. I open a valve and the elephant moves
                open_valve_move_elephant(
                    &opened_valves,
                    &my_position_ref,
                    &elephant_position_ref,
                    &mut most_value_per_minute_at_myself,
                    &mut most_value_per_minute_at_elephant,
                    &value_per_minute,
                    &steps,
                    &mut heap,
                    time,
                    total_value,
                    &my_position,
                );

                // 3. The elephant opens a valve and I move
                move_myself_open_valve_elephant(
                    &opened_valves,
                    &elephant_position_ref,
                    &my_position_ref,
                    &mut most_value_per_minute_at_myself,
                    &mut most_value_per_minute_at_elephant,
                    &value_per_minute,
                    &steps,
                    &mut heap,
                    time,
                    total_value,
                    &elephant_position,
                );

                // 4. We both move
                move_myself_and_elephant(
                    &my_position_ref,
                    &elephant_position_ref,
                    &mut most_value_per_minute_at_myself,
                    &mut most_value_per_minute_at_elephant,
                    &value_per_minute,
                    &steps,
                    &mut heap,
                    time,
                    total_value,
                    &opened_valves,
                );
            }
        }
    }
    None
}

fn open_both_valves(
    opened_valves: &Vec<String>,
    my_position_ref: &std::cell::Ref<Valve>,
    elephant_position_ref: &std::cell::Ref<Valve>,
    most_value_per_minute_at_myself: &mut HashMap<PositionAtValveKey, i32>,
    most_value_per_minute_at_elephant: &mut HashMap<PositionAtValveKey, i32>,
    value_per_minute: &ValueAtTime,
    steps: &Vec<String>,
    heap: &mut BinaryHeap<State>,
    time: i32,
    total_value: i32,
    my_position: &Rc<RefCell<Valve>>,
    elephant_position: &Rc<RefCell<Valve>>,
) {
    if !opened_valves.contains(&my_position_ref.name()) && my_position_ref.can_be_opened() {
        let mut opened_valves = opened_valves.clone();
        opened_valves.push(my_position_ref.name());
        if !opened_valves.contains(&elephant_position_ref.name())
            && elephant_position_ref.can_be_opened()
        {
            opened_valves.push(elephant_position_ref.name());
            if let ValueAtTime::Double(value_per_minute_myself, value_per_minute_elephant) =
                value_per_minute
            {
                if let Some(value_per_minute_at_myself) = most_value_per_minute_at_myself
                    .get(&PositionAtValveKey::new_single_key(my_position_ref.name()))
                {
                    if let Some(value_per_minute_at_elephant) = most_value_per_minute_at_elephant
                        .get(&PositionAtValveKey::new_single_key(
                            elephant_position_ref.name(),
                        ))
                    {
                        if *value_per_minute_at_myself <= (*value_per_minute_myself + total_value)
                            && *value_per_minute_at_elephant
                                <= (*value_per_minute_elephant + total_value)
                        {
                            add_step_to_heap(
                                &steps,
                                &my_position_ref,
                                &elephant_position_ref,
                                heap,
                                time,
                                total_value,
                                value_per_minute,
                                &ValueAtTime::Double(
                                    *value_per_minute_myself + my_position_ref.flow_rate(),
                                    *value_per_minute_elephant + elephant_position_ref.flow_rate(),
                                ),
                                &my_position,
                                &elephant_position,
                                opened_valves,
                            );
                            /*most_value_per_minute_at_myself.insert(
                                PositionAtValveKey::new_single_key(my_position_ref.name()),
                                *value_per_minute_myself + my_position_ref.flow_rate(),
                            );

                            most_value_per_minute_at_elephant.insert(
                                PositionAtValveKey::new_single_key(elephant_position_ref.name()),
                                *value_per_minute_elephant + elephant_position_ref.flow_rate(),
                            );*/
                        }
                    }
                }
            }
        }
    }
}

fn move_myself_open_valve_elephant(
    opened_valves: &Vec<String>,
    elephant_position_ref: &std::cell::Ref<Valve>,
    my_position_ref: &std::cell::Ref<Valve>,
    most_value_per_minute_at_myself: &mut HashMap<PositionAtValveKey, i32>,
    most_value_per_minute_at_elephant: &mut HashMap<PositionAtValveKey, i32>,
    value_per_minute: &ValueAtTime,
    steps: &Vec<String>,
    heap: &mut BinaryHeap<State>,
    time: i32,
    total_value: i32,
    elephant_position: &Rc<RefCell<Valve>>,
) {
    if !opened_valves.contains(&elephant_position_ref.name())
        && elephant_position_ref.can_be_opened()
    {
        if let ValueAtTime::Double(value_per_minute_myself, value_per_minute_elephant) =
            value_per_minute
        {
            let mut should_update = false;
            if let Some(value_per_minute_at_elephant) = most_value_per_minute_at_elephant.get(
                &PositionAtValveKey::new_single_key(elephant_position_ref.name()),
            ) {
                let mut opened_valves = opened_valves.clone();
                opened_valves.push(elephant_position_ref.name());
                for neighbor in my_position_ref.neighbors().iter() {
                    if let Some(neighbor) = neighbor.upgrade() {
                        let neighbor_ref = neighbor.borrow();

                        if let Some(value_per_minute_at_myself) = most_value_per_minute_at_myself
                            .get(&PositionAtValveKey::new_single_key(my_position_ref.name()))
                        {
                            if *value_per_minute_at_myself <= (*value_per_minute_myself)
                                && *value_per_minute_at_elephant
                                    <= (*value_per_minute_elephant + total_value)
                            {
                                add_step_to_heap(
                                    &steps,
                                    &neighbor_ref,
                                    &elephant_position_ref,
                                    heap,
                                    time,
                                    total_value,
                                    value_per_minute,
                                    &ValueAtTime::Double(
                                        *value_per_minute_myself,
                                        *value_per_minute_elephant
                                            + elephant_position_ref.flow_rate(),
                                    ),
                                    &neighbor,
                                    &elephant_position,
                                    opened_valves.clone(),
                                );
                                most_value_per_minute_at_myself.insert(
                                    PositionAtValveKey::new_single_key(neighbor_ref.name()),
                                    *value_per_minute_myself,
                                );
                                should_update = true;
                            }
                        }
                    }
                }
                /*if should_update {
                    most_value_per_minute_at_elephant.insert(
                        PositionAtValveKey::new_single_key(elephant_position_ref.name()),
                        *value_per_minute_elephant + elephant_position_ref.flow_rate(),
                    );
                }*/
            }
        }
    }
}

fn open_valve_move_elephant(
    opened_valves: &Vec<String>,
    my_position_ref: &std::cell::Ref<Valve>,
    elephant_position_ref: &std::cell::Ref<Valve>,
    most_value_per_minute_at_myself: &mut HashMap<PositionAtValveKey, i32>,
    most_value_per_minute_at_elephant: &mut HashMap<PositionAtValveKey, i32>,
    value_per_minute: &ValueAtTime,
    steps: &Vec<String>,
    heap: &mut BinaryHeap<State>,
    time: i32,
    total_value: i32,
    my_position: &Rc<RefCell<Valve>>,
) {
    if !opened_valves.contains(&my_position_ref.name()) && my_position_ref.can_be_opened() {
        if let ValueAtTime::Double(value_per_minute_myself, value_per_minute_elephant) =
            value_per_minute
        {
            let mut should_update = false;
            if let Some(value_per_minute_at_myself) = most_value_per_minute_at_myself
                .get(&PositionAtValveKey::new_single_key(my_position_ref.name()))
            {
                let mut opened_valves = opened_valves.clone();
                opened_valves.push(my_position_ref.name());
                for elephant_neighbor in elephant_position_ref.neighbors().iter() {
                    if let Some(elephant_neighbor) = elephant_neighbor.upgrade() {
                        let elephant_neighbor_ref = elephant_neighbor.borrow();

                        if let Some(value_per_minute_at_elephant) =
                            most_value_per_minute_at_elephant.get(
                                &PositionAtValveKey::new_single_key(elephant_neighbor_ref.name()),
                            )
                        {
                            if *value_per_minute_at_myself
                                <= (*value_per_minute_myself + total_value)
                                && *value_per_minute_at_elephant <= (*value_per_minute_elephant)
                            {
                                add_step_to_heap(
                                    &steps,
                                    &my_position_ref,
                                    &elephant_neighbor_ref,
                                    heap,
                                    time,
                                    total_value,
                                    value_per_minute,
                                    &ValueAtTime::Double(
                                        *value_per_minute_myself + my_position_ref.flow_rate(),
                                        *value_per_minute_elephant,
                                    ),
                                    &my_position,
                                    &elephant_neighbor,
                                    opened_valves.clone(),
                                );

                                most_value_per_minute_at_elephant.insert(
                                    PositionAtValveKey::new_single_key(
                                        elephant_neighbor_ref.name(),
                                    ),
                                    *value_per_minute_elephant,
                                );
                                should_update = true;
                            }
                        }
                    }
                }
            }
            /*if should_update {
                most_value_per_minute_at_myself.insert(
                    PositionAtValveKey::new_single_key(my_position_ref.name()),
                    *value_per_minute_myself + my_position_ref.flow_rate(),
                );
            }*/
        }
    }
}

fn move_myself_and_elephant(
    my_position_ref: &std::cell::Ref<Valve>,
    elephant_position_ref: &std::cell::Ref<Valve>,
    most_value_per_minute_at_myself: &mut HashMap<PositionAtValveKey, i32>,
    most_value_per_minute_at_elephant: &mut HashMap<PositionAtValveKey, i32>,
    value_per_minute: &ValueAtTime,
    steps: &Vec<String>,
    heap: &mut BinaryHeap<State>,
    time: i32,
    total_value: i32,
    opened_valves: &Vec<String>,
) {
    if let ValueAtTime::Double(value_per_minute_myself, value_per_minute_elephant) =
        value_per_minute
    {
        for neighbor in my_position_ref.neighbors().iter() {
            if let Some(value_per_minute_at_myself) = most_value_per_minute_at_myself
                .get(&PositionAtValveKey::new_single_key(my_position_ref.name()))
            {
                let mut should_update = false;
                if let Some(neighbor) = neighbor.upgrade() {
                    let neighbor_ref = neighbor.borrow();
                    for elephant_neighbor in elephant_position_ref.neighbors().iter() {
                        if let Some(elephant_neighbor) = elephant_neighbor.upgrade() {
                            let elephant_neighbor_ref = elephant_neighbor.borrow();

                            if let Some(value_per_minute_at_elephant) =
                                most_value_per_minute_at_elephant.get(
                                    &PositionAtValveKey::new_single_key(
                                        elephant_position_ref.name(),
                                    ),
                                )
                            {
                                if (value_per_minute_at_myself <= value_per_minute_myself)
                                    && (value_per_minute_at_elephant <= value_per_minute_elephant)
                                {
                                    add_step_to_heap(
                                        &steps,
                                        &neighbor_ref,
                                        &elephant_neighbor_ref,
                                        heap,
                                        time,
                                        total_value,
                                        value_per_minute,
                                        value_per_minute,
                                        &neighbor,
                                        &elephant_neighbor,
                                        opened_valves.clone(),
                                    );
                                    most_value_per_minute_at_elephant.insert(
                                        PositionAtValveKey::new_single_key(
                                            elephant_neighbor_ref.name(),
                                        ),
                                        *value_per_minute_elephant,
                                    );
                                    should_update = true;
                                }
                            }
                        }
                    }
                    if should_update {
                        most_value_per_minute_at_myself.insert(
                            PositionAtValveKey::new_single_key(neighbor_ref.name()),
                            *value_per_minute_myself,
                        );
                    }
                }
            }
        }
    }
}

fn add_step_to_heap(
    steps: &Vec<String>,
    neighbor_ref: &std::cell::Ref<Valve>,
    elephant_neighbor_ref: &std::cell::Ref<Valve>,
    heap: &mut BinaryHeap<State>,
    time: i32,
    total_value: i32,
    old_value_per_minute: &ValueAtTime,
    value_per_minute: &ValueAtTime,
    neighbor: &Rc<RefCell<Valve>>,
    elephant_neighbor: &Rc<RefCell<Valve>>,
    opened_valves: Vec<String>,
) {
    let mut new_steps = steps.clone();
    new_steps.push(neighbor_ref.name());
    new_steps.push(elephant_neighbor_ref.name());
    if let ValueAtTime::Double(value_minute_myself, value_minute_elephant) = old_value_per_minute {
        heap.push(State {
            time: time + 1,
            total_value: total_value + *value_minute_myself + *value_minute_elephant,
            value_per_minute: value_per_minute.clone(),
            position: PositionAtTime::Double(Rc::clone(neighbor), Rc::clone(elephant_neighbor)),
            opened_valves: opened_valves,
            steps: new_steps,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    fn test_get_path_value() {
        const MAX_TIME: i32 = 26;
        const EXPECTED_SCORE: i32 = 1707;
        let valves = create_valves();
        let best_path_score = get_best_path_with_two(&valves[0], &valves, MAX_TIME);
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
/*
if (*value_per_minute_at_myself + *value_per_minute_at_elephant)
                            <= ((*value_per_minute_myself)
                                + (*value_per_minute_elephant + total_value))
*/
